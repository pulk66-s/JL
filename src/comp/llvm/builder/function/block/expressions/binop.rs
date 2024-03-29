use crate::comp::llvm::{builder::types::Type, llvm_object::LlvmObject};

use super::ValueExpression;

#[derive(Debug, Clone)]
pub enum BinOpType {
    ADD,
    SUB,
    MUL,
}

#[derive(Clone)]
pub struct BinOpBuilder {}

impl BinOpBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_binop(
        &self,
        op: BinOpType,
        lhs: ValueExpression,
        rhs: ValueExpression,
        result_type: Option<Type>,
    ) -> BinOp {
        BinOp::new(
            op,
            match result_type {
                Some(t) => t,
                None => Type::Int32,
            },
            lhs,
            rhs,
        )
    }
}

#[derive(Debug, Clone)]
pub struct BinOp {
    pub result_type: Type,
    pub op_type: BinOpType,
    pub lhs: Box<ValueExpression>,
    pub rhs: Box<ValueExpression>,
}

impl BinOp {
    pub fn new(
        op_type: BinOpType,
        result_type: Type,
        lhs: ValueExpression,
        rhs: ValueExpression,
    ) -> Self {
        Self {
            op_type,
            result_type,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
}

impl LlvmObject for BinOp {
    fn to_llvm_ir(&self) -> String {
        format!(
            "{} {} {}, {}",
            match self.op_type {
                BinOpType::ADD => "add",
                BinOpType::SUB => "sub",
                BinOpType::MUL => "mul",
            },
            self.result_type.to_llvm_ir(),
            self.lhs.to_llvm_ir(),
            self.rhs.to_llvm_ir()
        )
    }
}
