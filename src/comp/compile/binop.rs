use crate::{
    ast::binop::AstBinop,
    comp::llvm::{
        builder::function::block::expressions::{
            binop::{BinOp, BinOpType},
            BlockExpression, ValueExpression,
        },
        module::Module,
    },
};

use super::expr::create_expr;

fn create_binop_type(
    lhs: ValueExpression,
    rhs: ValueExpression,
    op: String,
    module: Module,
) -> Result<(BinOp, Module), String> {
    match op.as_str() {
        "+" => Ok((
            module
                .builder
                .binop
                .create_binop(BinOpType::ADD, lhs, rhs, None),
            module,
        )),
        _ => Err("Unknown operator".to_string()),
    }
}

pub fn create_binop_expr(binop: AstBinop, module: Module) -> Result<(BinOp, Module), String> {
    match create_expr(&binop.left, module) {
        Ok((BlockExpression::VALUE(lhs), module)) => match create_expr(&binop.right, module) {
            Ok((BlockExpression::VALUE(rhs), module)) => {
                create_binop_type(lhs, rhs, binop.op, module)
            }
            Err(e) => Err(e),
            _ => Err("Error in binop".to_string()),
        },
        Err(e) => Err(e),
        _ => Err("Unknown binop".to_string()),
    }
}
