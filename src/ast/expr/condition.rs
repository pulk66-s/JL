use crate::ast::{
    expr::value::create_ast_expr_value,
    parse::keyword::{match_keyword, Tokens},
    AstExpr,
};

pub fn create_ast_condition(tokens: &mut Tokens) -> Option<AstExpr> {
    println!("create_ast_condition");
    let save_index = tokens.index;

    if match_keyword(tokens, "if") == false {
        tokens.index = save_index;
        return None;
    }
    println!("create_ast_condition 2");

    tokens.next();
    tokens.next();

    println!(
        "create_ast_condition 3 {}",
        tokens.peek().unwrap().unwrap_left().to_string()
    );
    let condition = match create_ast_expr_value(tokens) {
        Ok(e) => e,
        Err(e) => {
            println!("condition err: {:?}", e);
            tokens.index = save_index;
            return None;
        }
    };
    println!("condition: {:?}", condition);

    tokens.index = save_index;
    None
}
