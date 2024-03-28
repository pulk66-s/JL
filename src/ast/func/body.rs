use either::Either::Left;

use crate::{
    ast::{expr::value::create_ast_expr, parse::keyword::Tokens, AstExpr},
    cst::parser::ParserDataType,
};

pub fn fetch_fn_body(tokens: &mut Tokens) -> Result<Vec<AstExpr>, String> {
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
