use self::env::Env;

use super::{
    atom::{char::CharAtom, num::NumAtom, string::StringAtom, Atom},
    logic::{and::And, array::Array, maybe::Maybe, or::Or, repeat::Repeat, sub_parser::SubParser},
};

pub mod env;
pub mod gen;

#[derive(Clone)]
pub enum ParserDataType {
    Atom(Atom),
    Or(Or),
    And(And),
    Parser(SubParser),
    Repeat(Repeat),
    Maybe(Maybe),
    Array(Array),
    None,
}

impl ParserDataType {
    pub fn to_string(&self) -> String {
        match self {
            ParserDataType::Atom(atom) => atom.to_string(),
            ParserDataType::Or(value) => value.to_string(),
            ParserDataType::And(value) => value.to_string(),
            ParserDataType::Parser(value) => value.to_string(),
            ParserDataType::Repeat(value) => value.to_string(),
            ParserDataType::Maybe(value) => value.to_string(),
            ParserDataType::Array(value) => value.to_string(),
            ParserDataType::None => "\"None\"".to_string(),
        }
    }

    fn parse_subparser(
        &mut self,
        name: String,
        content: &String,
        env: &Env,
    ) -> Result<(ParserDataType, String), String> {
        match env.get_definition(name) {
            Ok(Some(mut parser)) => parser.parse(content, env),
            _ => Err("No definition provided".to_string()),
        }
    }

    pub fn parse(
        &mut self,
        content: &String,
        env: &Env,
    ) -> Result<(ParserDataType, String), String> {
        match self {
            ParserDataType::Atom(atom) => atom.parse(content, env),
            ParserDataType::Or(value) => value.parse(content, env),
            ParserDataType::And(value) => value.parse(content, env),
            ParserDataType::Parser(value) => value.parse(content, env),
            ParserDataType::Repeat(value) => value.parse(content, env),
            ParserDataType::Maybe(value) => value.parse(content, env),
            ParserDataType::Array(value) => value.parse(content, env),
            ParserDataType::None => Err("None can't be parsed".to_string()),
        }
    }
}

pub trait Parser: Clone {
    fn parse(&mut self, content: &String, env: &Env) -> Result<(ParserDataType, String), String>;
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

fn get_definition(
    i: usize,
    env: &Env,
    content: &Vec<String>,
) -> Result<(String, Vec<String>), String> {
    match (content.get(i - 1), get_definition_value(env, content, i)) {
        (_, vec) if vec.len() == 0 => return Err("No value found in get_definition".to_string()),
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

fn generate_and_parser(
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

    Ok(ParserDataType::And(And::new(vec![
        Box::new(left),
        Box::new(right),
    ])))
}

fn generate_parser_numeric(value: &String) -> Result<ParserDataType, String> {
    let parser = match value.parse::<i64>() {
        Ok(value) => Atom::Num(NumAtom::new(vec![value], None)),
        Err(_) => return Err("Not a number".to_string()),
    };
    Ok(ParserDataType::Atom(parser))
}

fn generate_parser_string(value: &String) -> Result<ParserDataType, String> {
    let value_without_quotes = value.trim_matches('\"');
    let parser = Atom::String(StringAtom::new(value_without_quotes.to_string()));

    Ok(ParserDataType::Atom(parser))
}

fn generate_parser_char(value: &String) -> Result<ParserDataType, String> {
    let value_without_quotes = value.trim_matches('\'');
    let parser = Atom::Char(CharAtom::new(match value_without_quotes.chars().next() {
        Some('\\') => match value_without_quotes.chars().nth(1) {
            Some('n') => '\n',
            Some('t') => '\t',
            Some('r') => '\r',
            Some('0') => '\0',
            Some('\'') => '\'',
            Some('\"') => '\"',
            Some('\\') => '\\',
            _ => return Err("generate_parser_char, Syntax error".to_string()),
        },
        Some(value) => value,
        None => return Err("generate_parser_char, Syntax error".to_string()),
    }));

    Ok(ParserDataType::Atom(parser))
}

fn generate_parser_repeat(value: String, env: &mut Env) -> Result<ParserDataType, String> {
    let delete_last = value[..(value.len() - 1)].to_string();
    let parser = match generate_parser_body(vec![delete_last], env) {
        Ok(r) => r,
        Err(err) => return Err(err),
    };

    Ok(ParserDataType::Repeat(Repeat::new(parser)))
}

fn generate_parser_maybe(value: String, env: &mut Env) -> Result<ParserDataType, String> {
    let delete_last = value[..(value.len() - 1)].to_string();
    let parser = match generate_parser_body(vec![delete_last], env) {
        Ok(r) => r,
        Err(err) => return Err(err),
    };

    Ok(ParserDataType::Maybe(Maybe::new(parser)))
}

fn generate_parser_array(value: &String, env: &mut Env) -> Result<ParserDataType, String> {
    let values = value
        .trim_matches(['[', ']'])
        .split("..")
        .collect::<Vec<&str>>();
    let start = match generate_parser_body(vec![values[0].to_string()], env) {
        Ok(r) => r,
        Err(err) => return Err(err),
    };
    let end = match generate_parser_body(vec![values[1].to_string()], env) {
        Ok(r) => r,
        Err(err) => return Err(err),
    };

    Ok(ParserDataType::Array(Array::new(start, end)))
}

fn generate_parser_body(values: Vec<String>, env: &mut Env) -> Result<ParserDataType, String> {
    if values.len() == 1 {
        match values[0].chars().last() {
            Some('*') => return generate_parser_repeat(values[0].clone(), env),
            Some('?') => return generate_parser_maybe(values[0].clone(), env),
            _ => (),
        }
        if values[0].chars().next() == Some('\"') {
            return generate_parser_string(&values[0]);
        }
        if env.get_definition(values[0].to_string()).is_ok() {
            return Ok(ParserDataType::Parser(SubParser::new(
                values[0].to_string(),
                None,
            )));
        }
        match values[0].chars().next() {
            Some('0'..='9') => return generate_parser_numeric(&values[0]),
            Some('\"') => return generate_parser_string(&values[0]),
            Some('\'') => return generate_parser_char(&values[0]),
            Some('[') => return generate_parser_array(&values[0], env),
            _ => return Err("generate_parser_body, Syntax error".to_string() + &values[0]),
        }
    }
    for (i, value) in values.iter().enumerate() {
        if value == "|" {
            return generate_or_parser(values, i, env);
        }
        if value == "&" {
            return generate_and_parser(values, i, env);
        }
    }
    Err("generate_parser_body 2, Syntax error".to_string())
}

fn save_parser_in_env(name: String, parser: ParserDataType, env: &mut Env) {
    env.add_definition(name, parser);
}

// return Some(String) when error
pub fn generate_parser_with_env(
    env: &mut Env,
    content: Vec<String>,
) -> Result<ParserDataType, String> {
    let mut first_parser = None;
    for (i, word) in content.iter().enumerate() {
        if word == "=" {
            let (name, values) = match get_definition(i, env, &content) {
                Ok(r) => r,
                Err(err) => return Err(err),
            };

            let parser = match generate_parser_body(values, env) {
                Ok(parser) => parser,
                Err(err) => return Err(err),
            };
            save_parser_in_env(name, parser.clone(), env);
            match first_parser {
                Some(_) => (),
                None => first_parser = Some(parser),
            }
        }
    }
    match first_parser {
        Some(parser) => Ok(parser),
        None => Err("No parser found".to_string()),
    }
}
