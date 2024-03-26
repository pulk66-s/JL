use either::Either::{self, Left, Right};

use crate::cst::{atom::Atom, parser::{Parser, ParserDataType}};

#[derive(Clone)]
pub struct Tokens {
    pub tokens: ParserDataType,
    pub index: usize,
}

#[derive(Debug)]
pub enum TokenType {
    String(String),
    Number(i64),
    Char(char),
}

impl Tokens {
    pub fn new(tokens: ParserDataType) -> Tokens {
        Tokens { tokens, index: 0 }
    }

    pub fn next(&mut self) -> Option<Either<ParserDataType, TokenType>> {
        match &self.tokens {
            ParserDataType::And(ref a) => {
                if self.index < a.values.len() {
                    let token = a.values[self.index].clone();
                    self.index += 1;
                    Some(Left(*token))
                } else {
                    None
                }
            }
            ParserDataType::Or(ref o) => {
                if self.index == 0 {
                    let token = o.left.clone();
                    self.index += 1;
                    Some(Left(*token))
                } else if self.index == 1 {
                    let token = o.right.clone();
                    self.index += 1;
                    Some(Left(*token))
                } else {
                    None
                }
            }
            ParserDataType::Atom(ref atom) => match atom {
                Atom::String(s) => Some(Right(TokenType::String(s.value.clone()))),
                Atom::Char(c) => Some(Right(TokenType::Char(c.value))),
                Atom::Num(n) => match n.value {
                    Some(v) => Some(Right(TokenType::Number(v))),
                    None => None,
                },
            },
            ParserDataType::Repeat(r) => {
                if self.index < r.values.len() {
                    println!("values {} index {}", r.to_string(), self.index);
                    let token = r.values[self.index].clone();
                    self.index += 1;
                    Some(Left(token))
                } else {
                    None
                }
            },
            _ => None
        }
    }
}

pub fn get_deep_token(tokens: &mut Tokens) -> Option<TokenType> {
    match tokens.next() {
        Some(t) => match t {
            Left(t) => get_deep_token(&mut Tokens::new(t)),
            Right(t) => Some(t),
        },
        None => None,
    }
}

pub fn match_keyword(tokens: &mut Tokens, keyword: &str) -> bool {
    let res = get_deep_token(tokens);

    match res {
        Some(TokenType::String(s)) => s == keyword,
        _ => false,
    }
}

pub fn get_identifier(tokens: &mut Tokens) -> Result<String, String> {
    let res = get_deep_token(tokens);

    match res {
        Some(TokenType::String(s)) => Ok(s),
        _ => Err("Expected identifier".to_string()),
    }
}
