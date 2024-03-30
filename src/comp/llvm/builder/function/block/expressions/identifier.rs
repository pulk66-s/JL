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
pub struct IdentifierBuilder {
    name_index: usize,
}

impl IdentifierBuilder {
    pub fn new() -> Self {
        Self { name_index: 0 }
    }

    pub fn build(
        &mut self,
        name: Option<String>,
        value: Option<IndirectValueExpression>,
    ) -> Identifier {
        Identifier::new(
            match name {
                Some(n) => n,
                None => {
                    self.name_index += 1;
                    format!("identifier_{}", self.name_index)
                }
            },
            value,
        )
    }
}
