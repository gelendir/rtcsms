use crate::json::Error;
use crate::json::{Token, TokenKind};
use std::char;
use std::str::Chars;

/// Internal state of the JSON lexer
enum State {
    Neutral,
    Text,
    Keyword,
    Number
}

/// JSON lexer, converts a JSON string to tokens
pub struct Lexer {
    state: State,
    buffer: String,
    tokens: Vec<Token>,
}

impl Lexer {

    pub fn new() -> Lexer {
        Lexer {
            state: State::Neutral,
            buffer: String::new(),
            tokens: Vec::new(),
        }
    }

    /// Convert a string to a series of tokens
    pub fn lex(&mut self, text: &str) -> Result<Vec<Token>, Error> {
        for (i, c) in text.chars().enumerate() {
            match self.state {
                State::Neutral => self.lex_neutral(i, c),
                State::Text => self.lex_text(i, c)?,
                State::Keyword => self.lex_keyword(i, c)?,
                State::Number => self.lex_number(i, c)?
            };
        }

        let pos = text.len();
        match self.state {
            State::Neutral => Ok(self.tokens.clone()),
            State::Text => {
                self.add_text(pos)?;
                Ok(self.tokens.clone())
            },
            State::Keyword => {
                self.add_keyword(pos)?;
                Ok(self.tokens.clone())
            },
            State::Number => {
                self.add_number(pos)?;
                Ok(self.tokens.clone())
            }
        }
    }

    /// Handle text when the lexer isn't in a specific state
    fn lex_neutral(&mut self, index: usize, character: char) {
        match character {
            '{' => self.tokens.push(Token{kind: TokenKind::ObjOpen, pos: index}),
            '}' => self.tokens.push(Token{kind: TokenKind::ObjClose, pos: index}),
            '[' => self.tokens.push(Token{kind: TokenKind::ArrayOpen, pos: index}),
            ']' => self.tokens.push(Token{kind: TokenKind::ArrayClose, pos: index}),
            ',' => self.tokens.push(Token{kind: TokenKind::Separator, pos: index}),
            ':' => self.tokens.push(Token{kind: TokenKind::Assign, pos: index}),
            '0'..= '9' | '-' => {
                self.state = State::Number;
                self.buffer.clear();
                self.buffer.push(character);
            }
            't' | 'f' | 'n' => {
                self.state = State::Keyword;
                self.buffer.clear();
                self.buffer.push(character);
            }
            '"' => {
                self.state = State::Text;
                self.buffer.clear();
                self.buffer.push(character);
            },
            _ => {}
        }
    }

    /// Handle text for a keyword such as 'true', 'false', 'null'
    fn lex_keyword(&mut self, index: usize, character: char) -> Result<(), Error> {
        match character {
            'r' | 'u' | 'a' | 'l' | 's' | 'e' => {
                self.buffer.push(character);
                Ok(())
            },
            _ => {
                self.add_keyword(index)?;
                self.state = State::Neutral;
                self.lex_neutral(index, character);
                Ok(())
            }
        }
    }

    /// Read the keyword in the buffer and convert it to a token
    fn add_keyword(&mut self, index: usize) -> Result<(), Error> {
        match &self.buffer[..] {
            "true" => {
                self.tokens.push(
                    Token{kind: TokenKind::Bool(true), pos: index - 4}
                );
                Ok(())
            },
            "false" => {
                self.tokens.push(
                    Token{kind: TokenKind::Bool(false), pos: index - 5}
                );
                Ok(())
            },
            "null" => {
                self.tokens.push(
                    Token{kind: TokenKind::Null, pos: index - 4}
                );
                Ok(())
            }
            _ => {
                Err(Error::invalid(index, "keyword", &self.buffer))
            }
        }
    }

    /// Handle text for a numeric value
    fn lex_number(&mut self, index: usize, character: char) -> Result<(), Error> {
        match character {
            '0'..= '9' => {
                self.buffer.push(character);
                Ok(())
            },
            '-' | '.' | 'e' => {
                self.buffer.push(character);
                Ok(())
            },
            _ => {
                self.add_number(index)?;
                self.state = State::Neutral;
                self.lex_neutral(index, character);
                Ok(())
            }
        }
    }

    /// Read the number in the buffer and convert to a token
    fn add_number(&mut self, index: usize) -> Result<(), Error> {
        let number: f64 = self.buffer.parse()
            .map_err(|_| Error::invalid(index, "digit", &self.buffer))
            ?;

        if self.buffer.contains('.') {
            self.tokens.push(Token{
                kind: TokenKind::Float(number), 
                pos: index - self.buffer.len()
            });
        } else {
            self.tokens.push(Token{
                kind: TokenKind::Int(number as i64), 
                pos: index - self.buffer.len()
            });
        }

        Ok(())
    }

    /// Handle text inside a string
    fn lex_text(&mut self, index: usize, character: char) -> Result<(), Error> {
        self.buffer.push(character);
        if character == '"' && !self.buffer.ends_with("\\\"") {
            self.add_text(index)?;
            self.state = State::Neutral;
        }
        Ok(())
    }

    /// Convert text in the buffer to a string
    fn add_text(&mut self, index: usize) -> Result<(), Error> {
        if !self.buffer.ends_with("\"") {
            return Err(Error::invalid(index, "string", &self.buffer));
        }

        self.tokens.push(Token{
            kind: TokenKind::Text(self.convert_string(index)?),
            pos: index - self.buffer.len()
        });
        Ok(())
    }

    /// Convert special characters such as \r \n \t etc
    fn convert_string(&self, index: usize) -> Result<String, Error> {
        let mut iter = self.buffer[1..self.buffer.len()-1].chars();
        let mut converted = String::new();
        loop {
            match &iter.next() {
                None => return Ok(converted),
                Some('\\') => {
                    match &iter.next() {
                        Some('n') => converted.push('\n'),
                        Some('t') => converted.push('\t'),
                        Some('r') => converted.push('\r'),
                        Some('u') => converted.push(Lexer::convert_unicode(index, &mut iter)?),
                        Some(c) => converted.push(*c),
                        None => return Ok(converted)
                    }
                },
                Some(c) => converted.push(*c)
            }
        }
    }

    /// Convert unicode code points such as \u0123
    fn convert_unicode(index: usize, iter: &mut Chars) -> Result<char, Error> {
        let digit: String = iter.take(3).collect();

        let code: u32 = digit.parse()
            .map_err(|_| Error::invalid(index, "string", &digit))?;

        char::from_u32(code)
            .ok_or(Error::invalid(index, "string", &digit))
    }
}
