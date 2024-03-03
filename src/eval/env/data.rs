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
pub struct EnvVariable {
    pub name: String,
    pub value: f64
}

#[derive(Debug)]
pub struct EnvVariables {
    pub variables: Vec<EnvVariable>
}

#[derive(Debug)]
pub struct Env {
    pub functions: EnvFunctions,
    pub variables: EnvVariables
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

impl EnvVariables {
    pub fn get_variable(&self, name: String) -> Option<&EnvVariable> {
        for variable in &self.variables {
            if variable.name == name {
                return Some(variable);
            }
        }
        return None;
    }

    pub fn add_variable(&mut self, name: String, value: f64) {
        self.variables.push(EnvVariable {
            name: name,
            value: value
        });
    }
}

impl Env {
    pub fn new() -> Env {
        return Env {
            functions: EnvFunctions {
                functions: vec![]
            },
            variables: EnvVariables {
                variables: vec![]
            }
        };
    }
}
