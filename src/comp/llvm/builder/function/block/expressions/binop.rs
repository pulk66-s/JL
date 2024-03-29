use crate::comp::llvm::{builder::types::Type, llvm_object::LlvmObject};

use super::DirectValueExpression;

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
        lhs: DirectValueExpression,
        rhs: DirectValueExpression,
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

    pub fn create_op(&self, symbol: String) -> Option<BinOpType> {
        match symbol.as_str() {
            "+" => Some(BinOpType::ADD),
            "-" => Some(BinOpType::SUB),
            "*" => Some(BinOpType::MUL),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BinOp {
    pub result_type: Type,
    pub op_type: BinOpType,
    pub lhs: Box<DirectValueExpression>,
    pub rhs: Box<DirectValueExpression>,
}

impl BinOp {
    pub fn new(
        op_type: BinOpType,
        result_type: Type,
        lhs: DirectValueExpression,
        rhs: DirectValueExpression,
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
