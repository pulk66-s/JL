use crate::ast::{
    parse::keyword::{get_identifier, match_keyword, Tokens},
    variable::AstVariableDecl,
    AstExpr,
};

use super::{body::fetch_fn_body, params::get_fn_params};

#[derive(Debug)]
pub struct AstFuncDef {
    pub name: String,
    pub args: Vec<AstVariableDecl>,
    pub return_type: String,
    pub body: Vec<Box<AstExpr>>,
}

pub fn create_ast_func_def(tokens: &mut Tokens) -> Result<AstFuncDef, String> {
    if match_keyword(tokens, "fn") == false {
        return Err("Expected 'fn' keyword".to_string());
    }

    tokens.next();

    let fn_name = match get_identifier(tokens) {
        Ok(r) => r,
        Err(e) => return Err(e),
    };

    tokens.next();
    tokens.next();
    tokens.next();

    let fn_params = match get_fn_params(tokens) {
        Ok(r) => r,
        Err(e) => return Err(e),
    };

    tokens.next();
    tokens.next();
    tokens.next();
    tokens.next();
    tokens.next();

    let fn_type = match get_identifier(tokens) {
        Ok(r) => r,
        Err(e) => return Err(e),
    };

    tokens.next();
    tokens.next();
    tokens.next();

    let body = match fetch_fn_body(tokens) {
        Ok(r) => r,
        Err(e) => return Err(e),
    };

    Ok(AstFuncDef {
        name: fn_name,
        args: fn_params,
        return_type: fn_type,
        body: body.into_iter().map(|e| Box::new(e)).collect(),
    })
}

pub fn create_ast_func_defs(tokens: &mut Tokens) -> Result<Vec<AstFuncDef>, String> {
    let mut funcs = vec![];

    loop {
        match create_ast_func_def(tokens) {
            Ok(f) => funcs.push(f),
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }
    Ok(funcs)
}
