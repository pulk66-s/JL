use either::Either;

use crate::cst::{data::CstNode, keyword::create_cst_equals_keyword};

use super::generic::create_binop;

pub fn create_cst_equality_keyword(expr: &str) -> Either<&str, (CstNode, &str)> {
    create_binop(expr, create_cst_equals_keyword)
}

#[cfg(test)]
mod tests {
    mod equality {
        use crate::cst::binop::{
            equality::create_cst_equality_keyword,
            generic::tests::generic::test_binop_abstract_keyword,
        };

        #[test]
        pub fn test_basic_equality() {
            test_binop_abstract_keyword(
                create_cst_equality_keyword,
                "==",
                "test_basic_equality",
                2,
                vec![1.0, 2.0],
                "1==2",
            );
        }

        #[test]
        pub fn test_spaced_equality() {
            test_binop_abstract_keyword(
                create_cst_equality_keyword,
                "==",
                "test_spaced_equality",
                2,
                vec![1.0, 2.0],
                "1 == 2",
            );
        }
    }
}
