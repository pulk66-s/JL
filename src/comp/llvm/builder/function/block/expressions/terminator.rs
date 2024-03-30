use crate::comp::llvm::{
    builder::{function::block::Block, types::Type},
    llvm_object::LlvmObject,
};

use self::{br::Br, return_term::Return};

use super::DirectValueExpression;

pub mod br;
pub mod return_term;

#[derive(Debug, Clone)]
pub enum Terminator {
    RETURN(Return),
    BR(Br),
}

impl LlvmObject for Terminator {
    fn to_llvm_ir(&self) -> String {
        match self {
            Terminator::RETURN(return_term) => return_term.to_llvm_ir(),
            Terminator::BR(br) => br.to_llvm_ir(),
        }
    }
}

#[derive(Clone)]
pub struct TerminatorBuilder {}

impl TerminatorBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_return(&self, value: DirectValueExpression, ty: Option<Type>) -> Terminator {
        Terminator::RETURN(Return::new(value, ty))
    }

    pub fn create_br(&self, condition: DirectValueExpression, tr: Block, fal: Block) -> Terminator {
        Terminator::BR(Br::new(condition, tr, fal))
    }
}
