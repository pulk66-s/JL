use crate::cst::parser::{env::Env, Parser, ParserDataType};

#[derive(Clone)]
pub struct Repeat {
    pub expr: Box<ParserDataType>,
    pub values: Vec<ParserDataType>,
}

impl Repeat {
    pub fn new(expr: ParserDataType) -> Self {
        Repeat {
            expr: Box::new(expr),
            values: vec![],
        }
    }
}

impl Parser for Repeat {
    fn to_string(&self) -> String {
        let mut res = "".to_string();

        res += &self.expr.to_string();
        res += "[";
        for value in self.values.iter() {
            res += &value.to_string();
        }
        res += "]";
        return res;
    }

    fn parse(&mut self, content: &String, env: &Env) -> Result<(ParserDataType, String), String> {
        let mut res = content.clone();

        loop {
            let (expr, rest) = match self.expr.parse(&res, env) {
                Ok((expr, rest)) => (expr, rest),
                Err(_) => break,
            };

            res = rest;
            self.values.push(expr);
        }
        match self.values.len() {
            0 => Err("No value found".to_string()),
            _ => Ok((ParserDataType::Repeat(self.clone()), res)),
        }
    }
}
