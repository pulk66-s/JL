use crate::comp::llvm::{builder::types::Type, llvm_object::LlvmObject};

pub struct FunctionParam {
    pub name: String,
    pub ty: Type,
}

impl FunctionParam {
    pub fn new(name: String, ty: Type) -> Self {
        Self { name: name, ty: ty }
    }
}

impl LlvmObject for FunctionParam {
    fn to_llvm_ir(&self) -> String {
        format!("{} {}", self.ty.to_llvm_ir(), self.name)
    }
}
