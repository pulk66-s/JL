use self::{
    function::{block::expressions::binop::BinOpBuilder, FunctionBuilder},
    types::TypesBuilder,
};

pub mod function;
pub mod types;

#[derive(Clone)]
pub struct Builder {
    pub function: FunctionBuilder,
    pub types: TypesBuilder,
    pub binop: BinOpBuilder,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            function: FunctionBuilder::new(),
            types: TypesBuilder::new(),
            binop: BinOpBuilder::new(),
        }
    }
}
