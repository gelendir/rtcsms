extern crate native_tls;

use std::io::{Read, Write};
use std::net::TcpStream;

/// This trait exists so that the HTTP lib can use encrypted or unencrypted
/// TCP sockets interchangeably
pub trait ReadWrite: Read + Write {}

impl ReadWrite for TcpStream {}
impl ReadWrite for native_tls::TlsStream<TcpStream> {}
