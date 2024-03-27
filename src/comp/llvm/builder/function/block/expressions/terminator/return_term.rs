use crate::comp::llvm::{
    builder::{function::block::expressions::ValueExpression, types::Type},
    llvm_object::LlvmObject,
};

pub struct Return {
    pub value: ValueExpression,
    pub ty: Type,
}

impl Return {
    pub fn new(value: ValueExpression, ty: Type) -> Self {
        Self { value, ty }
    }
}

impl LlvmObject for Return {
    fn to_llvm_ir(&self) -> String {
        format!("ret {} {}", self.ty.to_llvm_ir(), self.value.to_llvm_ir())
    }
}
