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

    pub fn get_definition(&self, name: String) -> Result<Option<ParserDataType>, String> {
        match self.definitions.iter().find(|x| x.0 == name) {
            Some((_, definition)) => Ok(definition.clone()),
            None => Err("Parser not found".to_string()),
        }
    }

    pub fn add_definition(&mut self, name: String, parser: ParserDataType) {
        match self.definitions.iter().position(|x| x.0 == name) {
            Some(pos) => self.definitions[pos].1 = Some(parser),
            None => self.definitions.push((name, Some(parser))),
        }
    }

    pub fn declaration_exist(&self, name: String) -> bool {
        self.definitions.iter().any(|x: &(String, Option<ParserDataType>)| x.0 == name)
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();

        for (name, parser) in self.definitions.iter() {
            result.push_str(&format!("{} = {}\n", name, parser.as_ref().map_or("".to_string(), |x| x.to_string())));
        }
        result
    }
}
