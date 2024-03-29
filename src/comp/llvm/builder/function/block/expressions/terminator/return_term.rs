use crate::comp::llvm::{
    builder::{function::block::expressions::DirectValueExpression, types::Type},
    llvm_object::LlvmObject,
};

#[derive(Debug, Clone)]
pub struct Return {
    pub value: DirectValueExpression,
    pub ty: Type,
}

impl Return {
    pub fn new(value: DirectValueExpression, ty: Option<Type>) -> Self {
        Self {
            value,
            ty: match ty {
                Some(t) => t,
                None => Type::Int32,
            },
        }
    }
}

impl LlvmObject for Return {
    fn to_llvm_ir(&self) -> String {
        format!(
            "ret {} {}",
            self.ty.to_llvm_ir(),
            match &self.value {
                DirectValueExpression::IDENTIFIER(i) => format!("%{}", i.name),
                e => e.to_llvm_ir(),
            }
        )
    }
}
