use crate::ast::{
    binop::{self, AstBinop},
    parse::keyword::{get_deep_token, TokenType, Tokens},
    Ast, AstExpr,
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

fn binop_priority(op: String) -> i32 {
    match op.as_str() {
        "++" | "--" => 1,
        "+" | "-" => 2,
        "*" | "/" | "%" => 3,
        "<<" | ">>" | ">>>" => 4,
        "<" | "<=" | ">" | ">=" => 5,
        "==" | "!=" => 6,
        "&" => 7,
        "^" => 8,
        "|" => 9,
        "&&" => 10,
        "||" => 11,
        _ => 0,
    }
}

fn check_optimisation(binop: String, left: AstExpr, right: AstExpr) -> AstExpr {
    let right_binop = match right.clone() {
        AstExpr::BINOP(binop) => binop,
        _ => return AstExpr::BINOP(AstBinop::new(Box::new(left), Box::new(right), binop)),
    };
    let op_priority = binop_priority(binop.clone());
    let right_binop_priority = binop_priority(right_binop.op.clone());

    if op_priority > right_binop_priority {
        return AstExpr::BINOP(AstBinop::new(Box::new(left), Box::new(right), binop));
    }
    return AstExpr::BINOP(AstBinop::new(
        Box::new(AstExpr::BINOP(AstBinop::new(
            Box::new(left),
            right_binop.left,
            binop,
        ))),
        right_binop.right,
        right_binop.op,
    ));
}

pub fn check_binop_stmt(tokens: &mut Tokens) -> Option<AstExpr> {
    let save_index = tokens.index;
    let binop;

    loop {
        binop = match get_deep_token(tokens) {
            Some(TokenType::String(op)) if is_valid_binop(op.to_string()) => op,
            Some(TokenType::Char(op)) if is_valid_binop(op.to_string()) => op.to_string(),
            None => {
                tokens.index = save_index;
                return None;
            }
            _ => continue,
        };
        break;
    }

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

    Some(check_optimisation(binop, left, right))
}
