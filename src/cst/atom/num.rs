use crate::cst::parser::{env::Env, Parser, ParserDataType};

use super::Atom;

#[derive(Clone)]
pub struct NumAtom {
    pub value: Option<i64>,
    pub possible_values: Vec<i64>,
}

impl Parser for NumAtom {
    fn parse(&self, content: &String, _env: &Env) -> Result<ParserDataType, String> {
        match content.parse::<i64>() {
            Ok(value) if self.possible_values.contains(&value) => {
                Ok(ParserDataType::Atom(Atom::Num(NumAtom {
                    value: Some(value),
                    possible_values: self.possible_values.clone(),
                })))
            }
            Ok(_) => Err("Value not in possible values".to_string()),
            Err(_) => Err("Not a number".to_string()),
        }
    }

    fn to_string(&self) -> String {
        let mut fmt = "Num: {".to_string();

        match self.value {
            Some(value) => fmt.push_str(&value.to_string()),
            None => fmt.push_str("None"),
        }
        fmt.push_str(", ");
        fmt.push_str(&self.possible_values.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(", "));
        fmt.push_str("}");
        fmt
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
