use std::cmp::PartialEq;
use std::fmt;
use std::vec::Vec;

pub type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, Clone)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "can't parse line")
    }
}

#[derive(PartialEq, Debug)]
pub enum Operation {
    Assign,
    Identifier,
    String,
}

#[derive(Debug)]
pub struct Token {
    operation: Operation,
    value: String,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.operation == other.operation && self.value == other.value
    }
}

enum State {
    Begin,
    AssignStart,
    IdentifierStart,
}
pub struct Lexer {
    state: State,
    start: usize,
}

const BEGIN: &str = "//";

impl Lexer {
    pub fn new() -> Self {
        Self {
            start: 0,
            state: State::Begin,
        }
    }

    pub fn parse_line(&mut self, line: String) -> Result<Vec<Token>> {
        let mut result = Vec::with_capacity(5);

        if !line.starts_with(BEGIN) {
            return Ok(result);
        }

        for (i, c) in line.chars().enumerate().skip(BEGIN.len()) {
            match self.state {
                State::Begin => {
                    self.state = State::IdentifierStart;
                    self.start = i;
                }

                State::IdentifierStart => {
                    if c != ':' {
                        continue;
                    }

                    result.push(Token {
                        operation: Operation::Identifier,
                        value: line.get(self.start..i).unwrap().to_string(),
                    });

                    result.push(Token {
                        operation: Operation::Assign,
                        value: c.to_string(),
                    });

                    self.state = State::AssignStart;
                    self.start = i;
                }

                State::AssignStart => {
                    if c == ' ' || c != ',' {
                        continue;
                    }

                    result.push(Token {
                        operation: Operation::String,
                        value: line.get(self.start..i).unwrap().to_string(),
                    })
                }
            }
        }
        match self.state {
            State::AssignStart => result.push(Token {
                operation: Operation::String,
                value: line.get(self.start..).unwrap().to_string(),
            }),
            _ => {}
        }

        return Ok(result);
    }
}

#[cfg(test)]
mod tests {
    use crate::lang::lang::{Operation, Token};

    use super::Lexer;

    #[test]
    fn it_works() {
        let mut l = Lexer::new();

        let res = l.parse_line("// language: rust".to_string());
        assert!(res.is_ok());

        let mut got = res.unwrap();
        let mut expected = vec![
            Token {
                operation: Operation::Identifier,
                value: "language".to_string(),
            },
            Token {
                operation: Operation::Assign,
                value: ":".to_string(),
            },
            Token {
                operation: Operation::String,
                value: "rust".to_string(),
            },
        ];

        assert!(
            got.len() == expected.len(),
            "got = {:?}, expected = {:?}",
            got,
            expected
        );

        assert_eq!(got.pop(), expected.pop())
    }
}
