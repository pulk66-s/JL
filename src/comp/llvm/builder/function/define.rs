use crate::comp::llvm::{builder::types::Type, llvm_object::LlvmObject};

use super::{block::Block, param::FunctionParam};

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub name: String,
    pub return_type: Type,
    pub params: Vec<FunctionParam>,
    pub body: Vec<Block>
}

impl FunctionDefinition {
    pub fn new(
        name: String,
        return_type: Type,
        params: Vec<FunctionParam>,
        body: Vec<Block>,
    ) -> Self {
        Self {
            name,
            return_type,
            params,
            body,
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.body.push(block);
    }
}

impl LlvmObject for FunctionDefinition {
    fn to_llvm_ir(&self) -> String {
        format!(
            "define {} @{}({}) {{\n{}}}",
            self.return_type.to_llvm_ir(),
            self.name,
            self.params
                .iter()
                .map(|param| param.to_llvm_ir())
                .collect::<Vec<String>>()
                .join(", "),
            self.body
                .iter()
                .map(|param| param.to_llvm_ir())
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}
