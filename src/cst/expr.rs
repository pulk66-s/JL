use either::Either::{self, Left, Right};

use crate::cst::{
    number::create_cst_number,
    data::CstNode
};

pub fn create_cst_value_expr(expr: &str) -> Either<&str, (CstNode, &str)> {
    match create_cst_number(expr) {
        Left(new_expr) => Left(new_expr),
        Right((atom, new_expr)) => Right((CstNode::ATOM(atom), new_expr)),
    }
}
