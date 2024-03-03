use either::Either::{self, Left, Right};

use crate::cst::data::CstNode;

use super::{binop::create_cst_binop, expr::create_cst_decl_expr, function_call::create_cst_function_call};

pub fn create_cst_from_string(expr: &str) -> Either<&str, (CstNode, &str)> {
    match create_cst_binop(expr) {
        Left(_) => {},
        Right(r) => return Right(r)
    };
    match create_cst_decl_expr(expr) {
        Left(_) => {},
        Right(r) => return Right(r)
    };
    match create_cst_function_call(expr) {
        Left(_) => {},
        Right(r) => return Right(r)
    };
    Left("No match found.")
}
