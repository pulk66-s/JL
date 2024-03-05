use either::Either;

use crate::cst::{data::CstNode, keyword::create_cst_equals_keyword};

use super::generic::create_binop;

pub fn create_cst_equality_keyword(expr: &str) -> Either<&str, (CstNode, &str)> {
    create_binop(expr, create_cst_equals_keyword)
}
