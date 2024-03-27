use std::{borrow::Borrow, error};

use crate::ast::{
    binop::AstBinop, expr, returnstmt::AstReturn, variable::AstVariableDecl, Ast, AstExpr,
};

pub mod builder;
pub mod llvm_object;
pub mod module;

use module::Module;

use self::{
    builder::{
        function::{
            block::{
                expressions::{
                    binop::{BinOp, BinOpType},
                    identifier,
                    terminator::Terminator,
                    BlockExpression, ValueExpression,
                },
                Block,
            },
            param::FunctionParam,
        },
        types::Type,
    },
    llvm_object::LlvmObject,
};

fn create_type(name: &str) -> Option<Type> {
    match name {
        "int" => Some(Type::Int32),
        _ => None,
    }
}

fn create_param(param: AstVariableDecl) -> Option<FunctionParam> {
    match create_type(&param.vtype) {
        Some(t) => Some(FunctionParam::new(param.name, t)),
        None => None,
    }
}

fn create_params(params: Vec<AstVariableDecl>) -> Option<Vec<FunctionParam>> {
    let mut result = Vec::new();

    for param in params {
        match create_param(param) {
            Some(p) => result.push(p),
            None => return None,
        }
    }
    Some(result)
}

fn create_expr(expr: &AstExpr, module: Module) -> Result<(BlockExpression, Module), String> {
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
        e => Err(format!("Unknown expression {}", e.to_string())),
    }
}

fn create_binop_expr(binop: AstBinop, module: Module) -> Result<(BinOp, Module), String> {
    match create_expr(&binop.left, module) {
        Ok((BlockExpression::VALUE(lhs), module)) => match create_expr(&binop.right, module) {
            Ok((BlockExpression::VALUE(rhs), module)) => match binop.op.as_str() {
                "+" => Ok((
                    module
                        .builder
                        .binop
                        .create_binop(BinOpType::ADD, lhs, rhs, None),
                    module,
                )),
                _ => Err("Unknown operator".to_string()),
            },
            Err(e) => Err(e),
            _ => Err("Error in binop".to_string()),
        },
        Err(e) => Err(e),
        _ => Err("Unknown binop".to_string()),
    }
    // match (
    //     create_expr(&binop.left, module),
    //     create_expr(&binop.right, module),
    // ) {
    //     (Ok(BlockExpression::VALUE(lhs)), Ok(BlockExpression::VALUE(rhs))) => {
    //         match binop.op.as_str() {
    //             "+" => Ok(module
    //                 .builder
    //                 .binop
    //                 .create_binop(BinOpType::ADD, lhs, rhs, None)),
    //             _ => Err("Unknown operator".to_string()),
    //         }
    //     }
    //     _ => return Err("Error in binop".to_string()),
    // }
}

fn create_return_expr(ret: AstReturn, module: Module) -> Result<(Terminator, Module), String> {
    let mut current_function = match module.current_function() {
        Some(f) => f,
        None => return Err("create_return_expr no function".to_string()),
    };
    let mut current_block = match current_function.current_block() {
        Some(f) => f,
        None => return Err("create_return_expr no block".to_string()),
    };

    match create_expr(&ret.value, module) {
        Err(e) => Err(e),
        Ok((BlockExpression::VALUE(ValueExpression::BINOP(binop)), mut module)) => {
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
        Ok((BlockExpression::VALUE(binop), module)) => Ok((
            module
                .builder
                .function
                .terminator
                .create_return(binop, None),
            module,
        )),
        _ => Err(format!("Unknown expression in create_return_expr")),
    }
}

fn create_function_body(ast: Vec<Box<AstExpr>>, module: Module) -> Result<Module, String> {
    let mut module = module.clone();
    let start_block = module.builder.function.block.build_block(None, vec![]);
    let mut current_function = match module.current_function() {
        Some(f) => f,
        None => return Err("No function".to_string()),
    };

    current_function.add_block(start_block.clone());
    for expr in ast {
        module.update_current_function(current_function.clone());
        match create_expr(expr.borrow(), module) {
            Ok((expr, new_module)) => {
                module = new_module;
                match expr {
                    BlockExpression::TERMINATOR(_) => {
                        let mut current_function = match module.current_function() {
                            Some(f) => f,
                            None => return Err("No function".to_string()),
                        };
                        let mut current_block = match current_function.current_block() {
                            Some(b) => b,
                            None => return Err("No block create_function_body 1".to_string()),
                        };
                        current_block.add_expression(expr.clone());
                        current_function.update_current_block(current_block);
                        module.update_current_function(current_function.clone());

                        let new_block = module.builder.function.block.build_block(None, vec![]);

                        current_function.add_block(new_block);
                    }
                    _ => {
                        let mut current_block = match current_function.current_block() {
                            Some(b) => b,
                            None => return Err("No block create_function_body 1".to_string()),
                        };
                        current_block.expressions.push(expr);
                    }
                };
            }
            Err(e) => return Err(e),
        }
    }
    Ok(module)
}

pub fn compile_with_llvm(ast: Ast) -> Result<String, String> {
    let mut module = Module::new();

    for function in ast.functions {
        module
            .functions_definitions
            .push(module.builder.function.define(
                function.name,
                match create_type(&function.return_type) {
                    Some(t) => t,
                    None => return Err(format!("Unknown type: {}", function.return_type)),
                },
                match create_params(function.args) {
                    Some(p) => p,
                    None => return Err("Unknown type".to_string()),
                },
                vec![],
            ));

        module = match create_function_body(function.body, module) {
            Ok(blocks) => blocks,
            Err(e) => return Err(format!("Error in function body: {}", e)),
        };
    }
    Ok(module.to_llvm_ir())
}
