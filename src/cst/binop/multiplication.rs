use either::Either;

use crate::cst::{char::create_cst_mul_atom, data::CstNode};

use super::generic::create_binop;

pub fn create_cst_multiplication(expr: &str) -> Either<&str, (CstNode, &str)> {
    create_binop(expr, create_cst_mul_atom)
}

#[cfg(test)]
mod tests {
    mod multiplication {
        use crate::cst::binop::{
            generic::tests::generic::test_binop_abstract, multiplication::create_cst_multiplication,
        };

        #[test]
        pub fn test_basic_multiplication() {
            test_binop_abstract(
                create_cst_multiplication,
                '*',
                "test_basic_multiplication",
                2,
                vec![1.0, 2.0],
                "1*2",
            );
        }

        #[test]
        pub fn test_spaced_multiplication() {
            test_binop_abstract(
                create_cst_multiplication,
                '*',
                "test_spaced_multiplication",
                2,
                vec![1.0, 2.0],
                "1 * 2",
            );
        }

        #[test]
        pub fn test_chained_multiplication() {
            test_binop_abstract(
                create_cst_multiplication,
                '*',
                "test_chained_multiplication",
                3,
                vec![1.0, 2.0, 3.0],
                "1 * 2 * 3",
            );
        }
    }
}
