use either::Either::{self, Left, Right};

use crate::cst::{
    number::create_cst_number,
    data::CstNode
};

use super::binop::create_cst_binop;

pub fn create_cst_value_expr(expr: &str) -> Either<&str, (CstNode, &str)> {
    match create_cst_binop(expr) {
        Left(_) => {},
        Right((binop, new_expr)) => return Right((binop, new_expr)),
    };
    match create_cst_number(expr) {
        Left(_) => {},
        Right((atom, new_expr)) => return Right((CstNode::ATOM(atom), new_expr)),
    };
    Left("create_cst_value_expr: no match found.")
}
