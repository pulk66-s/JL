use crate::ast::{
    parse::keyword::{get_deep_token, TokenType, Tokens},
    AstExpr,
};

pub fn check_number_stmt(tokens: &mut Tokens) -> Option<AstExpr> {
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
