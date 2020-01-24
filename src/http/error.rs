extern crate native_tls;

use std::fmt;
use std::io;
use std::net::TcpStream;

/// Error struct for managing all errors in the HTTP module
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Tls(native_tls::Error),
    Handshake(native_tls::HandshakeError<TcpStream>),
    Verb,
    Query,
    Version,
    Status,
    Protocol,
    Header(String),
    URL(String),
}

impl Error {

    pub fn header(message: &str) -> Error {
        Error::Header(message.to_string())
    }

    pub fn url(message: &str) -> Error {
        Error::URL(message.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            Error::Io(e) => format!("IO Error: {}", e),
            Error::Tls(e) => format!("TLS Error: {}", e),
            Error::Handshake(e) => format!("TLS Handshake Error: {}", e),
            Error::Verb => format!("HTTP error: invalid request method"),
            Error::Query => format!("HTTP error: invalid request URI"),
            Error::Version => format!("HTTP error: only HTTP/1.1 is supported"),
            Error::Status => format!("HTTP error: invalid status code"),
            Error::Protocol => format!("HTTP error: unsupported protocol"),
            Error::Header(e) => format!("HTTP error: invalid header: {}", e),
            Error::URL(e) => format!("HTTP error: invalid URL: {}", e)
        };
        write!(f, "{}", message)
    }
}

impl From<io::Error> for Error {
   fn from(e: io::Error) -> Self {
       Error::Io(e)
   }
}

impl From<native_tls::Error> for Error {
   fn from(e: native_tls::Error) -> Self {
       Error::Tls(e)
   }
}

impl From<native_tls::HandshakeError<TcpStream>> for Error {
   fn from(e: native_tls::HandshakeError<TcpStream>) -> Self {
       Error::Handshake(e)
   }
}

impl From<Error> for String {
   fn from(e: Error) -> Self {
       format!("{}", e)
   }
}
