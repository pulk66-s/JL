use super::data::CstAtom;
use either::Either::{self, Left, Right};

fn to_number(c: char) -> Option<f64> {
    match c {
        '0' => Some(0.0),
        '1' => Some(1.0),
        '2' => Some(2.0),
        '3' => Some(3.0),
        '4' => Some(4.0),
        '5' => Some(5.0),
        '6' => Some(6.0),
        '7' => Some(7.0),
        '8' => Some(8.0),
        '9' => Some(9.0),
        _ => None,
    }
}

fn create_cst_number_end(new_expr: &str, found: bool, res: f64) -> Either<&str, (CstAtom, &str)> {
    match found {
        true => Right((CstAtom::NUMBER(res), new_expr)),
        false => Left("No number found."),
    }
}

pub fn create_cst_number(expr: &str) -> Either<&str, (CstAtom, &str)> {
    let mut found = false;
    let mut res = 0.0;
    let mut new_expr = expr;

    loop {
        let first_char = new_expr.chars().next();

        match first_char {
            None => return create_cst_number_end(new_expr, found, res),
            Some(c) => match to_number(c) {
                Some(n) => {
                    found = true;
                    res = res * 10.0 + n;
                    new_expr = &new_expr[1..];
                },
                None => return create_cst_number_end(new_expr, found, res),
            },
        }
    }
}
