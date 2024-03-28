use either::Either::Left;

use crate::{
    ast::{parse::keyword::Tokens, AstExpr},
    cst::parser::ParserDataType,
};

use super::{
    binop::check_binop_stmt, condition::create_ast_condition, identifier::check_identifier_stmt,
    number::check_number_stmt, ret::check_return_stmt,
};

pub fn create_ast_binop_value(tokens: &mut Tokens) -> Result<AstExpr, String> {
    if let Some(r) = check_identifier_stmt(tokens) {
        return Ok(r);
    }
    if let Some(r) = check_number_stmt(tokens) {
        return Ok(r);
    }
    Err("create_ast_binop_value Don't know how to parse this".to_string())
}

pub fn create_ast_expr_value(tokens: &mut Tokens) -> Result<AstExpr, String> {
    loop {
        match tokens.peek() {
            Some(Left(ParserDataType::None)) => {
                tokens.next();
            }
            _ => break,
        }
    }
    if let Some(r) = check_binop_stmt(tokens) {
        return Ok(r);
    }
    if let Ok(r) = create_ast_binop_value(tokens) {
        return Ok(r);
    }
    Err("create_ast_expr_value Don't know how to parse this".to_string())
}

pub fn create_ast_expr(tokens: &mut Tokens) -> Result<AstExpr, String> {
    loop {
        match tokens.peek() {
            Some(Left(ParserDataType::None)) => {
                tokens.next();
            }
            _ => break,
        }
    }
    if let Some(c) = create_ast_condition(tokens) {
        return Ok(c);
    }
    if let Some(r) = check_return_stmt(tokens) {
        return Ok(r);
    }
    if let Ok(r) = create_ast_expr_value(tokens) {
        return Ok(r);
    }
    Err("create_ast_expr Don't know how to parse this".to_string())
}
