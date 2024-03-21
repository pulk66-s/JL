use crate::cst::parser::{env::Env, Parser, ParserDataType};

#[derive(Clone)]
pub struct And {
    pub left: Box<ParserDataType>,
    pub right: Box<ParserDataType>,
}

impl And {
    pub fn new(left: ParserDataType, right: ParserDataType) -> And {
        And {
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

impl Parser for And {
    fn parse(&mut self, input: &String, env: &Env) -> Result<(ParserDataType, String), String> {
        match self.left.parse(input, env) {
            Ok((l, new_input)) => match self.right.parse(&new_input, env) {
                Ok((r, new_input)) => Ok((ParserDataType::And(And::new(l, r)), new_input)),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    fn to_string(&self) -> String {
        format!(
            "{{\"And\": {{\"left\": {}, \"right\": {}}}}}",
            self.left.to_string(),
            self.right.to_string()
        )
    }
}
