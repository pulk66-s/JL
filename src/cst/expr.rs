use either::Either::{self, Left, Right};

use crate::cst::{
    number::create_cst_number,
    data::CstNode
};

use super::function_decl::create_cst_function_decl;

pub fn create_cst_value_expr(expr: &str) -> Either<&str, (CstNode, &str)> {
    match create_cst_number(expr) {
        Left(_) => {},
        Right((atom, new_expr)) => return Right((CstNode::ATOM(atom), new_expr)),
    };
    Left("create_cst_value_expr: no match found.")
}

pub fn create_cst_decl_expr(expr: &str) -> Either<&str, (CstNode, &str)> {
    create_cst_function_decl(expr)
}
