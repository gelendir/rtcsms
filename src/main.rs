mod http;
mod json;

use std::net::{TcpListener, TcpStream};
use std::collections::HashMap;
use chrono::prelude::*;

use http::{Request, Verb, Response, Error, URL, client};
use json::parse;
use json::JsonType;

struct RtcSms {
    username: String,
    password: String,
    did: String 
}

fn main() {
    let host = std::env::args().nth(1).expect("no host");
    let username = std::env::args().nth(2).expect("no voip.ms username");
    let password = std::env::args().nth(3).expect("no voip.ms password");
    let did = std::env::args().nth(4).expect("no voip.ms did");

    let rtcsms = RtcSms {
        username: username,
        password: password,
        did: did
    };

    let listener = TcpListener::bind(host).expect("cannot bind to host");

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                if let Err(e) = rtcsms.handle_client(s) {
                    println!("ERROR: {}", e);
                }
            },
            Err(e) => {
                println!("ERROR: {}", e);
            }
        }
    }
}

/// Structure that manages reading HTTP requests, fetching information from RTC
/// and sending an SMS with the next bus to pass
impl RtcSms {

    /// Handle new TCP socket from an HTTP client
    pub fn handle_client(&self, mut stream: TcpStream) -> Result<(), Error> {
        let request = Request::read(&mut stream)?;
        println!("request: {} {} '{}'", 
            request.verb, 
            request.url.to_query(), 
            request.body_string()
        );

        let response = match self.handle_request(request) {
            Ok(r) => r,
            Err(e) => {
                let body = format!("{}\n", e);
                Response::new(400, body.as_bytes())
            }
        };

        println!("response: {} '{}'", 
            response.code, 
            response.body_string()
        );
        response.write(&mut stream)?;

        Ok(())
    }

    /// Find for what bus we want a schedule for and send an SMS back
    fn handle_request(&self, request: Request) -> Result<Response, String> {
        if let Verb::Get = request.verb {
            let message = request.url.get_arg("message").ok_or("no message")?;
            let dst = request.url.get_arg("dst").ok_or("no dst")?;

            let mut parts = message.trim().splitn(2, "+");

            let stop = parts
                .next()
                .ok_or(String::from("no stop number"))?
                .parse::<i64>()
                .map_err(|_| String::from("invaid stop number"))?;

            let bus = parts
                .next()
                .ok_or(String::from("no bus number"))?
                .parse::<i64>()
                .map_err(|_| String::from("invaid bus number"))?;

            let minutes = self.get_next_bus(stop, bus)?;

            self.send_sms(dst, stop, bus, minutes)?;

            let message = format!("Bus: {}, stop: {}, minutes: {}\n", bus, stop, minutes);
            let response = Response::new(200, &message.as_bytes());
            Ok(response)
        } else {
            Err(String::from("Unhandled HTTP verb"))
        }
    }

    /// Get the minutes left before next bus passes at a stop from the RTC API
    fn get_next_bus(&self, stop: i64, bus: i64) -> Result<i64, String> {
            let mut response = self.rtc_json(stop, bus, 2)?;

            if let Some(JsonType::Null) = response.get("horaires") {
                // Since the RTC web API doesn't have any public documentation, 
                // I haven't figured out how to find directions available at a 
                // stop. I just try both directions hoping one of them actually 
                // returns back some data
                response = self.rtc_json(stop, bus, 3)?;
            }

            let schedule = match response.get("horaires") {
                Some(JsonType::Array(a)) => a,
                _ => return Err(String::from("missing horaires"))
            };

            let next = match schedule.get(0) {
                Some(JsonType::Object(o)) => o,
                _ => return Err(String::from("missing first horaire"))
            };

            match next.get("departMinutes") {
                Some(JsonType::Int(m)) => Ok(*m),
                _ => Err(String::from("missing departMinutes"))
            }
    }

    /// Send a HTTP request to the RTC API and convert the response to JSON
    fn rtc_json(&self, stop: i64, bus: i64, direction: i64) -> Result<HashMap<String, JsonType>, String> {
        let date = Local::now().format("%Y%m%d");

        let mut url = URL::parse("https://wssiteweb.rtcquebec.ca/api/v2/horaire/BorneVirtuelle_ArretParcours/")?;
        url.add_arg("noParcours", &bus.to_string());
        url.add_arg("noArret", &stop.to_string());
        url.add_arg("codeDirection", &direction.to_string());
        url.add_arg("date", &date.to_string());

        let request = Request::new(Verb::Get, url);
        let response = client::send(request)?;
        let body = response.body_string();

        match parse(&body)? {
            JsonType::Object(o) => Ok(o),
            _ => Err(format!("Invalid RTC response: {}", body))
        }
    }

    /// Send a SMS back to the user using the voip.ms API
    fn send_sms(&self, dst: &str, stop: i64, bus: i64, minutes: i64) -> Result<(), String> {
        let message = format!("{}+-+{}:+{}+minutes", stop, bus, minutes);

        let mut url = URL::parse("https://voip.ms/api/v1/rest.php")?;
        url.add_arg("api_username", &self.username);
        url.add_arg("api_password", &self.password);
        url.add_arg("method", "sendSMS");
        url.add_arg("did", &self.did);
        url.add_arg("dst", dst);
        url.add_arg("message", &message);

        let request = Request::new(Verb::Get, url);
        let response = client::send(request)?;

        println!("voip.ms response: {} '{}'", response.code, response.body_string());

        Ok(())

    }
}
