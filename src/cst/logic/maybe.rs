use crate::cst::parser::{env::Env, Parser, ParserDataType};

#[derive(Clone)]
pub struct Maybe {
    pub expr: Box<ParserDataType>,
    pub value: Option<Box<ParserDataType>>,
}

impl Parser for Maybe {
    fn to_string(&self) -> String {
        let mut res = "{\"Maybe\": {\"expr\":".to_string();

        res += &self.expr.to_string();
        res += ", \"value\": ";
        match &self.value {
            Some(value) => res += &value.to_string(),
            None => res += "\"None\"",
        }
        res += "}";
        return res;
    }

    fn parse(&mut self, content: &String, env: &Env) -> Result<(ParserDataType, String), String> {
        let (expr, rest) = match self.expr.parse(content, env) {
            Ok((expr, rest)) => (expr, rest),
            Err(_) => return Ok((ParserDataType::None, content.clone())),
        };

        self.value = Some(Box::new(expr.clone()));
        Ok((expr, rest))
    }
}

impl Maybe {
    pub fn new(expr: ParserDataType) -> Self {
        Maybe {
            expr: Box::new(expr),
            value: None,
        }
    }
}
