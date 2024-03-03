#[derive(Debug)]
pub enum CstAtom {
    NUMBER(f64),
    KEYWORD(String),
    CHAR(char),
    IDENTIFIER(String)
}

#[derive(Debug)]
pub enum CstNode {
    ATOM(CstAtom),
    BINOP(CstBinop),
    FUNCTION_DECL(CstFunctionDecl),
    FUNCTION_CALL(CstFunctionCall),
    FUNCTION_LINE_EXPR(CstFunctionLineExpr),
    VARIABLE_DECL(CstVariableDecl)
}

#[derive(Debug)]
pub struct CstVariableDecl {
    pub keyword: CstAtom,
    pub var_type: CstAtom,
    pub name: CstAtom,
    pub equal: CstAtom,
    pub value: Box<CstNode>
}

#[derive(Debug)]
pub struct CstFunctionCallArgChain {
    pub arg: Box<CstNode>,
    pub comma: CstAtom
}

#[derive(Debug)]
pub struct CstFunctionCallArgs {
    pub arg_chain: Vec<CstFunctionCallArgChain>,
    pub last_arg: Box<CstNode>
}

#[derive(Debug)]
pub struct CstFunctionCall {
    pub name: CstAtom,
    pub open_par: CstAtom,
    pub args: Option<CstFunctionCallArgs>,
    pub close_par: CstAtom,
}

#[derive(Debug)]
pub struct CstFunctionDeclArg {
    pub arg_type: CstAtom,
    pub name: CstAtom
}

#[derive(Debug)]
pub struct CstFunctionChainArg {
    pub arg_type: CstAtom,
    pub name: CstAtom,
    pub comma: CstAtom
}

#[derive(Debug)]
pub struct CstFunctionDeclArgs {
    pub arg_chains: Vec<CstFunctionChainArg>,
    pub last_arg: CstFunctionDeclArg
}

#[derive(Debug)]
pub struct CstLine {
    pub expr: Box<CstNode>,
}

#[derive(Debug)]
pub struct CstReturnExpr {
    pub keyword: CstAtom,
    pub value: Box<CstNode>
}

#[derive(Debug)]
pub enum CstFunctionLineExpr {
    LINE(CstLine),
    RETURN(CstReturnExpr)
}

#[derive(Debug)]
pub struct CstFunctionLine {
    pub expr: CstFunctionLineExpr,
    pub endline: CstAtom
}

#[derive(Debug)]
pub struct CstFunctionDecl {
    pub keyword: CstAtom,
    pub open_par: CstAtom,
    pub name: CstAtom,
    pub args: Option<CstFunctionDeclArgs>,
    pub close_par: CstAtom,
    pub return_arrow: CstAtom,
    pub return_type: CstAtom,
    pub open_brace: CstAtom,
    pub body: Vec<CstFunctionLine>,
    pub close_brace: CstAtom
}

#[derive(Debug)]
pub struct CstBinop {
    pub op: CstAtom,
    pub values: Box<Vec<CstNode>>
}
