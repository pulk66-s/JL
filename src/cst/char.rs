use either::Either::{self, Left, Right};

use super::data::CstAtom;

fn create_cst_char_atom(expr: &str, c: char) -> Either<&str, (CstAtom, &str)> {
    match expr.chars().next() {
        Some(ch) if ch == c => Right((CstAtom::CHAR(c), &expr[1..])),
        _ => Left("Expected a char atom"),
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

pub fn create_cst_comma_atom(expr: &str) -> Either<&str, (CstAtom, &str)> {
    create_cst_char_atom(expr, ',')
}

pub fn create_cst_openpar_atom(expr: &str) -> Either<&str, (CstAtom, &str)> {
    create_cst_char_atom(expr, '(')
}

pub fn create_cst_closepar_atom(expr: &str) -> Either<&str, (CstAtom, &str)> {
    create_cst_char_atom(expr, ')')
}

pub fn create_cst_openbrace_atom(expr: &str) -> Either<&str, (CstAtom, &str)> {
    create_cst_char_atom(expr, '{')
}

pub fn create_cst_closebrace_atom(expr: &str) -> Either<&str, (CstAtom, &str)> {
    create_cst_char_atom(expr, '}')
}

pub fn create_cst_equal_atom(expr: &str) -> Either<&str, (CstAtom, &str)> {
    create_cst_char_atom(expr, '=')
}

#[cfg(test)]
pub mod tests {
    use crate::cst::data::CstAtom;

    pub fn atom_match_char(atom: &CstAtom, c: char, err_message: &str) {
        match atom {
            CstAtom::CHAR(ch) => assert_eq!(*ch, c, "{}", err_message),
            _ => panic!("atom_match_char: atom is not a char"),
        }
    }
}
