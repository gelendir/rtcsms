use crate::http::Error;

use std::fmt;

/// Representation for a HTTP verb, i.e. GET, POST, etc.
#[derive(Debug)]
pub enum Verb {
    Connect,
    Delete,
    Get,
    Head,
    Options,
    Patch,
    Post,
    Put,
    Trace
}

impl Verb {

    /// Convert a string to a HTTP verb
    pub fn parse(text: &str) -> Result<Verb, Error> {
        match text {
            "CONNECT" => Ok(Verb::Connect),
            "DELETE" => Ok(Verb::Delete),
            "GET" => Ok(Verb::Get),
            "HEAD" => Ok(Verb::Head),
            "OPTIONS" => Ok(Verb::Options),
            "PATCH" => Ok(Verb::Patch),
            "POST" => Ok(Verb::Post),
            "PUT" => Ok(Verb::Put),
            "TRACE" => Ok(Verb::Trace),
            _ => Err(Error::Verb)
        }
    }

    /// Convert a HTTP verb to its string representation
    pub fn format(&self) -> String {
        match self {
            Verb::Connect => "CONNECT",
            Verb::Delete => "DELETE",
            Verb::Get =>"GET",
            Verb::Head =>"HEAD",
            Verb::Options =>"OPTIONS",
            Verb::Patch =>"PATCH",
            Verb::Post =>"POST",
            Verb::Put =>"PUT",
            Verb::Trace =>"TRACE",
        }.to_string()
    }
}

impl fmt::Display for Verb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}
