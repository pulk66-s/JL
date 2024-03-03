use either::Either::{self, Left, Right};

use super::{
    data::{CstFunctionLineExpr, CstNode, CstReturnExpr},
    expr::create_cst_value_expr,
    keyword::{create_cst_return_keyword, create_cst_spaces},
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

    Right((
        CstNode::FUNCTION_LINE_EXPR(CstFunctionLineExpr::RETURN(CstReturnExpr {
            keyword: keyword,
            value: Box::new(value),
        })),
        new_expr,
    ))
}
