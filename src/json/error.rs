use std::fmt;
use crate::json::{Token};

/// Error handler for all errors in the json module
#[derive(Debug)]
pub struct Error {
    pub pos: usize,
    pub message: String
}

impl Error {

    /// A JSON token that has a syntax error
    pub fn invalid(pos: usize, kind: &str, value: &str) -> Error {
        Error {
            pos: pos,
            message: format!("Invalid type {}: '{}'", kind, value)
        }
    }

    /// A JSON token that wasn't used in the right place
    pub fn unexpected(kind: &str, token: &Token) -> Error {
        Error {
            pos: token.pos,
            message: format!("Expecting type {}, got '{}'", kind, token.kind)
        }
    }

    /// A JSON token that's missing from the object
    pub fn missing(kind: &str) -> Error {
        Error {
            pos: 0,
            message: format!("Expecting {}, but no more tokens", kind)
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error at {}: {}", self.pos, self.message)
    }
}

impl From<Error> for String {
   fn from(e: Error) -> Self {
       format!("{}", e)
   }
}
