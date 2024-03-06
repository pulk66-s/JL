#[derive(Debug, PartialEq, Clone)]
pub enum Binop {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Gt,
    Ge,
    Lt,
    Le,
}

#[derive(Debug, Clone)]
pub struct AstBinop {
    pub op: Binop,
    pub values: Box<Vec<AstNode>>,
}

#[derive(Debug, Clone)]
pub enum AstType {
    Number,
}

#[derive(Debug, Clone)]
pub struct AstFunctionDeclArg {
    pub arg_type: AstType,
    pub name: Box<AstNode>,
}

#[derive(Debug, Clone)]
pub struct AstFunctionDecl {
    pub name: Box<AstNode>,
    pub args: Vec<AstFunctionDeclArg>,
    pub return_type: AstType,
    pub body: Vec<AstNode>,
}

#[derive(Debug, Clone)]
pub enum AstFunctionLine {
    Line(Box<AstNode>),
    Return(Box<AstNode>),
}

#[derive(Debug, Clone)]
pub struct AstFunctionCall {
    pub name: Box<AstNode>,
    pub args: Vec<AstNode>,
}

#[derive(Debug, Clone)]
pub struct AstVariableDecl {
    pub var_type: AstType,
    pub name: Box<AstNode>,
    pub value: Box<AstNode>,
}

#[derive(Debug, Clone)]
pub struct AstCondition {
    pub condition: Box<AstNode>,
    pub body: Vec<AstNode>,
    pub else_condition: Option<Box<AstCondition>>,
}

#[derive(Debug, Clone)]
pub enum AstNode {
    Binop(AstBinop),
    Number(f64),
    Identifier(String),
    FunctionDecl(AstFunctionDecl),
    FunctionCall(AstFunctionCall),
    VariableDecl(AstVariableDecl),
    Condition(AstCondition)
}
