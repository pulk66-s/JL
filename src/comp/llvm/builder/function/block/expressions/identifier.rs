use crate::comp::llvm::llvm_object::LlvmObject;

use super::ValueExpression;

pub struct Identifier {
    pub name: String,
    pub value: ValueExpression,
}

impl Identifier {
    pub fn new(name: String, value: ValueExpression) -> Self {
        Self { name, value }
    }
}

impl LlvmObject for Identifier {
    fn to_llvm_ir(&self) -> String {
        format!("%{} = {}", self.name, self.value.to_llvm_ir())
    }
}
