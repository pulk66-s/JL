use either::Either::{Left, Right};

use crate::ast::parse::keyword::get_deep_token;

use super::{
    binop::AstBinop,
    parse::keyword::{TokenType, Tokens},
    returnstmt::AstReturn,
    AstExpr,
};

fn check_return_stmt(tokens: &mut Tokens) -> Option<AstExpr> {
    let save_index = tokens.index;

    match get_deep_token(tokens) {
        Some(TokenType::String(s)) if s == "return" => {
            tokens.next();
            match create_ast_expr(tokens) {
                Ok(e) => Some(AstExpr::RETURN(AstReturn::new(Box::new(e)))),
                Err(_) => {
                    tokens.index = save_index;
                    None
                }
            }
        }
        _ => {
            tokens.index = save_index;
            None
        }
    }
}

fn is_valid_binop(op: String) -> bool {
    match op.as_str() {
        "+" | "-" | "*" | "/" | "%" | "==" | "<=" | ">=" | "<" | ">" | "!=" | "&&" | "||" | "^"
        | "&" | "|" | "<<" | ">>" | ">>>" | "++" | "--" | "+=" | "-=" | "*=" | "/=" | "%="
        | "&=" | "|=" | "^=" | "<<=" | ">>=" | ">>>=" => true,
        _ => false,
    }
}

fn check_binop_stmt(tokens: &mut Tokens) -> Option<AstExpr> {
    let save_index = tokens.index;
    loop {
        let binop = match get_deep_token(tokens) {
            Some(TokenType::String(op)) if is_valid_binop(op.to_string()) => op,
            Some(TokenType::Char(op)) if is_valid_binop(op.to_string()) => op.to_string(),
            None => {
                tokens.index = save_index;
                return None;
            }
            _ => continue,
        };

        tokens.prev();
        tokens.prev();
        tokens.prev();

        let left = match create_ast_expr_value(tokens) {
            Ok(e) => e,
            Err(_) => {
                tokens.index = save_index;
                return None;
            }
        };

        tokens.next();
        tokens.next();

        let right = match create_ast_expr(tokens) {
            Ok(e) => e,
            Err(_) => {
                tokens.index = save_index;
                return None;
            }
        };

        return Some(AstExpr::BINOP(AstBinop::new(
            Box::new(left),
            Box::new(right),
            binop,
        )));
    }
}

fn check_identifier_stmt(tokens: &mut Tokens) -> Option<AstExpr> {
    let save_index = tokens.index;

    match get_deep_token(tokens) {
        Some(TokenType::String(s)) => {
            tokens.next();
            Some(AstExpr::VARIABLE_CALL(s))
        }
        _ => {
            tokens.index = save_index;
            None
        }
    }
}

fn check_number_stmt(tokens: &mut Tokens) -> Option<AstExpr> {
    let save_index = tokens.index;

    match get_deep_token(tokens) {
        Some(TokenType::Number(n)) => {
            tokens.next();
            Some(AstExpr::NUMBER(n))
        }
        _ => {
            tokens.index = save_index;
            None
        }
    }
}

fn create_ast_expr_value(tokens: &mut Tokens) -> Result<AstExpr, String> {
    if let Some(r) = check_identifier_stmt(tokens) {
        return Ok(r);
    }
    if let Some(r) = check_number_stmt(tokens) {
        return Ok(r);
    }
    Err("Don't know how to parse this".to_string())
}

pub fn create_ast_expr(tokens: &mut Tokens) -> Result<AstExpr, String> {
    if let Some(r) = check_return_stmt(tokens) {
        return Ok(r);
    }
    if let Some(r) = check_binop_stmt(tokens) {
        return Ok(r);
    }
    if let Ok(r) = create_ast_expr_value(tokens) {
        return Ok(r);
    }
    Err("Don't know how to parse this".to_string())
}