use crate::cst::parser::{Parser, ParserDataType};

use super::Atom;

#[derive(Clone)]
pub struct StringAtom {
    pub value: String,
}

impl Parser for StringAtom {
    fn parse(
        &self,
        content: &String,
        _env: &crate::cst::parser::env::Env,
    ) -> Result<ParserDataType, String> {
        if content.starts_with(&self.value) {
            Ok(ParserDataType::Atom(Atom::String(StringAtom {
                value: self.value.clone(),
            })))
        } else {
            Err(format!("Expected {}, got {}", self.value, content))
        }
    }

    fn to_string(&self) -> String {
        self.value.clone()
    }
}

impl StringAtom {
    pub fn new(value: String) -> StringAtom {
        StringAtom { value }
    }
}
