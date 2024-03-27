use self::{block::{Block, BlockBuilder}, define::FunctionDefinition, param::FunctionParam};

use super::types::Type;

pub mod block;
pub mod define;
pub mod param;

pub struct FunctionBuilder {
    pub block: BlockBuilder
}

impl FunctionBuilder {
    pub fn new() -> Self {
        Self {
            block: BlockBuilder::new()
        }
    }

    pub fn define(
        &self,
        name: String,
        return_type: Type,
        params: Vec<FunctionParam>,
        body: Vec<Block>,
    ) -> FunctionDefinition {
        FunctionDefinition::new(name, return_type, params, body)
    }
}
