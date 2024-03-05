use either::Either;

use crate::cst::{char::create_cst_add_atom, data::CstNode};

use super::generic::create_binop;

pub fn create_cst_addition(expr: &str) -> Either<&str, (CstNode, &str)> {
    create_binop(expr, create_cst_add_atom)
}

#[cfg(test)]
mod tests {

    mod addition {
        use crate::cst::binop::{
            addition::create_cst_addition, generic::tests::generic::test_binop_abstract,
        };

        #[test]
        pub fn test_basic_addition() {
            test_binop_abstract(
                create_cst_addition,
                '+',
                "test_basic_addition",
                2,
                vec![1.0, 2.0],
                "1+2",
            );
        }

        #[test]
        pub fn test_spaced_addition() {
            test_binop_abstract(
                create_cst_addition,
                '+',
                "test_spaced_addition",
                2,
                vec![1.0, 2.0],
                "1 + 2",
            );
        }

        #[test]
        pub fn test_chained_addition() {
            test_binop_abstract(
                create_cst_addition,
                '+',
                "test_chained_addition",
                3,
                vec![1.0, 2.0, 3.0],
                "1 + 2 + 3",
            );
        }
    }
}
