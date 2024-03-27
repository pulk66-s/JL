use crate::ast::{returnstmt::AstReturn, variable::AstVariableDecl, Ast, AstExpr};

pub mod builder;
pub mod llvm_object;
pub mod module;

use module::Module;

use self::{builder::{function::{block::Block, param::FunctionParam}, types::Type}, llvm_object::LlvmObject};

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

fn create_return_expr(ret: AstReturn, module: Module) {
    
}

fn create_function_body(ast: Vec<Box<AstExpr>>, module: Module) -> Result<Vec<Block>, String> {
    let mut result = Vec::new();

    for expr in ast {
        match *expr {
            AstExpr::RETURN(ret) => create_return_expr(ret),
            _ => return Err("Unknown expression".to_string()),
        };
    }
    Ok(result)
}

pub fn compile_with_llvm(ast: Ast) -> Result<String, String> {
    let mut module = Module::new();
    let builder = &module.builder;

    for function in ast.functions {
        let def = builder.function.define(
            function.name,
            match create_type(&function.return_type) {
                Some(t) => t,
                None => return Err(format!("Unknown type: {}", function.return_type)),
            },
            match create_params(function.args) {
                Some(p) => p,
                None => return Err("Unknown type".to_string()),
            },
            match create_function_body(function.body, module) {
                Ok(blocks) => blocks,
                Err(e) => return Err(format!("Error in function body: {}", e)),
            }
        );

        module.functions_definitions.push(def)
    }
    Ok(module.to_llvm_ir())
}
