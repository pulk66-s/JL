use crate::{
    ast::AstExpr,
    comp::llvm::{
        builder::function::block::expressions::{
            identifier::Identifier, BlockExpression, ValueExpression,
        },
        module::Module,
    },
};

use super::{binop::create_binop_expr, ret::create_return_expr};

pub fn create_expr(expr: &AstExpr, module: Module) -> Result<(BlockExpression, Module), String> {
    match expr.clone() {
        AstExpr::NUMBER(n) => Ok((BlockExpression::VALUE(ValueExpression::NUMBER(n)), module)),
        AstExpr::BINOP(binop) => match create_binop_expr(binop, module) {
            Ok((b, module)) => Ok((BlockExpression::VALUE(ValueExpression::BINOP(b)), module)),
            Err(e) => Err(e),
        },
        AstExpr::RETURN(ret) => match create_return_expr(ret, module) {
            Ok((t, module)) => Ok((BlockExpression::TERMINATOR(t), module)),
            Err(e) => Err(e),
        },
        AstExpr::VARIABLE_CALL(v) => Ok((
            BlockExpression::VALUE(ValueExpression::IDENTIFIER(Identifier::new(v, None))),
            module,
        )),
        e => Err(format!(
            "Unknown expression in create_expr {}",
            e.to_string()
        )),
    }
}
