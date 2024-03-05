use either::Either::{self, Left, Right};

use crate::cst::char::create_cst_closebrace_atom;

use super::{
    char::{create_cst_closepar_atom, create_cst_openbrace_atom, create_cst_openpar_atom}, data::{CstCondition, CstNode}, expr::create_cst_value_expr, function::function_decl::body::create_cst_function_decl_body, keyword::{create_cst_if_keyword, create_cst_spaces}
};

pub fn create_cst_condition(expr: &str) -> Either<&str, (CstNode, &str)> {
    let (keyword, new_expr) = match create_cst_if_keyword(expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };
    let (_, new_expr) = match create_cst_spaces(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };
    let (open_par, new_expr) = match create_cst_openpar_atom(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };
    let (_, new_expr) = match create_cst_spaces(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };
    let (condition, new_expr) = match create_cst_value_expr(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };
    let (_, new_expr) = match create_cst_spaces(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };
    let (close_par, new_expr) = match create_cst_closepar_atom(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };
    let (_, new_expr) = match create_cst_spaces(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };
    let (open_brace, new_expr) = match create_cst_openbrace_atom(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };
    let (_, new_expr) = match create_cst_spaces(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };
    let (body, new_expr) = create_cst_function_decl_body(new_expr);
    let (_, new_expr) = match create_cst_spaces(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };
    let (close_brace, new_expr) = match create_cst_closebrace_atom(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };

    Right((
        CstNode::CONDITION(CstCondition {
            keyword: keyword,
            open_par: open_par,
            condition: Box::new(condition),
            close_par: close_par,
            open_brace: open_brace,
            body: body,
            close_brace: close_brace,
            else_condition: None,
        }),
        new_expr,
    ))
}
