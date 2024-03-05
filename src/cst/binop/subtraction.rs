use either::Either;

use crate::cst::{char::create_cst_sub_atom, data::CstNode};

use super::generic::create_binop;

pub fn create_cst_subtraction(expr: &str) -> Either<&str, (CstNode, &str)> {
    create_binop(expr, create_cst_sub_atom)
}
