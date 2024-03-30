use crate::{
    ast::{condition::ConditionAst, AstExpr},
    comp::llvm::{
        builder::function::{
            block::{expressions::terminator::Terminator, Block},
            define::FunctionDefinition,
        },
        llvm_object::LlvmObject,
        module::Module,
    },
};

use super::expr::{compile_ast_expr, convert_ast_to_direct_value};

fn create_true_block(
    true_expr: &Vec<Box<AstExpr>>,
    module: &mut Module,
    current_block: &mut Block,
    current_function: &mut FunctionDefinition,
) -> Result<Block, String> {
    let mut block = module
        .builder
        .function
        .block
        .build_block(Some("cond_true".to_string()), vec![]);

    for expr in true_expr {
        match compile_ast_expr(&expr, module, current_block, current_function) {
            Ok(expr) => block.add_expression(expr),
            Err(e) => return Err(e),
        }
    }
    Ok(block)
}

fn create_false_block(
    false_expr: &Option<Vec<Box<AstExpr>>>,
    module: &mut Module,
    current_block: &mut Block,
    current_function: &mut FunctionDefinition,
) -> Result<Block, String> {
    if let Some(exprs) = false_expr {
        let mut block = module
            .builder
            .function
            .block
            .build_block(Some("cond_false".to_string()), vec![]);

        for expr in exprs {
            match compile_ast_expr(&expr, module, current_block, current_function) {
                Ok(expr) => block.add_expression(expr),
                Err(e) => return Err(e),
            }
        }
        return Ok(block);
    } else {
        return Ok(current_block.clone());
    }
}

pub fn create_llvm_condition(
    ast: &ConditionAst,
    module: &mut Module,
    current_block: &mut Block,
    current_function: &mut FunctionDefinition,
) -> Result<Terminator, String> {
    let cond = match convert_ast_to_direct_value(&ast.condition, module, current_block) {
        Ok(cond) => cond,
        Err(e) => {
            return Err(e);
        }
    };
    let true_block =
        match create_true_block(&ast.true_expr, module, current_block, current_function) {
            Ok(b) => b,
            Err(e) => return Err(e),
        };
    let false_block =
        match create_false_block(&ast.false_expr, module, current_block, current_function) {
            Ok(b) => b,
            Err(e) => return Err(e),
        };

    current_function.add_block(true_block.clone());
    current_function.add_block(false_block.clone());
    Ok(module
        .builder
        .function
        .terminator
        .create_br(cond, true_block, false_block))
}
