use crate::{
    ast::{func::definition::AstFuncDef, variable::AstVariableDecl, AstExpr},
    comp::llvm::{
        builder::{
            function::{
                block::{expressions::BlockExpression, Block},
                define::FunctionDefinition,
                param::FunctionParam,
            },
            types::Type,
        },
        module::Module,
    },
};

use super::expr::compile_ast_expr;

fn collect_params(
    args: Vec<AstVariableDecl>,
    module: &mut Module,
) -> Result<Vec<FunctionParam>, String> {
    let params = args
        .iter()
        .map(|arg| {
            let arg_type = match module.builder.types.create(arg.vtype.clone()) {
                Some(x) => x,
                None => return Err(format!("Unknown type: {}", arg.vtype)),
            };
            Ok((arg.name.clone(), arg_type))
        })
        .collect::<Result<Vec<(String, Type)>, String>>();

    match params {
        Ok(r) => Ok(r
            .iter()
            .map(|(name, ty)| FunctionParam::new(name.clone(), ty.clone()))
            .collect()),
        Err(e) => Err(e),
    }
}

fn generate_blocks(body: Vec<Box<AstExpr>>, module: &mut Module) -> Result<Vec<Block>, String> {
    let mut blocks = vec![];
    let mut current_block = module
        .builder
        .function
        .block
        .build_block(Some("start".to_string()), vec![]);
    let mut block_index = 0;

    for expr in body {
        let llvm_expr = match compile_ast_expr(&*expr, module, &mut current_block) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        match llvm_expr {
            BlockExpression::TERMINATOR(term) => {
                current_block.add_expression(BlockExpression::TERMINATOR(term));
                blocks.push(current_block.clone());
                current_block = module
                    .builder
                    .function
                    .block
                    .build_block(Some(format!("block_{}", block_index)), vec![]);
                block_index += 1;
            }
            _ => current_block.add_expression(llvm_expr),
        }
    }
    Ok(blocks)
}

pub fn create_llvm_from_ast_function(
    func: AstFuncDef,
    module: &mut Module,
) -> Result<FunctionDefinition, String> {
    let name = func.name;
    let return_type = match module.builder.types.create(func.return_type.clone()) {
        Some(x) => x,
        None => return Err(format!("Unknown type: {}", func.return_type)),
    };
    let params = match collect_params(func.args, module) {
        Ok(r) => r,
        Err(e) => return Err(e),
    };
    let body = match generate_blocks(func.body, module) {
        Ok(r) => r,
        Err(e) => return Err(e),
    };
    let def = module
        .builder
        .function
        .define(name, return_type, params, body);

    Ok(def)
}
