use either::Either::{Left, Right};

use crate::{
    ast::{
        parse::keyword::{get_deep_token, TokenType, Tokens},
        variable::AstVariableDecl,
    },
    cst::parser::ParserDataType,
};

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

pub fn get_fn_params(tokens: &mut Tokens) -> Result<Vec<AstVariableDecl>, String> {
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
