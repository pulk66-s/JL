use either::Either::{self, Right};

use super::data::CstAtom;

pub fn create_cst_spaces(expr: &str) -> Either<&str, (CstAtom, &str)> {
    let mut new_expr = expr;

    loop {
        let first_char = new_expr.chars().next();

        match first_char {
            Some(' ') => new_expr = &new_expr[1..],
            _ => return Right((CstAtom::CHAR(' '), new_expr))
        }
    }
}
