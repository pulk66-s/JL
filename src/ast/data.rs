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
pub enum AstType {
    Number
}

#[derive(Debug, Clone)]
pub struct AstFunctionDeclArg {
    pub arg_type: AstType,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct AstFunctionDecl {
    pub name: String,
    pub args: Vec<AstFunctionDeclArg>,
    pub return_type: AstType,
}

#[derive(Debug, Clone)]
pub enum AstNode {
    Binop(AstBinop),
    Number(f64),
    FunctionDecl(AstFunctionDecl)
}
