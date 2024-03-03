use either::Either::{self, Right};

use super::data::CstAtom;

pub fn create_cst_spaces(expr: &str) -> Either<&str, (CstAtom, &str)> {
    let mut new_expr = expr;

    loop {
        let first_char = new_expr.chars().next();

        match first_char {
            Some(' ') => new_expr = &new_expr[1..],
            _ => return Right((CstAtom::CHAR(' '), new_expr)),
        }
    }
}

fn check_keyword(expr: &str, keyword: &str) -> bool {
    expr.starts_with(keyword)
}

pub fn create_cst_function_decl_keyword(expr: &str) -> Either<&str, (CstAtom, &str)> {
    match check_keyword(expr, "fn") {
        true => Right((CstAtom::KEYWORD("fn".to_string()), &expr[2..])),
        false => Either::Left("Expected a function declaration"),
    }
}

pub fn create_cst_identifier(expr: &str) -> Either<&str, (CstAtom, &str)> {
    let mut new_expr = expr;
    let mut identifier = String::new();

    loop {
        let first_char = new_expr.chars().next();

        match first_char {
            Some(ch) if ch.is_alphabetic() => {
                identifier.push(ch);
                new_expr = &new_expr[1..];
            }
            _ => match identifier.len() {
                0 => return Either::Left("Expected an identifier"),
                _ => return Right((CstAtom::IDENTIFIER(identifier), new_expr)),
            },
        }
    }
}

pub fn create_cst_function_return_arrow(expr: &str) -> Either<&str, (CstAtom, &str)> {
    match check_keyword(expr, "->") {
        true => Right((CstAtom::CHAR('-'), &expr[2..])),
        false => Either::Left("Expected a function return arrow"),
    }
}

pub fn create_cst_return_keyword(expr: &str) -> Either<&str, (CstAtom, &str)> {
    match check_keyword(expr, "return") {
        true => Right((CstAtom::KEYWORD("return".to_string()), &expr[6..])),
        false => Either::Left("Expected a return keyword"),
    }
}

pub fn create_cst_variable_decl_keyword(expr: &str) -> Either<&str, (CstAtom, &str)> {
    match check_keyword(expr, "let") {
        true => Right((CstAtom::KEYWORD("let".to_string()), &expr[3..])),
        false => Either::Left("Expected a variable declaration"),
    }
}
