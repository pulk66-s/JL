use crate::comp::llvm::{
    builder::function::block::{expressions::DirectValueExpression, Block},
    llvm_object::LlvmObject,
};

#[derive(Debug, Clone)]
pub struct Br {
    pub condition: DirectValueExpression,
    pub tr: Block,
    pub fal: Block,
}

impl Br {
    pub fn new(condition: DirectValueExpression, tr: Block, fal: Block) -> Self {
        Self { condition, tr, fal }
    }
}

impl LlvmObject for Br {
    fn to_llvm_ir(&self) -> String {
        format!(
            "br i1 {}, label %{}, label %{}",
            self.condition.to_llvm_ir(),
            self.tr.label,
            self.fal.label
        )
    }
}
