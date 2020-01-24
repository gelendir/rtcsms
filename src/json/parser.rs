use crate::json::lexer::Lexer;
use crate::json::{Token, TokenKind};
use crate::json::Error;

use std::collections::HashMap;
use std::slice::Iter;

/// Type for representing our converted JSON structure
#[derive(Debug)]
pub enum JsonType {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Object(HashMap<String, JsonType>),
    Array(Vec<JsonType>),
}

/// Convert a string to its JSON representation
pub fn parse(text: &str) -> Result<JsonType, Error> {
    let mut lexer = Lexer::new();
    let tokens = lexer.lex(text)?;
    let mut iterator = tokens.iter();
    parse_tokens(&mut iterator)
}

/// Read JSON tokens and convert them to a JSON data type. Can be called recursively
fn parse_tokens(tokens: &mut Iter<Token>) -> Result<JsonType, Error> {
    let item = tokens.next().ok_or(Error::missing("token"))?;
    match &item.kind {
        TokenKind::Null => Ok(JsonType::Null),
        TokenKind::Bool(b) => Ok(JsonType::Bool(*b)),
        TokenKind::Int(i) => Ok(JsonType::Int(*i)),
        TokenKind::Float(f) => Ok(JsonType::Float(*f)),
        TokenKind::Text(t) => Ok(JsonType::String(t.clone())),
        TokenKind::ArrayOpen => parse_array(tokens),
        TokenKind::ObjOpen => parse_object(tokens),
        _ => Err(Error::unexpected("value", item))
    }
}

/// Read and convert tokens forming an array
fn parse_array(tokens: &mut Iter<Token>) -> Result<JsonType, Error> {
    let mut items: Vec<JsonType> = Vec::new();
    loop {
        items.push(parse_tokens(tokens)?);

        let item = tokens.next().ok_or(Error::missing("array"))?;
        match item.kind {
            TokenKind::Separator => {},
            TokenKind::ArrayClose => return Ok(JsonType::Array(items)),
            _ => return Err(Error::unexpected("separator", item))
        }
    }
}

/// Read and convert tokens forming an object
fn parse_object(tokens: &mut Iter<Token>) -> Result<JsonType, Error> {
    let mut items: HashMap<String, JsonType> = HashMap::new();

    loop {
        //the key as in {"key": "value"}
        let key = match tokens.next() {
            Some(token) => {
                match &token.kind {
                    TokenKind::Text(t) => t.to_string(),
                    _ => return Err(Error::unexpected("string", token))
                }
            }
            None => return Err(Error::missing("assignment"))
        };

        //make sure there is a ":" after the key
        match tokens.next() {
            Some(token) => {
                match token.kind {
                    TokenKind::Assign => {},
                    _ => return Err(Error::unexpected("assignment", token)),
                }
            },
            _ => return Err(Error::missing("assignment"))
        };

        //convert the value
        let value = parse_tokens(tokens)?;

        // handle a "," or "}"
        match tokens.next() {
            Some(token) => {
                match token.kind {
                    TokenKind::Separator => items.insert(key, value),
                    TokenKind::ObjClose => {
                        items.insert(key, value);
                        return Ok(JsonType::Object(items))
                    },
                    _ => return Err(Error::unexpected("object close or separator", token))
                }
            }
            None => return Err(Error::missing("object close or separator"))
        };
    }
}
