#[derive(Debug)]
pub struct EnvFunction {
    pub name: String
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
    pub fn exists(&self, name: &str) -> bool {
        for function in &self.functions {
            if function.name == name {
                return true;
            }
        }
        return false;
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

    pub fn add_function(&mut self, name: String) {
        self.functions.functions.push(EnvFunction {
            name: name
        });
    }
}
