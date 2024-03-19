use crate::cst::{node::Node, parser::Parser};

#[derive(Clone)]
pub struct StringAtom {
    pub value: String
}

impl Node for StringAtom {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

impl Parser for StringAtom {
    fn parse(&self, content: &String, _env: &crate::cst::parser::env::Env) -> Result<Box<dyn Node>, String> {
        if content.starts_with(&self.value) {
            Ok(Box::new(StringAtom { value: self.value.clone() }))
        } else {
            Err(format!("Expected {}, got {}", self.value, content))
        }
    }
}
