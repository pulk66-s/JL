use super::parser::{env::Env, Parser, ParserDataType};

pub mod char;
pub mod num;
pub mod string;

#[derive(Clone)]
pub enum Atom {
    Num(num::NumAtom),
    Char(char::CharAtom),
    String(string::StringAtom),
}

impl Parser for Atom {
    fn parse(&mut self, content: &String, env: &Env) -> Result<(ParserDataType, String), String> {
        match self {
            Atom::Num(parser) => parser.parse(content, env),
            Atom::Char(parser) => parser.parse(content, env),
            Atom::String(parser) => parser.parse(content, env),
        }
    }

    fn to_string(&self) -> String {
        match self {
            Atom::Num(parser) => parser.to_string(),
            Atom::Char(parser) => parser.to_string(),
            Atom::String(parser) => parser.to_string(),
        }
    }
}
