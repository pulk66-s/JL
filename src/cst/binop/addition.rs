use either::Either;

use crate::cst::{char::create_cst_add_atom, data::CstNode};

use super::generic::create_binop;

pub fn create_cst_addition(expr: &str) -> Either<&str, (CstNode, &str)> {
    create_binop(expr, create_cst_add_atom)
}

#[cfg(test)]
mod tests {

    mod addition {
        use either::Either::{Left, Right};

        use crate::cst::binop::{
            addition::create_cst_addition,
            expr::tests::exprs::{atom_match_char, node_match_binop, node_match_float},
        };

        pub fn test_addition_abstract(expr: &str, from: &str, nb_values: usize, values: Vec<f64>) {
            let (node, rest) = match create_cst_addition(expr) {
                Left(err) => panic!("{}", err),
                Right(r) => r,
            };

            assert_eq!(rest, "");

            let binop = node_match_binop(&node, &format!("{}: Expected a binop node.", from));

            atom_match_char(&binop.op, '+', &format!("{}: Expected a '+' atom.", from));
            assert_eq!(binop.values.len(), nb_values);

            for (i, value) in values.iter().enumerate() {
                node_match_float(
                    &binop.values[i],
                    *value,
                    &format!("{}: Expected a float atom.", from),
                );
            }
        }

        #[test]
        pub fn test_basic_addition() {
            test_addition_abstract("1+2", "test_basic_addition", 2, vec![1.0, 2.0]);
        }

        #[test]
        pub fn test_spaced_addition() {
            test_addition_abstract("1 + 2", "test_spaced_addition", 2, vec![1.0, 2.0]);
        }

        #[test]
        pub fn test_chained_addition() {
            test_addition_abstract("1 + 2 + 3", "test_chained_addition", 3, vec![1.0, 2.0, 3.0]);
        }
    }
}
