use crate::comp::llvm::llvm_object::LlvmObject;

use self::return_term::Return;

pub mod return_term;

pub enum Terminator {
    RETURN(Return)
}

impl LlvmObject for Terminator {
    fn to_llvm_ir(&self) -> String {
        match self {
            Terminator::RETURN(return_term) => return_term.to_llvm_ir(),
        }
    }
}
