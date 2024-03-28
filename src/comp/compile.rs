pub mod binop;
pub mod expr;
pub mod function;
pub mod ret;

use crate::ast::{variable::AstVariableDecl, Ast};

use self::function::create_function_body;

use super::llvm::{
    builder::{function::param::FunctionParam, types::Type},
    llvm_object::LlvmObject,
};

use super::llvm::module::Module;

fn create_type(name: &str) -> Option<Type> {
    match name {
        "int" => Some(Type::Int32),
        "bool" => Some(Type::Bool),
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
