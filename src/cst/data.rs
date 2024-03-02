#[derive(Debug)]
pub enum CstAtom {
    NUMBER(f64),
    IDENTIFIER(String),
    CHAR(char),
}

#[derive(Debug)]
pub enum CstNode {
    ATOM(CstAtom),
    BINOP(CstBinop),
}

#[derive(Debug)]
pub struct CstBinop {
    pub op: char,
    pub values: Box<Vec<CstNode>>
}
