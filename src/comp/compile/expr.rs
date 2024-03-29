use crate::{
    ast::AstExpr,
    comp::llvm::{
        builder::function::block::{
            expressions::{
                identifier::Identifier, terminator::Terminator, BlockExpression,
                DirectValueExpression, IndirectValueExpression,
            },
            Block,
        },
        module::Module,
    },
};

use super::{binop::compile_binop, ret::create_llvm_ret};

pub fn compile_ast_direct_value_expr(
    ast: &AstExpr,
    _module: &mut Module,
    _current_block: &mut Block,
) -> Result<DirectValueExpression, String> {
    match ast {
        AstExpr::NUMBER(num) => Ok(DirectValueExpression::NUMBER(*num)),
        AstExpr::VARIABLE_CALL(var) => Ok(DirectValueExpression::IDENTIFIER(Identifier::new(
            var.to_string(),
            None,
        ))),
        e => {
            println!(
                "Unknown expression in compile_ast_direct_value_expr: {}",
                e.to_string()
            );
            Err(format!(
                "Unknown expression in compile_ast_direct_value_expr: {}",
                e.to_string()
            ))
        }
    }
}

pub fn compile_ast_indirect_value_expr(
    ast: &AstExpr,
    module: &mut Module,
    current_block: &mut Block,
) -> Result<IndirectValueExpression, String> {
    match ast {
        AstExpr::BINOP(binop) => match compile_binop(binop, module, current_block) {
            Ok(b) => Ok(IndirectValueExpression::BINOP(b)),
            Err(e) => Err(e),
        },
        e => {
            println!(
                "Unknown expression in compile_ast_indirect_value_expr {}",
                e.to_string()
            );
            Err(format!(
                "Unknown expression in compile_ast_indirect_value_expr {}",
                e.to_string()
            ))
        }
    }
}

pub fn convert_ast_to_direct_value(
    ast: &AstExpr,
    module: &mut Module,
    current_block: &mut Block,
) -> Result<DirectValueExpression, String> {
    if let Ok(value) = compile_ast_indirect_value_expr(ast, module, current_block) {
        let name = "ret_value".to_string();
        let identifier = module
            .builder
            .function
            .block
            .identifier
            .build(name.clone(), Some(value));

        current_block.add_expression(BlockExpression::IDENTIFIER(identifier));
        return Ok(DirectValueExpression::IDENTIFIER(Identifier::new(
            name, None,
        )));
    } else if let Ok(value) = compile_ast_direct_value_expr(ast, module, current_block) {
        return Ok(value);
    } else {
        return Err("create_llvm_ret Unknown expression".to_string());
    }
}

pub fn compile_ast_expr(
    ast: &AstExpr,
    module: &mut Module,
    current_block: &mut Block,
) -> Result<BlockExpression, String> {
    match ast {
        AstExpr::RETURN(ret) => match create_llvm_ret(ret, module, current_block) {
            Ok(r) => Ok(BlockExpression::TERMINATOR(Terminator::RETURN(r))),
            Err(e) => Err(e),
        },
        _ => Err("Unknown expression".to_string()),
    }
}
