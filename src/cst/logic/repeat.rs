use crate::cst::{
    atom::{num::NumAtom, string::StringAtom, Atom},
    parser::{env::Env, Parser, ParserDataType},
};

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

    fn combine_num(&self, values: Vec<ParserDataType>) -> ParserDataType {
        ParserDataType::Atom(Atom::Num(NumAtom::new(
            vec![],
            Some(
                values
                    .iter()
                    .map(|value| match value {
                        ParserDataType::Atom(Atom::Num(value)) => match value.value {
                            Some(value) => value.to_string(),
                            _ => "".to_string(),
                        },
                        _ => "".to_string(),
                    })
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap(),
            ),
        )))
    }

    fn combine_char(&self, values: Vec<ParserDataType>) -> ParserDataType {
        ParserDataType::Atom(Atom::String(StringAtom::new(
            values
                .iter()
                .map(|value| match value {
                    ParserDataType::Atom(Atom::Char(value)) => value.value.to_string(),
                    _ => "".to_string(),
                })
                .collect::<String>(),
        )))
    }

    fn combine_arr(&self, values: Vec<ParserDataType>) -> ParserDataType {
        let array = values
            .iter()
            .map(|x| match x {
                ParserDataType::Array(a) => a.value.clone(),
                _ => None,
            })
            .filter(|x| x.is_some())
            .map(|x| *x.unwrap())
            .collect::<Vec<ParserDataType>>();

        match array[0] {
            ParserDataType::Atom(Atom::Char(_)) => self.combine_char(array),
            ParserDataType::Atom(Atom::Num(_)) => self.combine_num(array),
            _ => ParserDataType::Repeat(self.clone()),
        }
    }

    fn combine(&self) -> ParserDataType {
        match &self.values[0] {
            ParserDataType::Atom(Atom::Char(_)) => self.combine_char(self.values.clone()),
            ParserDataType::Atom(Atom::Num(_)) => self.combine_num(self.values.clone()),
            ParserDataType::Array(arr) => match arr.value.clone() {
                Some(v) => self.combine_arr(self.values.clone()),
                None => ParserDataType::Repeat(self.clone()),
            },
            _ => ParserDataType::Repeat(self.clone()),
        }
    }
}

impl Parser for Repeat {
    fn to_string(&self) -> String {
        let mut res = "{\"Repeat\": {".to_string();

        res += "\"values\": [";
        for value in self.values.iter() {
            res += &value.to_string();
        }
        res += "]}}";
        return res;
    }

    fn parse(&mut self, content: &String, env: &Env) -> Result<(ParserDataType, String), String> {
        let mut res = content.clone();

        loop {
            let (expr, rest) = match self.expr.parse(&res, env) {
                Ok((expr, rest)) => (expr, rest),
                Err(e) => break,
            };

            res = rest;
            self.values.push(expr);
        }
        match self.values.len() {
            0 => Err("No value found".to_string()),
            _ => Ok((self.combine(), res)),
        }
    }
}
