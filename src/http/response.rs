use std::io::BufReader;
use std::io::BufRead;
use std::io::{Read, Write};

use crate::http;
use crate::http::{HeaderSet, Error};

/// Representation for a basic HTTP response
#[derive(Debug)]
pub struct Response {
    pub code: u32,
    pub headers: HeaderSet,
    pub body: Vec<u8>
}

impl Response {

    pub fn new(code: u32, content: &[u8]) -> Response {
        Response {
            code: code,
            headers: HeaderSet::new(),
            body: content.to_vec(),
        }
    }

    /// Read a HTTP response from a TCP socket
    pub fn read<T: Read>(stream: &mut T) -> Result<Response, Error> {
        let mut reader = BufReader::new(stream);
        let mut response = Response::read_stanza(&mut reader)?;
        response.headers = HeaderSet::read(&mut reader)?;
        response.body = http::read_body(&response.headers, &mut reader)?;
        Ok(response)
    }

    /// Read and convert the first line of a HTTP response
    fn read_stanza<T: Read>(reader: &mut BufReader<&mut T>) -> Result<Response, Error> {
        let mut stanza = String::new();
        reader.read_line(&mut stanza)?;

        if !stanza.trim().starts_with("HTTP/1.1") {
            return Err(Error::Version);
        }

        let mut stanza = stanza.split_whitespace();
        let code = stanza
            .nth(1)
            .ok_or(Error::Status)?
            .parse::<u32>()
            .map_err(|_| Error::Status)?;

        Ok(Response::new(code, &[]))
    }

    /// Write the response to a TCP socket
    pub fn write<T: Write>(&self, stream: &mut T) -> Result<(), Error> {
        let stanza = format!("HTTP/1.1 {}\r\n", self.code);
        stream.write(stanza.as_bytes())?;

        self.headers.write(stream)?;
        http::write_body(&self.body, stream)
    }

    pub fn body_string(&self) -> String {
        String::from_utf8_lossy(&self.body).to_string()
    }

}
