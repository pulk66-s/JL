use self::{
    binop::AstBinop, condition::ConditionAst, func::definition::AstFuncDef, returnstmt::AstReturn, variable::AstVariableDecl
};

pub mod binop;
pub mod expr;
pub mod func;
pub mod parse;
pub mod returnstmt;
pub mod variable;
pub mod condition;

#[derive(Debug, Clone)]
pub enum AstExpr {
    VARIABLE_DECL(AstVariableDecl),
    RETURN(AstReturn),
    BINOP(AstBinop),
    VARIABLE_CALL(String),
    NUMBER(i64),
    CONDITION(ConditionAst)
}

impl AstExpr {
    pub fn to_string(&self) -> String {
        match self {
            AstExpr::VARIABLE_DECL(decl) => format!("Variable declaration: {:?}", decl),
            AstExpr::RETURN(ret) => format!("Return: {:?}", ret),
            AstExpr::BINOP(binop) => format!("Binop: {:?}", binop),
            AstExpr::VARIABLE_CALL(name) => format!("Variable call: {}", name),
            AstExpr::NUMBER(n) => format!("Number: {}", n),
            AstExpr::CONDITION(c) => format!("Condition: {}", c.to_string())
        }
    }
}

#[derive(Debug)]
pub struct Ast {
    pub functions: Vec<AstFuncDef>,
}

impl Ast {
    pub fn new() -> Self {
        Ast { functions: vec![] }
    }
}
