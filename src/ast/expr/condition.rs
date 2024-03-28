use crate::ast::{
    condition::ConditionAst,
    expr::value::create_ast_expr_value,
    func::body::fetch_fn_body,
    parse::keyword::{match_keyword, Tokens},
    AstExpr,
};

fn fetch_expr(tokens: &mut Tokens) -> Option<Vec<AstExpr>> {
    tokens.next();
    tokens.next();
    tokens.next();
    if !match_keyword(tokens, "else") {
        return None;
    }
    tokens.next();
    tokens.next();
    tokens.next();
    tokens.next();
    tokens.next();
    match fetch_fn_body(tokens) {
        Ok(e) => Some(e),
        Err(e) => {
            println!("fetch_expr err: {:?}", e);
            None
        }
    }
}

pub fn create_ast_condition(tokens: &mut Tokens) -> Option<AstExpr> {
    let save_index = tokens.index;

    if match_keyword(tokens, "if") == false {
        tokens.index = save_index;
        return None;
    }

    tokens.next();
    tokens.next();

    let condition = match create_ast_expr_value(tokens) {
        Ok(e) => e,
        Err(e) => {
            println!("condition err: {:?}", e);
            tokens.index = save_index;
            return None;
        }
    };

    println!(
        "token: {}",
        tokens.peek().unwrap().unwrap_left().to_string()
    );
    tokens.next();
    tokens.next();
    tokens.next();
    tokens.next();

    let true_expr = match fetch_fn_body(tokens) {
        Ok(e) => e,
        Err(e) => {
            println!("true_expr err: {:?}", e);
            tokens.index = save_index;
            return None;
        }
    };

    let else_expr = fetch_expr(tokens);

    Some(AstExpr::CONDITION(ConditionAst::new(
        condition, true_expr, else_expr,
    )))
}
