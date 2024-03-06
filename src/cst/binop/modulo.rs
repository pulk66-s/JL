use either::Either;

use crate::cst::{char::create_cst_modulo_atom, data::CstNode};

use super::generic::create_binop;

pub fn create_cst_modulo(expr: &str) -> Either<&str, (CstNode, &str)> {
    create_binop(expr, create_cst_modulo_atom)
}

#[cfg(test)]
mod tests {
    mod modulo {
        use crate::cst::binop::{
            generic::tests::generic::test_binop_abstract, modulo::create_cst_modulo,
        };

        #[test]
        pub fn test_basic_modulo() {
            test_binop_abstract(
                create_cst_modulo,
                '%',
                "test_basic_modulo",
                2,
                vec![1.0, 2.0],
                "1%2",
            );
        }

        #[test]
        pub fn test_spaced_modulo() {
            test_binop_abstract(
                create_cst_modulo,
                '%',
                "test_spaced_modulo",
                2,
                vec![1.0, 2.0],
                "1 % 2",
            );
        }

        #[test]
        pub fn test_chained_modulo() {
            test_binop_abstract(
                create_cst_modulo,
                '%',
                "test_chained_modulo",
                3,
                vec![1.0, 2.0, 3.0],
                "1 % 2 % 3",
            );
        }
    }
}
