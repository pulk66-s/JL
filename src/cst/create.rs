use either::Either::{self, Left, Right};

use crate::cst::data::CstNode;

use super::{binop::create_cst_binop, expr::create_cst_decl_expr};

pub fn create_cst_from_string(expr: &str) -> Either<&str, (CstNode, &str)> {
    let res = create_cst_binop(expr);

    match res {
        Left(_) => {},
        Right(r) => return Right(r)
    };

    let res = create_cst_decl_expr(expr);

    match res {
        Left(e) => return Left(e),
        Right(r) => return Right(r)
    };
    Left("No match found.")
}
