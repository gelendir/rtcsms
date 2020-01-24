use std::io::{Read, Write};
use std::io::BufReader;
use std::io::BufRead;

use crate::http::Error;

/// A Header in an HTTP request or response
#[derive(Debug)]
pub struct Header {
    pub key: String,
    pub value: String
}

/// A set of Headers in an HTTP request or response
#[derive(Debug)]
pub struct HeaderSet {
    headers: Vec<Header>
}

impl HeaderSet {

    pub fn new() -> HeaderSet {
        HeaderSet{ headers: Vec::new() }
    }

    /// Add a header to the set.
    ///
    /// Duplicate header keys are not managed. If Headers with the same name
    /// are added, they will be sent in the order they were added
    pub fn add(&mut self, name: &str, value: &str) {
        self.headers.push(Header{
            key: name.to_string(),
            value: value.to_string()
        })
    }

    /// Add a header to the set if it doesn't already exist
    pub fn add_default(&mut self, name: &str, value: &str) { 
        if let None = self.get(name) {
            self.add(name, value);
        }
    }

    /// Get the value of the first header with the corresponding name from the set
    pub fn get(&self, name: &str) -> Option<String> {
        self.headers.iter()
            .filter(|h| h.key == name)
            .map(|h| h.value.to_string())
            .next()
    }

    /// Read all headers from a TCP socket and convert them to a HeaderSet
    pub fn read<T: Read>(reader: &mut BufReader<&mut T>) -> Result<HeaderSet, Error> {
        let mut headers: Vec<Header> = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if line == "" {
                return Ok(HeaderSet{headers})
            }

            let mut parts = line.splitn(2, ": ");
            let key = parts.next().ok_or(Error::header("Missing header name"))?;
            let value = parts.next().ok_or(Error::header("Missing header value"))?;
            headers.push(
                Header{ 
                    key: key.to_string(),
                    value: value.to_string()
                }
            );
        }

        return Ok(HeaderSet{headers})
    }

    /// Write all headers in the set to a TCP socket
    pub fn write<T: Write>(&self, stream: &mut T) -> Result<(), Error> {
        for header in self.headers.iter() {
            let line = format!("{}: {}\r\n", header.key, header.value);
            stream.write(line.as_bytes())?;
        }

        Ok(())
    }
}
