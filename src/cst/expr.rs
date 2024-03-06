use either::Either::{self, Left, Right};

use crate::cst::{data::CstNode, number::create_cst_number};

use super::{
    binop::expr::create_cst_binop,
    char::create_cst_endexpr_atom,
    conditions::create_cst_condition,
    data::{CstFunctionLineExpr, CstLine, CstReturnExpr},
    function::{function_call::create_cst_function_call, function_decl::create_cst_function_decl},
    keyword::{create_cst_identifier, create_cst_return_keyword, create_cst_spaces}, variable_decl::create_cst_variable_decl,
};

pub fn create_cst_atom_value_expr(expr: &str) -> Either<&str, (CstNode, &str)> {
    match create_cst_number(expr) {
        Left(_) => {}
        Right((atom, new_expr)) => return Right((CstNode::ATOM(atom), new_expr)),
    };
    match create_cst_identifier(expr) {
        Left(_) => {}
        Right((atom, new_expr)) => return Right((CstNode::ATOM(atom), new_expr)),
    };
    Left("create_cst_atom_value_expr: no match found.")
}

pub fn create_cst_value_expr(expr: &str) -> Either<&str, (CstNode, &str)> {
    match create_cst_binop(expr) {
        Left(_) => {}
        Right(r) => return Right(r),
    };
    match create_cst_function_call(expr) {
        Left(_) => {}
        Right(r) => return Right(r),
    };
    match create_cst_atom_value_expr(expr) {
        Left(_) => {}
        Right(r) => return Right(r),
    };
    Left("create_cst_value_expr: no match found.")
}

pub fn create_cst_value_line_expr_possibilities(expr: &str) -> Either<&str, (CstNode, &str)> {
    match create_cst_variable_decl(expr) {
        Left(_) => {}
        Right(r) => return Right(r),
    };
    match create_cst_value_expr(expr) {
        Left(_) => {}
        Right(r) => return Right(r),
    };
    Left("create_cst_value_line_expr_possibilities: no match found.")
}

pub fn create_cst_value_line_expr(expr: &str) -> Either<&str, (CstNode, &str)> {
    let (expr, new_expr) = match create_cst_value_line_expr_possibilities(expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };
    let (_, new_expr) = match create_cst_spaces(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };
    let (endline, new_expr) = match create_cst_endexpr_atom(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };

    Right((
        CstNode::FUNCTION_LINE(CstFunctionLineExpr::LINE(CstLine {
            expr: Box::new(expr),
            endline: endline,
        })),
        new_expr,
    ))
}

pub fn create_cst_function_expr(expr: &str) -> Either<&str, (CstNode, &str)> {
    match create_cst_function_return_expr(expr) {
        Left(_) => {}
        Right(r) => return Right(r),
    };
    match create_cst_condition(expr) {
        Left(_) => {}
        Right((CstNode::CONDITION(cond), rest)) => {
            return Right((
                CstNode::FUNCTION_LINE(CstFunctionLineExpr::CONDITION(cond)),
                rest,
            ))
        }
        Right(_) => return Left("create_cst_function_expr: condition not implemented yet."),
    };
    match create_cst_value_line_expr(expr) {
        Left(_) => {}
        Right(r) => return Right(r),
    };
    Left("create_cst_function_expr: no match found.")
}

pub fn create_cst_function_return_expr(expr: &str) -> Either<&str, (CstNode, &str)> {
    let (keyword, new_expr) = match create_cst_return_keyword(expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };
    let (_, new_expr) = match create_cst_spaces(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };
    let (value, new_expr) = match create_cst_value_expr(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };
    let (endline, new_expr) = match create_cst_endexpr_atom(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };

    Right((
        CstNode::FUNCTION_LINE(CstFunctionLineExpr::RETURN(CstReturnExpr {
            keyword: keyword,
            value: Box::new(value),
            endline: endline,
        })),
        new_expr,
    ))
}
