use crate::comp::llvm::llvm_object::LlvmObject;

#[derive(Clone)]
pub struct TypesBuilder {}

impl TypesBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create(&self, name: String) -> Option<Type> {
        match name.as_str() {
            "int" => Some(Type::Int32),
            "bool" => Some(Type::Bool),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    Int32,
    Bool,
}

impl LlvmObject for Type {
    fn to_llvm_ir(&self) -> String {
        match self {
            Type::Int32 => "i32".to_string(),
            Type::Bool => "i1".to_string(),
        }
    }
}
