use crate::comp::llvm::llvm_object::LlvmObject;

use self::{binop::BinOp, identifier::Identifier, terminator::Terminator};

pub mod binop;
pub mod identifier;
pub mod terminator;

#[derive(Debug, Clone)]
pub enum DirectValueExpression {
    NUMBER(i64),
    IDENTIFIER(Identifier),
}

impl LlvmObject for DirectValueExpression {
    fn to_llvm_ir(&self) -> String {
        match self {
            DirectValueExpression::NUMBER(n) => n.to_string(),
            DirectValueExpression::IDENTIFIER(identifier) => identifier.to_llvm_ir(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum IndirectValueExpression {
    BINOP(BinOp),
}

impl LlvmObject for IndirectValueExpression {
    fn to_llvm_ir(&self) -> String {
        match self {
            IndirectValueExpression::BINOP(binop) => binop.to_llvm_ir(),
        }
    }
}

#[derive(Debug, Clone)]
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
