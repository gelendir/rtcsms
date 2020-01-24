use crate::http::HeaderSet;
use crate::http::Error;
use std::io::{Read, Write};
use std::io::BufReader;
use std::io::BufRead;

/// Read the HTTP body from a TCP stream. Can be used for a request or response
pub fn read_body<T: Read>(headers: &HeaderSet, reader: &mut BufReader<&mut T>) -> Result<Vec<u8>, Error> {
    if let Some(header) = headers.get("Content-Length") {
        return read_length(header, reader);
    } else if let Some(header) = headers.get("Transfer-Encoding") {
        return read_transfer(header, reader);
    }

    Ok(Vec::new())
}

/// Read a body using the Content-Length header
pub fn read_length<T: Read>(length: String, reader: &mut BufReader<&mut T>) -> Result<Vec<u8>, Error> {
    let length = length
        .parse::<usize>()
        .map_err(|_| Error::header("Invalid Content-Length"))?;

    let mut body: Vec<u8> = Vec::with_capacity(length);
    reader.take(length as u64).read_to_end(&mut body)?;
    Ok(body)
}

/// Read a body using the Transfer-Encoding: chunked method
pub fn read_transfer<T: Read>(transfer: String, reader: &mut BufReader<&mut T>) -> Result<Vec<u8>, Error> {
    if transfer != "chunked" {
        return Err(Error::header("Unsupported Transfer-Encoding"));
    }

    let mut body: Vec<u8> = Vec::new();
    let mut line = String::new();

    loop {
        line.clear();
        reader.read_line(&mut line)?;
        let line = line.trim();

        if line == "" {
            return Ok(body)
        }
        
        let length = usize::from_str_radix(line, 16)
            .map_err(|_| Error::header("Invalid Transfer-Encoding"))?;

        reader.take(length as u64).read_to_end(&mut body)?;
    }
}

/// Write a body to a TCP stream for a HTTP request or response
pub fn write_body<T: Write>(body: &[u8], stream: &mut T) -> Result<(), Error> {
    let line = format!("Content-Length: {}\r\n\r\n", body.len());
    stream.write(line.as_bytes())?;
    stream.write(body)?;

    Ok(())
}
