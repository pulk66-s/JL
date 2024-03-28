use crate::{
    ast::AstExpr,
    comp::llvm::{builder::function::block::expressions::BlockExpression, module::Module},
};

use super::expr::create_expr;

fn understand_expr(result: Result<(BlockExpression, Module), String>) -> Result<Module, String> {
    match result {
        Ok((expr, mut module)) => {
            let mut current_function = match module.current_function() {
                Some(f) => f,
                None => return Err("No function".to_string()),
            };
            let mut current_block = match current_function.current_block() {
                Some(b) => b,
                None => return Err("No block create_function_body 1".to_string()),
            };

            match expr {
                BlockExpression::TERMINATOR(_) => {
                    current_block.add_expression(expr.clone());
                    current_function.update_current_block(current_block);
                    module.update_current_function(current_function.clone());

                    let new_block = module.builder.function.block.build_block(None, vec![]);

                    current_function.add_block(new_block);
                }
                _ => current_block.expressions.push(expr),
            };
            Ok(module)
        }
        Err(e) => return Err(e),
    }
}

pub fn create_function_body(ast: Vec<Box<AstExpr>>, module: Module) -> Result<Module, String> {
    let mut module = module.clone();
    let start_block = module.builder.function.block.build_block(None, vec![]);
    let mut current_function = match module.current_function() {
        Some(f) => f,
        None => return Err("No function".to_string()),
    };

    current_function.add_block(start_block.clone());
    for expr in ast {
        module.update_current_function(current_function.clone());
        module = match understand_expr(create_expr(&*expr, module)) {
            Ok(m) => m,
            Err(e) => return Err(e),
        };
    }
    Ok(module)
}
