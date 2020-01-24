use std::fmt;

/// JSON token structure
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: usize
}

/// Type of JSON token
#[derive(Debug)]
pub enum TokenKind {
    ObjOpen,
    ObjClose,
    ArrayOpen,
    ArrayClose,
    Assign,
    Separator,
    Null,
    Text(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self {
            TokenKind::ObjOpen => "{".to_string(),
            TokenKind::ObjClose => "}".to_string(),
            TokenKind::ArrayOpen => "[".to_string(),
            TokenKind::ArrayClose => "]".to_string(),
            TokenKind::Assign => ":".to_string(),
            TokenKind::Separator => ",".to_string(),
            TokenKind::Null => "null".to_string(),
            TokenKind::Text(t) => format!("\"{}\"", t),
            TokenKind::Int(i) => format!("{}", i),
            TokenKind::Float(f) => format!("{}", f),
            TokenKind::Bool(b) => format!("{}", b)
        };
        write!(f, "{}", text)
    }
}

impl Clone for TokenKind {
    fn clone(&self) -> TokenKind {
        match self {
            TokenKind::ObjOpen => TokenKind::ObjOpen,
            TokenKind::ObjClose => TokenKind::ObjClose,
            TokenKind::ArrayOpen => TokenKind::ArrayOpen,
            TokenKind::ArrayClose => TokenKind::ArrayClose,
            TokenKind::Assign => TokenKind::Assign,
            TokenKind::Separator => TokenKind::Separator,
            TokenKind::Null => TokenKind::Null,
            TokenKind::Text(t) => TokenKind::Text(t.clone()),
            TokenKind::Int(i) => TokenKind::Int(*i),
            TokenKind::Float(f) => TokenKind::Float(*f),
            TokenKind::Bool(b) => TokenKind::Bool(*b)
        }
    }
}
