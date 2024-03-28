use crate::ast::{
    parse::keyword::{get_deep_token, TokenType, Tokens},
    returnstmt::AstReturn,
    AstExpr,
};

use super::value::create_ast_expr;

pub fn check_return_stmt(tokens: &mut Tokens) -> Option<AstExpr> {
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
