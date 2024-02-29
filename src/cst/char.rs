use either::Either::{self, Left, Right};

use super::data::CstAtom;

fn create_cst_char_atom(expr: &str, c: char) -> Either<&str, (CstAtom, &str)> {
    match expr.chars().next() {
        Some(ch) if ch == c => Right((CstAtom::CHAR(c), &expr[1..])),
        _ => Left(expr)
    }
}

pub fn create_cst_add_atom(expr: &str) -> Either<&str, (CstAtom, &str)> {
    create_cst_char_atom(expr, '+')
}

pub fn create_cst_sub_atom(expr: &str) -> Either<&str, (CstAtom, &str)> {
    create_cst_char_atom(expr, '-')
}

pub fn create_cst_mul_atom(expr: &str) -> Either<&str, (CstAtom, &str)> {
    create_cst_char_atom(expr, '*')
}

pub fn create_cst_div_atom(expr: &str) -> Either<&str, (CstAtom, &str)> {
    create_cst_char_atom(expr, '/')
}

pub fn create_cst_endexpr_atom(expr: &str) -> Either<&str, (CstAtom, &str)> {
    create_cst_char_atom(expr, ';')
}
