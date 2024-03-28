use crate::{
    ast::returnstmt::AstReturn,
    comp::llvm::{
        builder::function::block::expressions::{
            binop::BinOp, terminator::Terminator, BlockExpression, ValueExpression,
        },
        module::Module,
    },
};

use super::expr::create_expr;

fn create_ret_binop(binop: BinOp, mut module: Module) -> Result<(Terminator, Module), String> {
    let mut current_function = match module.current_function() {
        Some(f) => f,
        None => return Err("create_return_expr no function".to_string()),
    };
    let mut current_block = match current_function.current_block() {
        Some(f) => f,
        None => return Err("create_return_expr no block".to_string()),
    };
    let identifier = module
        .builder
        .function
        .block
        .identifier
        .build("ret".to_string(), ValueExpression::BINOP(binop));

    current_block.add_expression(BlockExpression::IDENTIFIER(identifier.clone()));
    current_function.update_current_block(current_block);
    module.update_current_function(current_function);
    Ok((
        module
            .builder
            .function
            .terminator
            .create_return(ValueExpression::IDENTIFIER(identifier), None),
        module,
    ))
}

fn create_ret_value(
    value: ValueExpression,
    module: Module,
) -> Result<(Terminator, Module), String> {
    Ok((
        module
            .builder
            .function
            .terminator
            .create_return(value, None),
        module,
    ))
}

pub fn create_return_expr(ret: AstReturn, module: Module) -> Result<(Terminator, Module), String> {
    match create_expr(&ret.value, module) {
        Err(e) => Err(e),
        Ok((BlockExpression::VALUE(ValueExpression::BINOP(binop)), module)) => {
            create_ret_binop(binop, module)
        }
        Ok((BlockExpression::VALUE(value), module)) => create_ret_value(value, module),
        _ => Err(format!("Unknown expression in create_return_expr")),
    }
}
