use either::Either;

use crate::cst::{char::create_cst_div_atom, data::CstNode};

use super::generic::create_binop;

pub fn create_cst_division(expr: &str) -> Either<&str, (CstNode, &str)> {
    create_binop(expr, create_cst_div_atom)
}

#[cfg(test)]
mod tests {

    mod division {
        use crate::cst::binop::{
            division::create_cst_division, generic::tests::generic::test_binop_abstract,
        };

        #[test]
        pub fn test_basic_division() {
            test_binop_abstract(
                create_cst_division,
                '/',
                "test_basic_division",
                2,
                vec![1.0, 2.0],
                "1/2",
            );
        }

        #[test]
        pub fn test_spaced_division() {
            test_binop_abstract(
                create_cst_division,
                '/',
                "test_spaced_division",
                2,
                vec![1.0, 2.0],
                "1 / 2",
            );
        }

        #[test]
        pub fn test_chained_division() {
            test_binop_abstract(
                create_cst_division,
                '/',
                "test_chained_division",
                3,
                vec![1.0, 2.0, 3.0],
                "1 / 2 / 3",
            );
        }
    }
}
