use crate::cst::{node::Node, parser::{env::Env, Parser}};

#[derive(Clone)]
pub struct NumAtom {
    pub value: Option<i64>,
    pub possible_values: Vec<i64>,
}

impl Node for NumAtom {
    fn to_string(&self) -> String {
        match self.value {
            Some(value) => value.to_string(),
            None => "None".to_string(),
        }
    }
}

impl Parser for NumAtom {
    fn parse(&self, content: &String, _env: &Env) -> Result<Box<dyn Node>, String> {
        match content.parse::<i64>() {
            Ok(value) if self.possible_values.contains(&value) => {
                Ok(Box::new(NumAtom {
                    value: Some(value),
                    possible_values: self.possible_values.clone(),
                }))
            },
            Ok(_) => Err("Value not in possible values".to_string()),
            Err(_) => Err("Not a number".to_string()),
        }
    }
}

impl NumAtom {
    pub fn new(values: Vec<i64>) -> NumAtom {
        NumAtom {
            value: None,
            possible_values: values,
        }
    }
}
