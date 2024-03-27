use crate::comp::llvm::llvm_object::LlvmObject;

use self::{binop::BinOp, identifier::Identifier, terminator::Terminator};

pub mod binop;
pub mod identifier;
pub mod terminator;

#[derive(Debug, Clone)]
pub enum ValueExpression {
    NUMBER(i64),
    IDENTIFIER(Identifier),
    BINOP(BinOp),
}

impl LlvmObject for ValueExpression {
    fn to_llvm_ir(&self) -> String {
        match self {
            ValueExpression::NUMBER(n) => n.to_string(),
            ValueExpression::IDENTIFIER(identifier) => identifier.to_llvm_ir(),
            ValueExpression::BINOP(binop) => binop.to_llvm_ir(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BlockExpression {
    IDENTIFIER(Identifier),
    TERMINATOR(Terminator),
    VALUE(ValueExpression),
}

impl LlvmObject for BlockExpression {
    fn to_llvm_ir(&self) -> String {
        match self {
            BlockExpression::IDENTIFIER(identifier) => identifier.to_llvm_ir(),
            BlockExpression::TERMINATOR(terminator) => terminator.to_llvm_ir(),
            BlockExpression::VALUE(value) => value.to_llvm_ir(),
        }
    }
}
