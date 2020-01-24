use std::io::BufReader;
use std::io::BufRead;
use std::io::{Read, Write};

use crate::http;
use crate::http::Error;
use crate::http::Verb;
use crate::http::HeaderSet;
use crate::http::URL;

/// Representation for a basic HTTP request
#[derive(Debug)]
pub struct Request {
    pub verb: Verb,
    pub url: URL,
    pub headers: HeaderSet,
    pub body: Vec<u8>
}

impl Request {

    pub fn new(verb: Verb, url: URL) -> Request {
        Request {
            verb: verb,
            url: url,
            headers: HeaderSet::new(),
            body: Vec::new(),
        }
    }

    /// Read a HTTP request from a TCP socket
    pub fn read<T: Read>(stream: &mut T) -> Result<Request, Error> {
        let mut reader = BufReader::new(stream);
        let mut request = Request::read_stanza(&mut reader)?;
        request.headers = HeaderSet::read(&mut reader)?;
        request.body = http::read_body(&request.headers, &mut reader)?;

        Ok(request)
    }

    /// Write a HTTP request to a TCP socket
    pub fn write<T: Write>(&self, stream: &mut T) -> Result<(), Error> {
        let stanza = format!(
            "{} {} HTTP/1.1\r\n", 
            self.verb.format(), 
            self.url.to_query(),
        );
        stream.write(stanza.as_bytes())?;

        self.headers.write(stream)?;

        if !self.body.is_empty() {
            http::write_body(&self.body, stream)?;
        } else {
            stream.write(b"\r\n")?;
        }

        Ok(())
    }

    /// Read and convert the first line of a HTTP request
    fn read_stanza<T: Read>(reader: &mut BufReader<&mut T>) -> Result<Request, Error> {
        let mut stanza = String::new();
        reader.read_line(&mut stanza)?;

        let mut stanza = stanza.trim().split_whitespace();

        let verb = stanza.next().ok_or(Error::Verb)?;
        let query = stanza.next().ok_or(Error::Query)?;
        let version = stanza.next().ok_or(Error::Version)?;

        if version != "HTTP/1.1" {
            return Err(Error::Version);
        }

        let verb = Verb::parse(verb)?;
        let url = URL::from_request(query);

        Ok(Request::new(verb, url))
    }

    /// Convert the body to a Unicode String
    pub fn body_string(&self) -> String {
        String::from_utf8_lossy(&self.body).to_string()
    }
}
