#[derive(Debug)]
pub enum Binop {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub struct AstBinop {
    pub left: Box<AstNode>,
    pub op: Binop,
    pub right: Box<AstNode>,
}

#[derive(Debug)]
pub enum AstNode {
    Binop(AstBinop),
    Number(f64)
}
