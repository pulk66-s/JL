use self::env::Env;

use super::{atom::num::NumAtom, node::Node};

pub mod env;
pub mod gen;

pub trait Parser {
    fn parse(&self, content: &String, env: &Env) -> Result<Box<dyn Node>, String>;
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

fn generate_parser_with_data(
    name: String,
    values: Vec<String>,
    env: &mut Env,
) -> Result<Box<dyn Parser>, String> {
    let first_value = match values.get(0) {
        Some(x) => x,
        None => return Err("values is empty".to_string()),
    };
    let parser = match first_value.chars().next() {
        Some(c) if c.is_numeric() => match first_value.parse::<i64>() {
            Ok(value) => NumAtom::new(vec![value]),
            Err(_) => return Err("Not a number".to_string()),
        },
        _ => return Err("generate_parser_with_data not implemented".to_string()),
    };

    env.add_definition(name, Box::new(parser.clone()));
    Ok(Box::new(parser))
}

// return Some(String) when error
pub fn generate_parser_with_env(
    env: &mut Env,
    content: Vec<String>,
) -> Result<Box<dyn Parser>, String> {
    let (first_name, first_value) = match get_first_definition(&env, &content) {
        Ok((name, value)) => (name, value),
        Err(err) => return Err(err),
    };
    generate_parser_with_data(first_name, first_value, env)
}
