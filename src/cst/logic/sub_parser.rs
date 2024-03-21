use crate::cst::parser::{env::Env, Parser, ParserDataType};

#[derive(Clone)]
pub struct SubParser {
    pub name: String,
    pub value: Option<Box<ParserDataType>>,
}

impl SubParser {
    pub fn new(name: String, value: Option<ParserDataType>) -> Self {
        SubParser {
            name: name,
            value: match value {
                Some(v) => Some(Box::new(v)),
                None => None,
            },
        }
    }
}

impl Parser for SubParser {
    fn parse(
        &mut self,
        content: &String,
        env: &Env,
    ) -> Result<(ParserDataType, String), String> {
        let mut parser = match env.get_definition(self.name.clone()) {
            Ok(Some(p)) => p,
            _ => return Err("No definition for subparser".to_string())
        };
        let (res, rest) = match parser.parse(content, env) {
            Ok(r) => r,
            Err(e) => return Err(e)
        };

        self.value = Some(Box::new(res.clone()));
        Ok((res, rest))
    }

    fn to_string(&self) -> String {
        format!("{{\"SubParser\": {{\"name\": \"{}\", \"value\": {}}}}}", self.name, match &self.value {
            Some(r) => r.to_string(),
            None => "None".to_string()
        })
    }
}
