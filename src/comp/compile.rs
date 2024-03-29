pub mod binop;
pub mod expr;
pub mod function;
pub mod ret;
pub mod variable;

use crate::ast::Ast;

use self::function::create_llvm_from_ast_function;

use super::llvm::{llvm_object::LlvmObject, module::Module};

pub fn compile_with_llvm(ast: Ast) -> Result<String, String> {
    let mut module = Module::new();
    let mut functions = vec![];

    for func in ast.functions {
        match create_llvm_from_ast_function(func, &mut module) {
            Ok(f) => functions.push(f),
            Err(e) => return Err(e),
        }
    }
    Ok(functions
        .iter()
        .map(|f| f.to_llvm_ir())
        .collect::<Vec<String>>()
        .join("\n"))
}
