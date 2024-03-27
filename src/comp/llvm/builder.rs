use self::{function::FunctionBuilder, types::TypesBuilder};

pub mod function;
pub mod types;

pub struct Builder {
    pub function: FunctionBuilder,
    pub types: TypesBuilder
}

impl Builder {
    pub fn new() -> Self {
        Self {
            function: FunctionBuilder::new(),
            types: TypesBuilder::new()
        }
    }
}
