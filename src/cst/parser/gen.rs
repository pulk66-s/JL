use super::env::Env;
use super::{generate_parser_with_env, ParserDataType};
use std::fs::File;
use std::io::Read;

fn read_file(file: &str) -> Result<String, String> {
    let mut file = match File::open(file) {
        Ok(file) => file,
        Err(_) => return Err("File not found".to_string()),
    };
    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(_) => Err("Error reading file".to_string()),
    }
}

fn split_content(content: &String) -> Vec<String> {
    let mut res = Vec::new();
    let mut sub_content = String::new();
    let mut string = false;

    for letter in content.chars() {
        match letter {
            '\n' | '\t' => {
                if sub_content.len() > 0 {
                    res.push(sub_content.clone());
                    sub_content.clear();
                }
            },
            '\"' | '\'' => {
                string = !string;
                sub_content.push(letter);
            },
            ' ' if !string => {
                if sub_content.len() > 0 {
                    res.push(sub_content.clone());
                    sub_content.clear();
                }
            },
            _ => sub_content.push(letter),
        }
    }
    if sub_content.len() > 0 {
        res.push(sub_content);
    }
    res
}

pub fn generate_parser(file: &str) -> Result<(ParserDataType, Env), String> {
    let file_content = match read_file(file) {
        Ok(content) => content,
        Err(err) => return Err(err),
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
