extern crate native_tls;

use std::net::TcpStream;

use crate::http::{Request, Response, Error, URL, Protocol};
use crate::http::io::ReadWrite;
use native_tls::TlsConnector;


/// Send an HTTP request and read the HTTP response
pub fn send(mut request: Request) -> Result<Response, Error> {
    request.headers.add_default("Host", &request.url.host);
    request.headers.add_default("User-Agent", "rtcsms");
    request.headers.add_default("Accept", "*/*");

    let mut stream = connect(&request.url)?;

    request.write(&mut stream)?;
    Response::read(&mut stream)
}

/// Connect to a server using a TCP or TLS-over-TCP socket
pub fn connect(url: &URL) -> Result<Box<dyn ReadWrite>, Error> {
    match url.protocol {
        Protocol::Http => {
            let stream = TcpStream::connect(url.connection())?;
            return Ok(Box::new(stream));
        }
        Protocol::Https => {
            let connector = TlsConnector::new()?;
            let stream = TcpStream::connect(url.connection())?;
            let stream = connector.connect(&url.host, stream)?;
            return Ok(Box::new(stream));
        }
    };
}
