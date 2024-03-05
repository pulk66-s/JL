use either::Either;

use crate::cst::{char::create_cst_div_atom, data::CstNode};

use super::generic::create_binop;

pub fn create_cst_division(expr: &str) -> Either<&str, (CstNode, &str)> {
    create_binop(expr, create_cst_div_atom)
}
