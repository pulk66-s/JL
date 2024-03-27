use crate::comp::llvm::llvm_object::LlvmObject;

use self::{identifier::Identifier, terminator::Terminator};

pub mod binop;
pub mod identifier;
pub mod terminator;

pub enum ValueExpression {
    NUMBER(i32),
    IDENTIFIER(String),
}

impl LlvmObject for ValueExpression {
    fn to_llvm_ir(&self) -> String {
        match self {
            ValueExpression::NUMBER(n) => n.to_string(),
            ValueExpression::IDENTIFIER(identifier) => identifier.to_string(),
        }
    }
}

pub enum BlockExpression {
    IDENTIFIER(Identifier),
    TERMINATOR(Terminator),
}

impl LlvmObject for BlockExpression {
    fn to_llvm_ir(&self) -> String {
        match self {
            BlockExpression::IDENTIFIER(identifier) => identifier.to_llvm_ir(),
            BlockExpression::TERMINATOR(terminator) => terminator.to_llvm_ir(),
        }
    }
}
