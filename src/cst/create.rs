use either::Either::{self, Left, Right};

use crate::cst::data::CstNode;

use super::{
    binop::expr::create_cst_binop, expr::create_cst_decl_expr,
    function::function_call::create_cst_function_call, keyword::create_cst_identifier,
    variable_decl::create_cst_variable_decl,
};

pub fn create_cst_from_string(expr: &str) -> Either<&str, (CstNode, &str)> {
    match create_cst_binop(expr) {
        Left(_) => {}
        Right(r) => return Right(r),
    };
    match create_cst_decl_expr(expr) {
        Left(_) => {}
        Right(r) => return Right(r),
    };
    match create_cst_function_call(expr) {
        Left(_) => {}
        Right(r) => return Right(r),
    };
    match create_cst_variable_decl(expr) {
        Left(_) => {}
        Right(r) => return Right(r),
    };
    match create_cst_identifier(expr) {
        Left(_) => {}
        Right((atom, rest)) => return Right((CstNode::ATOM(atom), rest)),
    };
    Left("No match found.")
}

// pub fn create_cst_from_string(expr: &str) -> Either<&str, (CstNode, &str)> {
//     create_cst_condition(expr)
// }
