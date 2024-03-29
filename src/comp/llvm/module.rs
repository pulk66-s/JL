use super::{
    builder::{function::define::FunctionDefinition, Builder},
    llvm_object::LlvmObject,
};

#[derive(Clone)]
pub struct Module {
    pub builder: Builder,
    pub functions_definitions: Vec<FunctionDefinition>,
}

impl Module {
    pub fn new() -> Self {
        Self {
            builder: Builder::new(),
            functions_definitions: Vec::new(),
        }
    }
}

impl LlvmObject for Module {
    fn to_llvm_ir(&self) -> String {
        format!(
            "{}",
            self.functions_definitions
                .iter()
                .map(|f| f.to_llvm_ir())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
