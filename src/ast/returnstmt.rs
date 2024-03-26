use super::AstExpr;

#[derive(Debug)]
pub struct AstReturn {
    pub value: Box<AstExpr>,
}

impl AstReturn {
    pub fn new(value: Box<AstExpr>) -> Self {
        AstReturn { value }
    }
}