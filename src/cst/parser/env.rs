use super::{Parser, ParserDataType};

pub struct Env {
    definitions: Vec<(String, Option<ParserDataType>)>,
}

impl Env {
    // return Some(String) when error
    pub fn setup_declarations(&mut self, content: &Vec<String>) -> Option<String> {
        for (i, word) in content.iter().enumerate() {
            match word.chars().next() {
                Some('=') => match content.get(i - 1) {
                    Some(declaration) => self.definitions.push((declaration.to_string(), None)),
                    None => return Some("No declaration found".to_string()),
                },
                _ => continue,
            }
        }
        None
    }

    pub fn new() -> Env {
        Env {
            definitions: vec![],
        }
    }

    pub fn add_definition(&mut self, name: String, parser: ParserDataType) {
        match self.definitions.iter().position(|x| x.0 == name) {
            Some(pos) => self.definitions[pos].1 = Some(parser),
            None => self.definitions.push((name, Some(parser))),
        }
    }
}
