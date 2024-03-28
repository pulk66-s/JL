use crate::ast::{
    parse::keyword::{get_deep_token, TokenType, Tokens},
    AstExpr,
};

pub fn check_identifier_stmt(tokens: &mut Tokens) -> Option<AstExpr> {
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
