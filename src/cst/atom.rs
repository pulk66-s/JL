use super::parser::{env::Env, Parser};

pub mod num;
pub mod char;
pub mod string;

#[derive(Clone)]
pub enum Atom {
    Num(num::NumAtom),
    Char(char::CharAtom),
    String(string::StringAtom),
}

impl Parser for Atom {
    fn parse(&self, content: &String, env: &Env) -> Result<Box<dyn super::node::Node>, String> {
        match self {
            Atom::Num(parser) => parser.parse(content, env),
            Atom::Char(parser) => parser.parse(content, env),
            Atom::String(parser) => parser.parse(content, env),
        }
    }
}