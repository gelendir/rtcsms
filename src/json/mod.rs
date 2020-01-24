mod lexer;
mod parser;
mod error;
mod token;

pub use parser::parse;
pub use parser::JsonType;
pub use error::Error;
pub use token::{TokenKind, Token};
