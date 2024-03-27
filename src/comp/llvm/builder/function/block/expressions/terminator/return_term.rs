use crate::comp::llvm::{
    builder::{function::block::expressions::ValueExpression, types::Type},
    llvm_object::LlvmObject,
};

#[derive(Debug, Clone)]
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
        format!("ret {} {}", self.ty.to_llvm_ir(), match &self.value {
            ValueExpression::IDENTIFIER(i) => format!("%{}", i.name),
            e => e.to_llvm_ir()
        })
    }
}
