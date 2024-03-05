use either::Either;

use crate::cst::{char::create_cst_sub_atom, data::CstNode};

use super::generic::create_binop;

pub fn create_cst_subtraction(expr: &str) -> Either<&str, (CstNode, &str)> {
    create_binop(expr, create_cst_sub_atom)
}

#[cfg(test)]
mod tests {
    mod subtraction {
        use crate::cst::binop::{
            generic::tests::generic::test_binop_abstract, subtraction::create_cst_subtraction,
        };

        #[test]
        pub fn test_basic_subtraction() {
            test_binop_abstract(
                create_cst_subtraction,
                '-',
                "test_basic_subtraction",
                2,
                vec![1.0, 2.0],
                "1-2",
            );
        }

        #[test]
        pub fn test_spaced_subtraction() {
            test_binop_abstract(
                create_cst_subtraction,
                '-',
                "test_spaced_subtraction",
                2,
                vec![1.0, 2.0],
                "1 - 2",
            );
        }

        #[test]
        pub fn test_chained_subtraction() {
            test_binop_abstract(
                create_cst_subtraction,
                '-',
                "test_chained_subtraction",
                3,
                vec![1.0, 2.0, 3.0],
                "1 - 2 - 3",
            );
        }
    }
}
