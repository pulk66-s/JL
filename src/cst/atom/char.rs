use crate::cst::{node::Node, parser::Parser};

#[derive(Clone)]
pub struct CharAtom {
    pub value: char
}

impl Node for CharAtom {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl Parser for CharAtom {
    fn parse(&self, content: &String, _env: &crate::cst::parser::env::Env) -> Result<Box<dyn Node>, String> {
        Err("Not implemented".to_string())
    }
}
