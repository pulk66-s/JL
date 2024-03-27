use either::Either::{Left, Right};

use crate::{
    ast::{
        expr::create_ast_expr,
        parse::keyword::{get_deep_token, get_identifier, match_keyword, TokenType, Tokens},
        variable::AstVariableDecl,
        AstExpr,
    },
    cst::parser::ParserDataType,
};

#[derive(Debug)]
pub struct AstFuncDef {
    pub name: String,
    pub args: Vec<AstVariableDecl>,
    pub return_type: String,
    pub body: Vec<Box<AstExpr>>,
}

fn get_fn_repeated_params(tokens: &mut Tokens) -> Vec<AstVariableDecl> {
    let repeat = match tokens.next() {
        Some(t) => match t {
            Left(t) => match t {
                ParserDataType::Repeat(r) => r,
                _ => return vec![],
            },
            Right(_) => return vec![],
        },
        None => return vec![],
    };
    let mut params = vec![];
    let mut tokens = Tokens::new(ParserDataType::Repeat(repeat));

    loop {
        let mut arg_token = Tokens::new(match tokens.next() {
            Some(Left(e)) => e,
            _ => break,
        });
        let p_type = match get_deep_token(&mut arg_token) {
            Some(TokenType::String(s)) => s,
            _ => break,
        };

        arg_token.next();

        let p_identifier = match get_deep_token(&mut arg_token) {
            Some(TokenType::String(s)) => s,
            _ => break,
        };

        params.push(AstVariableDecl {
            name: p_identifier,
            vtype: p_type,
        })
    }
    params
}

fn get_fn_last_param(tokens: &mut Tokens) -> Result<Option<AstVariableDecl>, String> {
    let p_type = match get_deep_token(tokens) {
        Some(TokenType::String(s)) => match s.as_str() {
            "None" => return Ok(None),
            n => n.to_string(),
        },
        _ => return Err("Expected a string in the last param type".to_string()),
    };

    tokens.next();

    let p_identifier = match get_deep_token(tokens) {
        Some(TokenType::String(s)) => s,
        _ => return Err("Expected a string in last param identiifer".to_string()),
    };

    Ok(Some(AstVariableDecl {
        name: p_identifier,
        vtype: p_type,
    }))
}

fn get_fn_params(tokens: &mut Tokens) -> Result<Vec<AstVariableDecl>, String> {
    let params = get_fn_repeated_params(tokens);
    let last = match get_fn_last_param(tokens) {
        Ok(r) => r,
        Err(e) => return Err(format!("err while fetching fn def last param: {}", e)),
    };
    let mut vars = params;

    if let Some(r) = last {
        vars.push(r);
    }
    Ok(vars)
}

fn fetch_fn_body(tokens: &mut Tokens) -> Result<Vec<AstExpr>, String> {
    let mut body_tokens = Tokens::new(match tokens.next() {
        Some(Left(ParserDataType::Repeat(e))) => ParserDataType::Repeat(e),
        _ => return Err("Expected a repeat token".to_string()),
    });
    let mut exprs = vec![];

    loop {
        let line = match body_tokens.next() {
            Some(Left(e)) => e,
            _ => break,
        };

        match create_ast_expr(&mut Tokens::new(line)) {
            Ok(e) => exprs.push(e),
            Err(e) => return Err(format!("err while fetching fn body: {}", e)),
        }
    }
    Ok(exprs)
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
        Err(e) => return Err(e)
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
            Err(_) => break,
        }
    }
    Ok(funcs)
}
