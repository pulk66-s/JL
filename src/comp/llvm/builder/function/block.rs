pub mod expressions;

use crate::comp::llvm::llvm_object::LlvmObject;

use self::expressions::BlockExpression;

pub struct BlockBuilder {
    last_block_index: usize,
}

impl BlockBuilder {
    pub fn new() -> Self {
        Self {
            last_block_index: 0,
        }
    }

    pub fn build_block(
        &mut self,
        label: Option<String>,
        expressions: Vec<BlockExpression>,
    ) -> Block {
        match label {
            Some(label) => Block::new(label, expressions),
            None => {
                let block = Block::new(self.last_block_index.to_string(), expressions);
                self.last_block_index += 1;
                block
            }
        }
    }
}

pub struct Block {
    pub label: String,
    pub expressions: Vec<BlockExpression>,
}

impl Block {
    pub fn new(label: String, expressions: Vec<BlockExpression>) -> Self {
        Self { label, expressions }
    }
}

impl LlvmObject for Block {
    fn to_llvm_ir(&self) -> String {
        format!("{}:", self.label)
    }
}
