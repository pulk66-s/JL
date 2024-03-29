use crate::{
    ast::binop::AstBinop,
    comp::llvm::{
        builder::function::block::{expressions::binop::BinOp, Block},
        module::Module,
    },
};

use super::expr::convert_ast_to_direct_value;

pub fn compile_binop(
    ast: &AstBinop,
    module: &mut Module,
    current_block: &mut Block,
) -> Result<BinOp, String> {
    let lhs = match convert_ast_to_direct_value(&*ast.left, module, current_block) {
        Ok(r) => r,
        Err(e) => return Err(e),
        _ => return Err("compile_binop Unknown expression lhs".to_string()),
    };
    let rhs = match convert_ast_to_direct_value(&*ast.right, module, current_block) {
        Ok(r) => r,
        Err(e) => return Err(e),
        _ => return Err("compile_binop Unknown expression rhs".to_string()),
    };
    let op = match module.builder.binop.create_op(ast.op.clone()) {
        Some(x) => x,
        None => return Err("Unknown operator".to_string()),
    };

    Ok(module.builder.binop.create_binop(op, lhs, rhs, None))
}
