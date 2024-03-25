use crate::cst::parser::{env::Env, Parser, ParserDataType};

#[derive(Clone)]
pub struct And {
    pub values: Vec<Box<ParserDataType>>,
}

impl And {
    pub fn new(vec: Vec<Box<ParserDataType>>) -> And {
        And {
            values: vec,
        }
    }
}

impl Parser for And {
    fn parse(&mut self, input: &String, env: &Env) -> Result<(ParserDataType, String), String> {
        let mut result = vec![];
        let mut rest = input.clone();

        for mut value in self.values.clone() {
            let (expr, res) = match value.parse(&rest, env) {
                Ok(r) => r,
                Err(e) => return Err(e),
            };

            match expr {
                ParserDataType::And(mut p) => result.append(&mut p.values),
                e => result.push(Box::new(e)),
            }
            rest = res.clone();
        }
        Ok((ParserDataType::And(And::new(result)), rest.to_string()))
    }

    fn to_string(&self) -> String {
        format!(
            "{{\"And\": {{\"values\": [{}]}}}}",
            self.values
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
