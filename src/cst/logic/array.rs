use crate::cst::{
    atom::{char::CharAtom, num::NumAtom, Atom},
    parser::{env::Env, Parser, ParserDataType},
};

#[derive(Clone)]
pub struct Array {
    pub start_expr: Box<ParserDataType>,
    pub last_expr: Box<ParserDataType>,
    pub value: Option<Box<ParserDataType>>,
}

impl Array {
    pub fn new(start_expr: ParserDataType, lab_expr: ParserDataType) -> Self {
        Array {
            start_expr: Box::new(start_expr),
            last_expr: Box::new(lab_expr),
            value: None,
        }
    }

    fn parse_char(
        &mut self,
        content: &String,
        s: char,
        l: char,
    ) -> Result<(ParserDataType, String), String> {
        let first_char = match content.chars().next() {
            Some(l) => l,
            None => return Err("No char".to_string()),
        };

        if first_char >= s && first_char <= l {
            let value = Box::new(ParserDataType::Atom(Atom::Char(CharAtom::new(first_char))));
            self.value = Some(value.clone());
            return Ok((*value, content[1..].to_string()));
        }
        return Err("char doesn't match".to_string());
    }

    fn parse_num(
        &mut self,
        content: &String,
        s: i64,
        l: i64,
    ) -> Result<(ParserDataType, String), String> {
        let first_char = match content.chars().next() {
            Some(l) => l,
            None => return Err("No char".to_string()),
        };
        let first_num = match first_char.to_digit(10) {
            Some(e) => e as i64,
            None => return Err(("array, Not a number".to_string())),
        };

        if first_num >= s && first_num <= l {
            let value = Box::new(ParserDataType::Atom(Atom::Num(NumAtom::new(
                vec![],
                Some(first_num),
            ))));

            self.value = Some(value.clone());
            return Ok((*value, content[1..].to_string()));
        }
        return Err("char doesn't match".to_string());
    }
}

impl Parser for Array {
    fn parse(&mut self, content: &String, env: &Env) -> Result<(ParserDataType, String), String> {
        match (*self.start_expr.clone(), *self.last_expr.clone()) {
            (ParserDataType::Atom(Atom::Char(s)), ParserDataType::Atom(Atom::Char(l))) => {
                self.parse_char(content, s.value, l.value)
            }
            (ParserDataType::Atom(Atom::Num(s)), ParserDataType::Atom(Atom::Num(l))) => self
                .parse_num(
                    content,
                    match s.possible_values.len() {
                        0 => return Err("left doesn't have enought value".to_string()),
                        _ => s.possible_values[0],
                    },
                    match l.possible_values.len() {
                        0 => return Err("left doesn't have enought value".to_string()),
                        _ => l.possible_values[0],
                    },
                ),
            _ => return Err("Array only supports char and num".to_string()),
        }
    }

    fn to_string(&self) -> String {
        format!(
            "{{\"Array\": {{\"start\": {}, \"last\": {}, \"value\": {}}}}}",
            self.start_expr.to_string(),
            self.last_expr.to_string(),
            match &self.value {
                Some(e) => e.to_string(),
                None => "None".to_string(),
            }
        )
    }
}
