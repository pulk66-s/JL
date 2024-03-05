use either::Either;

use crate::cst::{data::CstNode, keyword::create_cst_ge_keyword};

use super::generic::create_binop;

pub fn create_cst_greater_than_or_equals_keyword(expr: &str) -> Either<&str, (CstNode, &str)> {
    create_binop(expr, create_cst_ge_keyword)
}

#[cfg(test)]
mod tests {
    mod create_cst_greater_than_or_equals_keyword {
        use crate::cst::binop::{
            generic::tests::generic::test_binop_abstract_keyword,
            greater_or_equal::create_cst_greater_than_or_equals_keyword,
        };

        #[test]
        pub fn test_basic_create_cst_greater_than_or_equals_keyword() {
            test_binop_abstract_keyword(
                create_cst_greater_than_or_equals_keyword,
                ">=",
                "test_basic_create_cst_greater_than_or_equals_keyword",
                2,
                vec![1.0, 2.0],
                "1>=2",
            );
        }

        #[test]
        pub fn test_spaced_create_cst_greater_than_or_equals_keyword() {
            test_binop_abstract_keyword(
                create_cst_greater_than_or_equals_keyword,
                ">=",
                "test_spaced_create_cst_greater_than_or_equals_keyword",
                2,
                vec![1.0, 2.0],
                "1 >= 2",
            );
        }
    }
}
