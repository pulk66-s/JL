use crate::comp::llvm::llvm_object::LlvmObject;

pub struct TypesBuilder {

}

impl TypesBuilder {
    pub fn new() -> Self {
        Self {

        }
    }
}

pub enum Type {
    Int32
}

impl LlvmObject for Type {
    fn to_llvm_ir(&self) -> String {
        match self {
            Type::Int32 => "i32".to_string()
        }
    }
}
