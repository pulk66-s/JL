use crate::comp::llvm::{builder::types::Type, llvm_object::LlvmObject};

use self::return_term::Return;

use super::DirectValueExpression;

pub mod return_term;

#[derive(Debug, Clone)]
pub enum Terminator {
    RETURN(Return),
}

impl LlvmObject for Terminator {
    fn to_llvm_ir(&self) -> String {
        match self {
            Terminator::RETURN(return_term) => return_term.to_llvm_ir(),
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
}
