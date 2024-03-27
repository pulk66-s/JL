use super::AstExpr;

#[derive(Debug, Clone)]
pub struct AstBinop {
    pub left: Box<AstExpr>,
    pub right: Box<AstExpr>,
    pub op: String,
}

impl AstBinop {
    pub fn new(left: Box<AstExpr>, right: Box<AstExpr>, op: String) -> Self {
        AstBinop { left, right, op }
    }
}
