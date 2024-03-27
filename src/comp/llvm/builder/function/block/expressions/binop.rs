use crate::comp::llvm::llvm_object::LlvmObject;

use super::ValueExpression;

pub enum BinOpType {
    ADD,
    SUB,
    MUL,
}

pub struct BinOp {
    pub op_type: BinOpType,
    pub lhs: Box<ValueExpression>,
    pub rhs: Box<ValueExpression>,
}

impl BinOp {
    pub fn new(op_type: BinOpType, lhs: ValueExpression, rhs: ValueExpression) -> Self {
        Self {
            op_type,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
}

impl LlvmObject for BinOp {
    fn to_llvm_ir(&self) -> String {
        format!("")
    }
}
