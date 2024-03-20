use self::env::Env;

use super::{
    atom::{num::NumAtom, Atom},
    logic::or::Or,
};

pub mod env;
pub mod gen;

#[derive(Clone)]
pub enum ParserDataType {
    Atom(Atom),
    Or(Or),
}

impl ParserDataType {
    pub fn to_string(&self) -> String {
        match self {
            ParserDataType::Atom(atom) => atom.to_string(),
            ParserDataType::Or(value) => value.to_string(),
        }
    }

    pub fn parse(&self, content: &String, env: &Env) -> Result<ParserDataType, String> {
        match self {
            ParserDataType::Atom(atom) => atom.parse(content, env),
            ParserDataType::Or(value) => value.parse(content, env),
        }
    }
}

pub trait Parser: Clone {
    fn parse(&self, content: &String, env: &Env) -> Result<ParserDataType, String>;
    fn to_string(&self) -> String;
}

fn get_definition_value(env: &Env, content: &Vec<String>, i: usize) -> Vec<String> {
    let mut values = vec![];

    for word in content.iter().skip(i + 1) {
        if word.chars().last() == Some(';') {
            values.push(word.trim_end_matches(';').to_string());
            break;
        }
        values.push(word.to_string());
    }
    values
}

fn get_first_definition(env: &Env, content: &Vec<String>) -> Result<(String, Vec<String>), String> {
    let first_equal = match content.iter().position(|x| x.chars().next() == Some('=')) {
        Some(pos) => pos,
        None => return Err("No definition found".to_string()),
    };
    match (
        content.get(first_equal - 1),
        get_definition_value(env, content, first_equal),
    ) {
        (_, vec) if vec.len() == 0 => return Err("No value found".to_string()),
        (Some(declaration), values) => return Ok((declaration.to_string(), values)),
        (None, _) => return Err("No declaration found".to_string()),
    }
}

fn generate_or_parser(
    values: Vec<String>,
    i: usize,
    env: &mut Env,
) -> Result<ParserDataType, String> {
    let left = match generate_parser_body(vec![values[0].clone()], env) {
        Ok(parser) => parser,
        Err(err) => return Err(err),
    };
    let right = match generate_parser_body(
        values.iter().skip(i + 1).map(|x| x.to_string()).collect(),
        env,
    ) {
        Ok(parser) => parser,
        Err(err) => return Err(err),
    };

    Ok(ParserDataType::Or(Or::new(left, right)))
}

fn generate_parser_numeric(value: &String) -> Result<ParserDataType, String> {
    let parser = match value.parse::<i64>() {
        Ok(value) => Atom::Num(NumAtom::new(vec![value])),
        Err(_) => return Err("Not a number".to_string()),
    };
    Ok(ParserDataType::Atom(parser))
}

fn generate_parser_body(values: Vec<String>, env: &mut Env) -> Result<ParserDataType, String> {
    if values.len() == 1 {
        return generate_parser_numeric(&values[0]);
    }
    for (i, value) in values.iter().enumerate() {
        if value == "|" {
            return generate_or_parser(values, i, env);
        }
    }
    Err("generate_parser_body, Syntax error".to_string())
}

fn save_parser_in_env(name: String, parser: ParserDataType, env: &mut Env) {
    env.add_definition(name, parser);
}

fn generate_parser_with_data(
    name: String,
    values: Vec<String>,
    env: &mut Env,
) -> Result<ParserDataType, String> {
    if values.len() == 1 {
        let parser = match generate_parser_numeric(&values[0]) {
            Ok(p) => p,
            Err(err) => return Err(err),
        };
        return Ok(parser);
    }
    for (i, value) in values.iter().enumerate() {
        if value == "|" {
            return generate_or_parser(values.clone(), i, env);
        }
    }
    Err("generate_parser_with_data, Syntax error".to_string())
}

// return Some(String) when error
pub fn generate_parser_with_env(
    env: &mut Env,
    content: Vec<String>,
) -> Result<ParserDataType, String> {
    let (first_name, first_value) = match get_first_definition(&env, &content) {
        Ok((name, value)) => (name, value),
        Err(err) => return Err(err),
    };
    let parser = match generate_parser_with_data(first_name.clone(), first_value, env) {
        Ok(parser) => parser,
        Err(err) => return Err(err),
    };

    save_parser_in_env(first_name, parser.clone(), env);
    Ok(parser)
}
