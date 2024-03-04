use either::Either::{self, Left, Right};

use super::{
    char::create_cst_endexpr_atom, data::{CstFunctionLineExpr, CstNode, CstReturnExpr}, expr::create_cst_value_expr, keyword::{create_cst_return_keyword, create_cst_spaces}
};

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
