use super::env::Env;
use super::{generate_parser_with_env, Parser};
use std::fs::File;
use std::io::Read;

fn read_file(file: &str) -> Result<String, String> {
    let mut file = match File::open(file) {
        Ok(file) => file,
        Err(_) => return Err("File not found".to_string())
    };
    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(_) => Err("Error reading file".to_string())
    }
}

fn split_content(content: &String) -> Vec<String> {
    content.split([' ', '\n', '\t']).map(|x| x.trim().to_string()).collect()
}

pub fn generate_parser(file: &str) -> Result<(Box<dyn Parser>, Env), String> {
    let file_content = match read_file(file) {
        Ok(content) => content,
        Err(err) => return Err(err)
    };
    let content = split_content(&file_content);
    let mut env = Env::new();

    match env.setup_declarations(&content) {
        Some(err) => return Err(err),
        None => (),
    }
    match generate_parser_with_env(&mut env, content) {
        Ok(parser) => Ok((parser, env)),
        Err(err) => Err(err),
    }
}
