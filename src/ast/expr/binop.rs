use crate::ast::{
    binop::AstBinop,
    parse::keyword::{get_deep_token, TokenType, Tokens},
    AstExpr,
};

use super::value::{create_ast_binop_value, create_ast_expr};

fn is_valid_binop(op: String) -> bool {
    match op.as_str() {
        "+" | "-" | "*" | "/" | "%" | "==" | "<=" | ">=" | "<" | ">" | "!=" | "&&" | "||" | "^"
        | "&" | "|" | "<<" | ">>" | ">>>" | "++" | "--" | "+=" | "-=" | "*=" | "/=" | "%="
        | "&=" | "|=" | "^=" | "<<=" | ">>=" | ">>>=" => true,
        _ => false,
    }
}

pub fn check_binop_stmt(tokens: &mut Tokens) -> Option<AstExpr> {
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

        let left = match create_ast_binop_value(tokens) {
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
