#[derive(Debug, PartialEq, Clone)]
pub enum Binop {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
pub struct AstBinop {
    pub op: Binop,
    pub values: Box<Vec<AstNode>>,
}

#[derive(Debug, Clone)]
pub enum AstNode {
    Binop(AstBinop),
    Number(f64)
}
