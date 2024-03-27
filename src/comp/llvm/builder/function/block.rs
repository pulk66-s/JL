pub mod expressions;

use crate::comp::llvm::llvm_object::LlvmObject;

use self::expressions::{identifier::IdentifierBuilder, BlockExpression};

#[derive(Clone)]
pub struct BlockBuilder {
    pub identifier: IdentifierBuilder,
}

impl BlockBuilder {
    pub fn new() -> Self {
        Self {
            identifier: IdentifierBuilder::new(),
        }
    }

    pub fn build_block(&self, label: Option<String>, expressions: Vec<BlockExpression>) -> Block {
        match label {
            Some(label) => Block::new(label, expressions),
            None => {
                let block = Block::new("start".to_string(), expressions);
                block
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub label: String,
    pub expressions: Vec<BlockExpression>,
}

impl Block {
    pub fn new(label: String, expressions: Vec<BlockExpression>) -> Self {
        Self { label, expressions }
    }

    pub fn add_expression(&mut self, expression: BlockExpression) {
        self.expressions.push(expression);
    }
}

impl LlvmObject for Block {
    fn to_llvm_ir(&self) -> String {
        format!(
            "{}:\n\t{}",
            self.label,
            self.expressions
                .iter()
                .map(|f| f.to_llvm_ir())
                .collect::<Vec<String>>()
                .join("\n\t")
        )
    }
}
