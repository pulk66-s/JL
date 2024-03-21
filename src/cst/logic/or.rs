use crate::cst::parser::{env::Env, Parser, ParserDataType};

#[derive(Clone)]
pub struct Or {
    pub left: Box<ParserDataType>,
    pub right: Box<ParserDataType>,
}

impl Or {
    pub fn new(left: ParserDataType, right: ParserDataType) -> Or {
        Or {
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

impl Parser for Or {
    fn parse(&mut self, input: &String, env: &Env) -> Result<(ParserDataType, String), String> {
        let left = self.left.parse(input, env);
        let right = self.right.parse(input, env);

        match (left, right) {
            (Ok(l), _) => Ok(l),
            (_, Ok(r)) => Ok(r),
            (Err(left_err), Err(right_err)) => Err(format!("{} and {}", left_err, right_err)),
        }
    }

    fn to_string(&self) -> String {
        format!("Or: {{{}, {}}}", self.left.to_string(), self.right.to_string())
    }
}
