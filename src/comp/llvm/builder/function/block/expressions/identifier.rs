use crate::comp::llvm::llvm_object::LlvmObject;

use super::IndirectValueExpression;

#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
    pub value: Option<Box<IndirectValueExpression>>,
}

impl Identifier {
    pub fn new(name: String, value: Option<IndirectValueExpression>) -> Self {
        Self {
            name,
            value: match value {
                Some(v) => Some(Box::new(v)),
                None => None,
            },
        }
    }
}

impl LlvmObject for Identifier {
    fn to_llvm_ir(&self) -> String {
        match self.value {
            Some(ref v) => format!("%{} = {}", self.name, v.to_llvm_ir()),
            None => format!("%{}", self.name),
        }
    }
}

#[derive(Clone)]
pub struct IdentifierBuilder {}

impl IdentifierBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(&self, name: String, value: Option<IndirectValueExpression>) -> Identifier {
        Identifier::new(name, value)
    }
}
