use either::Either;

use crate::cst::{data::CstNode, keyword::create_cst_ge_keyword};

use super::generic::create_binop;

pub fn create_cst_greater_than_or_equals_keyword(expr: &str) -> Either<&str, (CstNode, &str)> {
    create_binop(expr, create_cst_ge_keyword)
}
