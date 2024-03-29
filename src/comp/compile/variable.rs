use crate::comp::llvm::builder::function::block::expressions::identifier::Identifier;

pub fn create_llvm_variable_call(ast: String) -> Identifier {
    Identifier::new(ast, None)
}
