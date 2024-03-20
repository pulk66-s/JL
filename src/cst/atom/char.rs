use crate::cst::parser::{Parser, ParserDataType};

#[derive(Clone)]
pub struct CharAtom {
    pub value: char,
}

impl Parser for CharAtom {
    fn parse(
        &self,
        content: &String,
        _env: &crate::cst::parser::env::Env,
    ) -> Result<ParserDataType, String> {
        Err("Not implemented".to_string())
    }

    fn to_string(&self) -> String {
        format!("CharAtom: {}", self.value)
    }
}
