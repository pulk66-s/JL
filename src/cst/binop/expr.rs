use either::Either::{self, Left, Right};

use crate::cst::data::CstNode;

use super::{
    addition::create_cst_addition, division::create_cst_division,
    equality::create_cst_equality_keyword, greater::create_cst_greater_than_keyword,
    greater_or_equal::create_cst_greater_than_or_equals_keyword,
    less::create_cst_less_than_keyword, less_or_equal::create_cst_less_than_or_equals_keyword,
    modulo::create_cst_modulo, multiplication::create_cst_multiplication,
    subtraction::create_cst_subtraction,
};

pub fn create_cst_binop(expr: &str) -> Either<&str, (CstNode, &str)> {
    match create_cst_addition(expr) {
        Right(r) => return Right(r),
        Left(_) => {}
    };
    match create_cst_subtraction(expr) {
        Right(r) => return Right(r),
        Left(_) => {}
    };
    match create_cst_multiplication(expr) {
        Right(r) => return Right(r),
        Left(_) => {}
    };
    match create_cst_division(expr) {
        Right(r) => return Right(r),
        Left(_) => {}
    };
    match create_cst_greater_than_keyword(expr) {
        Right(r) => return Right(r),
        Left(_) => {}
    };
    match create_cst_greater_than_or_equals_keyword(expr) {
        Right(r) => return Right(r),
        Left(_) => {}
    };
    match create_cst_less_than_keyword(expr) {
        Right(r) => return Right(r),
        Left(_) => {}
    };
    match create_cst_less_than_or_equals_keyword(expr) {
        Right(r) => return Right(r),
        Left(_) => {}
    };
    match create_cst_equality_keyword(expr) {
        Right(r) => return Right(r),
        Left(_) => {}
    };
    match create_cst_modulo(expr) {
        Right(r) => return Right(r),
        Left(_) => {}
    };
    Left("No binop found.")
}

#[cfg(test)]
pub mod tests {
    pub mod exprs {
        use crate::cst::data::{CstAtom, CstBinop, CstNode};

        pub fn atom_match_float(atom: &CstAtom, value: f64, err_message: &str) {
            match atom {
                CstAtom::NUMBER(n) => assert_eq!(*n, value),
                _ => panic!("Expected a number atom. {}", err_message),
            }
        }

        pub fn node_match_float(node: &CstNode, value: f64, err_message: &str) {
            match node {
                CstNode::ATOM(atom) => atom_match_float(atom, value, err_message),
                _ => panic!("Err: {}", err_message),
            }
        }

        pub fn node_match_binop<'a>(node: &'a CstNode, err_message: &str) -> &'a CstBinop {
            match node {
                CstNode::BINOP(binop) => binop,
                _ => panic!("Err: {}", err_message),
            }
        }
    }
}
