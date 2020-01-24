use crate::http::Error;

#[derive(Debug, Copy, Clone)]
pub enum Protocol {
    Http = 80,
    Https = 443
}

/// Struct to manage URLs both for requests and responses.
/// The complete URI spec isn't managed. For now what is supported is:
/// - HTTP and HTTPS
/// - host
/// - port
/// - path
/// - query arguments (but they will not be url encoded/decoded)
#[derive(Debug)]
pub struct URL {
    pub host: String,
    pub port: u32,
    pub path: String,
    pub args: Vec<(String, String)>,
    pub protocol: Protocol,
}

impl URL {

    /// Convert the query in a HTTP request to a URL
    pub fn from_request(text: &str) -> URL {
        let (path, args) = URL::parse_query(text);
        // Ideally the host and port should reflect what the HTTP server is currently
        // listening on, but I don't need this for now so setting to localhost is 
        // sufficient
        URL{
            host: String::from("127.0.0.1"),
            port: 80,
            path: path,
            args: args,
            protocol: Protocol::Http
        }
    }

    /// Convert a textual URL, e.g. 'http://example.com' to its equivalent URL struct
    pub fn parse(url: &str) -> Result<URL, Error> {
        let mut url = url.splitn(2, "://");

        let protocol = match &url.next() {
            Some("http") => Ok(Protocol::Http),
            Some("https") => Ok(Protocol::Https),
            _ => Err(Error::Protocol)
        }?;

        let mut url = url
            .next()
            .ok_or(Error::url("No host"))?
            .splitn(2, "/");

        let mut location = url
            .next()
            .ok_or(Error::url("No host"))?
            .splitn(2, ":");

        let host = location
            .next()
            .ok_or(Error::url("No host"))?;

        let port: u32 = match location.next() {
            Some(p) => p.parse().map_err(|_| Error::url("Invalid port"))?,
            None => protocol as u32
        };

        let query = format!("/{}", url.next().unwrap_or(""));
        let (path, args) = URL::parse_query(&query);

        Ok(URL{
            host: host.to_string(),
            port: port,
            path: path,
            args: args,
            protocol: protocol,
        })
    }

    /// Parse the query part of a URL to its path and query arguments
    fn parse_query(text: &str) -> (String, Vec<(String, String)>) {
        let mut parts = text.splitn(2, "?");
        let path = parts.next().unwrap_or("/").to_string();

        let mut args: Vec<(String, String)> = Vec::new();
        if let Some(a) = parts.next() {
            for arg in a.split("&") {
                let mut arg = arg.splitn(2, "=");
                let name = arg.next().unwrap_or("").to_string();
                let value = arg.next().unwrap_or("").to_string();
                if name != "" {
                    args.push((name, value))
                }
            }
        }

        (path, args)
    }

    /// Return a connection string used to connect to the host with a TCP socket
    pub fn connection(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    /// Add a query argument to the URL
    pub fn add_arg(&mut self, name: &str, value: &str) {
        self.args.push((name.to_string(), value.to_string()));
    }

    /// Convert the 'query' (i.e. path and arguments) to a form usable in a HTTP request
    pub fn to_query(&self) -> String {
        let mut query = self.path.clone();

        let mut args = self.args.iter();
        match args.next() {
            None => return query,
            Some(arg) => {
                query.push('?');
                URL::push_arg(&mut query, arg);
            }
        }

        for arg in args {
            query.push('&');
            URL::push_arg(&mut query, arg);
        }

        query
    }

    /// Convert a query argument to its url representation and add it at the end of the string
    fn push_arg(result: &mut String, arg: &(String, String)) {
        result.push_str(&arg.0);
        if arg.1 != "" {
            result.push('=');
            result.push_str(&arg.1);
        }
    }

    /// Get the first query argument available. Will only return the first
    /// if multiple arguments with the same name are present.
    pub fn get_arg(&self, key: &str) -> Option<&str> {
        for (name, value) in self.args.iter() {
            if name == key {
                return Some(value);
            }
        }
        None
    }

}
