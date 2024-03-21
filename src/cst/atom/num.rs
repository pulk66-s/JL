use crate::cst::parser::{env::Env, Parser, ParserDataType};

use super::Atom;

#[derive(Clone)]
pub struct NumAtom {
    pub value: Option<i64>,
    pub possible_values: Vec<i64>,
}

impl NumAtom {
    fn get_rest(&self, content: String) -> String {
        let mut index = 0;

        loop {
            if index >= content.len() {
                break;
            }
            if content.chars().nth(index).unwrap().is_numeric() {
                index += 1;
            } else {
                break;
            }
        }
        return content[index..].to_string();
    }

    fn get_number(&self, content: String) -> Option<i64> {
        let mut index = 0;
        let mut value = 0;

        loop {
            if index >= content.len() {
                break;
            }
            if content.chars().nth(index).unwrap().is_numeric() {
                value = value * 10 + content.chars().nth(index).unwrap().to_digit(10).unwrap() as i64;
                index += 1;
            } else {
                break;
            }
        }
        if index == 0 {
            return None;
        }
        return Some(value);
    }
}

impl Parser for NumAtom {
    fn parse(&self, content: &String, _env: &Env) -> Result<(ParserDataType, String), String> {
        match self.get_number(content.to_string()) {
            Some(value) if self.possible_values.contains(&value) => {
                let rest = self.get_rest(content.to_string());

                return Ok((
                    ParserDataType::Atom(Atom::Num(self.clone())),
                    rest,
                ));
            }
            Some(_) => Err("Value not in possible values".to_string()),
            None => Err("Not a number".to_string()),
        }
    }

    fn to_string(&self) -> String {
        let mut fmt = "Num: {".to_string();

        match self.value {
            Some(value) => fmt.push_str(&value.to_string()),
            None => fmt.push_str("None"),
        }
        fmt.push_str(", ");
        fmt.push_str(
            &self
                .possible_values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(", "),
        );
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
