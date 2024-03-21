use crate::cst::parser::{Parser, ParserDataType};

use super::Atom;

#[derive(Clone)]
pub struct CharAtom {
    pub value: char,
}

impl CharAtom {
    pub fn new(value: char) -> CharAtom {
        CharAtom { value }
    }
}

impl Parser for CharAtom {
    fn parse(
        &self,
        content: &String,
        _env: &crate::cst::parser::env::Env,
    ) -> Result<(ParserDataType, String), String> {
        if content.len() == 1 {
            if content.chars().next() == Some(self.value) {
                return Ok((
                    ParserDataType::Atom(Atom::Char(self.clone())),
                    content[1..].to_string(),
                ));
            }
        }
        Err(format!("Expected: {}, Found: {}", self.value, content))
    }

    fn to_string(&self) -> String {
        format!("CharAtom: {}", self.value)
    }
}
