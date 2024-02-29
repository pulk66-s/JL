use either::Either::{self, Left, Right};

use super::data::CstAtom;

pub fn create_cst_add_atom(expr: &str) -> Either<&str, (CstAtom, &str)> {
    match expr.chars().next() {
        Some('+') => Right((CstAtom::CHAR('+'), &expr[1..])),
        _ => Left(expr)
    }
}
