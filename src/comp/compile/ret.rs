use crate::{
    ast::returnstmt::AstReturn,
    comp::llvm::{
        builder::function::block::{expressions::terminator::return_term::Return, Block},
        module::Module,
    },
};

use super::expr::convert_ast_to_direct_value;

pub fn create_llvm_ret(
    ast: &AstReturn,
    module: &mut Module,
    current_block: &mut Block,
) -> Result<Return, String> {
    let value = match convert_ast_to_direct_value(&*ast.value, module, current_block) {
        Ok(r) => r,
        Err(e) => return Err(e),
        _ => return Err("create_llvm_ret Unknown expression".to_string()),
    };

    Ok(Return::new(value, None))
}
