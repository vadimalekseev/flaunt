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
    Value,
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
    BeforeIdentifierStart,
    IdentifierStart,
    BeforeAssignStart,
    AssignStart,
    BeforeValueStart,
    ValueStart,
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
            state: State::BeforeIdentifierStart,
        }
    }

    pub fn parse_line(&mut self, line: String) -> Result<Vec<Token>> {
        let mut result = Vec::with_capacity(5);

        if !line.starts_with(BEGIN) {
            return Ok(result);
        }

        for (i, c) in line.chars().enumerate().skip(BEGIN.len()) {
            match self.state {
                State::BeforeIdentifierStart | State::BeforeAssignStart | State::BeforeValueStart => {
                    if c == ' ' {
                        continue;
                    }
                    self.start = i;

                    match self.state {
                        State::BeforeIdentifierStart => self.state = State::IdentifierStart,
                        State::BeforeAssignStart => self.state = State::AssignStart,
                        State::BeforeValueStart => self.state = State::ValueStart,
                        _ => unreachable!()
                    }
                }

                State::IdentifierStart => {
                    match line.get(i+1..i+2) {
                        Option::Some(s) => {
                            if s != ":" {
                                continue;
                            }

                            result.push(Token {
                                operation: Operation::Identifier,
                                value: line.get(self.start..i).unwrap().to_string(),
                            });

                            self.state = State::BeforeAssignStart;
                        }
                        Option::None => {}
                    }

                }

                State::AssignStart => {
                    result.push(Token {
                        operation: Operation::Assign,
                        value: line.get(self.start..self.start+1).unwrap().to_string(),
                    });

                    self.state = State::BeforeValueStart;
                }

                State::ValueStart => {
                    if c != ',' && i != line.len()-1 {
                        continue;
                    }
                    result.push(Token {
                        operation: Operation::Value,
                        value: line.get(self.start..i+1).unwrap().to_string(),
                    });
                    self.state = State::BeforeIdentifierStart;
                    self.start = i
                }
            }
        }
        match self.state {
            State::AssignStart => result.push(Token {
                operation: Operation::Value,
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

        let res = l.parse_line("//language:rust".to_string());
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
                operation: Operation::Value,
                value: "rust".to_string(),
            },
        ];

        println!(
            "\ngot = {:?}\nexpected = {:?}\n",
            got,
            expected,
        );

        assert!(got.len() == expected.len());
        assert_eq!(got.pop(), expected.pop());
        assert_eq!(got.pop(), expected.pop());
        assert_eq!(got.pop(), expected.pop());
    }

    #[test]
    fn it_works_with_spaces() {
        let mut l = Lexer::new();

        let res = l.parse_line("// language : rust".to_string());
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
                operation: Operation::Value,
                value: "rust".to_string(),
            },
        ];

        println!(
            "\ngot = {:?}\nexpected = {:?}\n",
            got,
            expected,
        );

        assert!(got.len() == expected.len());
        assert_eq!(got.pop(), expected.pop());
        assert_eq!(got.pop(), expected.pop());
        assert_eq!(got.pop(), expected.pop());
    }
}
