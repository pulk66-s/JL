use crate::cst::parser::{Parser, ParserDataType};

use super::Atom;

#[derive(Clone)]
pub struct StringAtom {
    pub value: String,
}

impl Parser for StringAtom {
    fn parse(
        &mut self,
        content: &String,
        _env: &crate::cst::parser::env::Env,
    ) -> Result<(ParserDataType, String), String> {
        if content.starts_with(&self.value) {
            Ok((
                ParserDataType::Atom(Atom::String(self.clone())),
                content[self.value.len()..].to_string(),
            ))
        } else {
            Err(format!("Expected {}, got {}", self.value, content))
        }
    }

    fn to_string(&self) -> String {
        format!("StringAtom: '{}'", self.value)
    }
}

impl StringAtom {
    pub fn new(value: String) -> StringAtom {
        StringAtom { value }
    }
}
