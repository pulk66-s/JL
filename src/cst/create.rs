use either::Either;

use crate::cst::data::CstNode;

use super::binop::create_cst_addition;

pub fn create_cst_from_string(expr: &str) -> Either<&str, (CstNode, &str)> {
    create_cst_addition(expr)
}
