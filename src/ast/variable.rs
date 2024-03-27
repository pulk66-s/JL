use super::AstExpr;

#[derive(Debug, Clone)]
pub struct AstVariableDecl {
    pub name: String,
    pub vtype: String
}
