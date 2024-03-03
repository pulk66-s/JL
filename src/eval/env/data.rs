use crate::ast::data::AstNode;

#[derive(Debug, Clone)]
pub struct EnvFunction {
    pub name: String,
    pub body: Vec<AstNode>
}

#[derive(Debug)]
pub struct EnvFunctions {
    pub functions: Vec<EnvFunction>
}

#[derive(Debug)]
pub struct Env {
    pub functions: EnvFunctions
}

impl EnvFunctions {
    pub fn get_function(&self, name: String) -> Option<&EnvFunction> {
        for function in &self.functions {
            if function.name == name {
                return Some(function);
            }
        }
        return None;
    }

    pub fn add_function(&mut self, name: String, body: Vec<AstNode>) {
        self.functions.push(EnvFunction {
            name: name,
            body: body
        });
    }
}

impl Env {
    pub fn new() -> Env {
        return Env {
            functions: EnvFunctions {
                functions: vec![]
            }
        };
    }
}
