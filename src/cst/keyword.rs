use either::Either::{self, Right};

use super::data::CstAtom;

pub fn create_cst_spaces(expr: &str) -> Either<&str, (CstAtom, &str)> {
    let mut new_expr = expr;

    loop {
        let first_char = new_expr.chars().next();

        match first_char {
            Some(' ') => new_expr = &new_expr[1..],
            Some('\n') => new_expr = &new_expr[1..],
            Some('\t') => new_expr = &new_expr[1..],
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
        true => Right((CstAtom::KEYWORD("->".to_string()), &expr[2..])),
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

pub fn create_cst_equals_keyword(expr: &str) -> Either<&str, (CstAtom, &str)> {
    match check_keyword(expr, "==") {
        true => Right((CstAtom::KEYWORD("==".to_string()), &expr[2..])),
        false => Either::Left("Expected an equals keyword"),
    }
}

pub fn create_cst_ge_keyword(expr: &str) -> Either<&str, (CstAtom, &str)> {
    match check_keyword(expr, ">=") {
        true => Right((CstAtom::KEYWORD(">=".to_string()), &expr[2..])),
        false => Either::Left("Expected a greater than or equal keyword"),
    }
}

pub fn create_cst_gt_keyword(expr: &str) -> Either<&str, (CstAtom, &str)> {
    match check_keyword(expr, ">") {
        true => Right((CstAtom::KEYWORD(">".to_string()), &expr[1..])),
        false => Either::Left("Expected a greater than keyword"),
    }
}

pub fn create_cst_le_keyword(expr: &str) -> Either<&str, (CstAtom, &str)> {
    match check_keyword(expr, "<=") {
        true => Right((CstAtom::KEYWORD("<=".to_string()), &expr[2..])),
        false => Either::Left("Expected a less than or equal keyword"),
    }
}

pub fn create_cst_lt_keyword(expr: &str) -> Either<&str, (CstAtom, &str)> {
    match check_keyword(expr, "<") {
        true => Right((CstAtom::KEYWORD("<".to_string()), &expr[1..])),
        false => Either::Left("Expected a less than keyword"),
    }
}

pub fn create_cst_ne_keyword(expr: &str) -> Either<&str, (CstAtom, &str)> {
    match check_keyword(expr, "!=") {
        true => Right((CstAtom::KEYWORD("!=".to_string()), &expr[2..])),
        false => Either::Left("Expected a not equal keyword"),
    }
}

pub fn create_cst_if_keyword(expr: &str) -> Either<&str, (CstAtom, &str)> {
    match check_keyword(expr, "if") {
        true => Right((CstAtom::KEYWORD("if".to_string()), &expr[2..])),
        false => Either::Left("Expected an if keyword"),
    }
}

pub fn create_cst_else_keyword(expr: &str) -> Either<&str, (CstAtom, &str)> {
    match check_keyword(expr, "else") {
        true => Right((CstAtom::KEYWORD("else".to_string()), &expr[4..])),
        false => Either::Left("Expected an else keyword"),
    }
}

#[cfg(test)]
pub mod tests {
    pub mod keyword {
        use crate::cst::data::CstAtom;

        pub fn atom_match_keyword(atom: &CstAtom, value: &str, err_message: &str) {
            match atom {
                CstAtom::KEYWORD(k) => assert_eq!(*k, value),
                _ => panic!("Expected a keyword atom. {}", err_message),
            }
        }
    }

    pub mod identifier {
        use crate::cst::data::CstAtom;

        pub fn atom_match_identifier(atom: &CstAtom, value: &str, err_message: &str) {
            match atom {
                CstAtom::IDENTIFIER(i) => assert_eq!(*i, value),
                _ => panic!("Expected an identifier atom. {}", err_message),
            }
        }
    }
}
