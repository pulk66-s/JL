use self::{func::definition::AstFuncDef, variable::AstVariableDecl};

pub mod func;
pub mod variable;
pub mod parse;

pub enum AstExpr {
    FUNC_DEF(AstFuncDef),
    VARIABLE_DECL(AstVariableDecl)
}

#[derive(Debug)]
pub struct Ast {
    pub functions: Vec<AstFuncDef>
}

impl Ast {
    pub fn new() -> Self {
        Ast {
            functions: vec![]
        }
    }
}
