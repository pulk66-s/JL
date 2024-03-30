use crate::ast::{
    parse::keyword::{get_deep_token, TokenType, Tokens},
    AstExpr,
};

pub fn check_identifier_stmt(tokens: &mut Tokens) -> Option<AstExpr> {
    let save_index = tokens.index;

    match get_deep_token(tokens) {
        Some(TokenType::String(s)) => match s.as_str() {
            "false" => Some(AstExpr::NUMBER(0)),
            "true" => Some(AstExpr::NUMBER(1)),
            _ => {
                tokens.next();
                Some(AstExpr::VARIABLE_CALL(s))
            }
        },
        _ => {
            tokens.index = save_index;
            None
        }
    }
}
