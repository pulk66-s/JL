use either::Either::{Left, Right};

use crate::cst::{data::{CstFunctionLineExpr, CstNode}, expr::create_cst_function_expr, keyword::create_cst_spaces};

pub fn create_cst_function_decl_body(expr: &str) -> (Vec<CstFunctionLineExpr>, &str) {
    let mut lines = vec![];
    let mut line;
    let mut new_expr = expr;

    loop {
        (_, new_expr) = match create_cst_spaces(new_expr) {
            Left(_) => break,
            Right(r) => r,
        };
        (line, new_expr) = match create_cst_function_expr(new_expr) {
            Left(_) => break,
            Right(r) => r,
        };
        (_, new_expr) = match create_cst_spaces(new_expr) {
            Left(_) => break,
            Right(r) => r,
        };
        lines.push(match line {
            CstNode::FUNCTION_LINE(line) => line,
            _ => break,
        });
    }
    (lines, new_expr)
}
