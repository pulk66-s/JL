use crate::comp::llvm::llvm_object::LlvmObject;

use super::ValueExpression;

#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
    pub value: Box<ValueExpression>,
}

impl Identifier {
    pub fn new(name: String, value: ValueExpression) -> Self {
        Self {
            name,
            value: Box::new(value),
        }
    }
}

impl LlvmObject for Identifier {
    fn to_llvm_ir(&self) -> String {
        format!("%{} = {}", self.name, self.value.to_llvm_ir())
    }
}

#[derive(Clone)]
pub struct IdentifierBuilder {}

impl IdentifierBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(&self, name: String, value: ValueExpression) -> Identifier {
        Identifier::new(name, value)
    }
}
