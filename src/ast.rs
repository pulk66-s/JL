use self::{
    binop::AstBinop, func::definition::AstFuncDef, returnstmt::AstReturn, variable::AstVariableDecl,
};

pub mod binop;
pub mod expr;
pub mod func;
pub mod parse;
pub mod returnstmt;
pub mod variable;

#[derive(Debug)]
pub enum AstExpr {
    FUNC_DEF(AstFuncDef),
    VARIABLE_DECL(AstVariableDecl),
    RETURN(AstReturn),
    BINOP(AstBinop),
    VARIABLE_CALL(String),
    NUMBER(i64),
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
