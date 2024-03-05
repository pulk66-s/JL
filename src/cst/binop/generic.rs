use either::Either::{self, Left, Right};

use crate::cst::{
    data::{CstAtom, CstBinop, CstNode},
    expr::create_cst_atom_value_expr,
    keyword::create_cst_spaces,
    number::create_cst_number,
};

use super::chain::create_binop_chained;

pub fn create_binop(
    expr: &str,
    atom_parse: fn(&str) -> Either<&str, (CstAtom, &str)>,
) -> Either<&str, (CstNode, &str)> {
    let (left_number, new_expr) = match create_cst_number(expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };
    let (_, new_expr) = match create_cst_spaces(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };
    let (op_atom, new_expr) = match atom_parse(new_expr) {
        Left(err) => return Left(err),
        Right((CstAtom::CHAR(c), expr)) => (CstAtom::CHAR(c), expr),
        Right((CstAtom::KEYWORD(keyword), expr)) => (CstAtom::KEYWORD(keyword), expr),
        Right(_) => return Left("atom_parse return a non-char atom."),
    };
    let (_, new_expr) = match create_cst_spaces(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };
    let (right_number, new_expr) = match create_cst_atom_value_expr(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };
    let mut datas = vec![CstNode::ATOM(left_number), right_number];
    let (rest, new_expr) = create_binop_chained(new_expr, atom_parse);

    datas.extend(rest);

    let binop = CstBinop {
        op: op_atom,
        values: Box::new(datas),
    };
    Right((CstNode::BINOP(binop), new_expr))
}

#[cfg(test)]
pub mod tests {
    pub mod generic {
        use either::Either::{self, Left, Right};

        use crate::cst::{
            binop::expr::tests::exprs::{
                atom_match_char, atom_match_keyword, node_match_binop, node_match_float,
            },
            data::CstNode,
        };

        pub fn test_binop_abstract(
            function: fn(&str) -> Either<&str, (CstNode, &str)>,
            op: char,
            from: &str,
            nb_values: usize,
            values: Vec<f64>,
            expr: &str,
        ) {
            let (node, rest) = match function(expr) {
                Left(err) => panic!("{}", err),
                Right(r) => r,
            };

            assert_eq!(rest, "");

            let binop = node_match_binop(&node, &format!("{}: Expected a binop node.", from));

            atom_match_char(
                &binop.op,
                op,
                &format!("{}: Expected a '{}' atom.", from, op),
            );
            assert_eq!(binop.values.len(), nb_values);

            for (i, value) in values.iter().enumerate() {
                node_match_float(
                    &binop.values[i],
                    *value,
                    &format!("{}: Expected a float atom.", from),
                );
            }
        }

        pub fn test_binop_abstract_keyword(
            function: fn(&str) -> Either<&str, (CstNode, &str)>,
            op: &str,
            from: &str,
            nb_values: usize,
            values: Vec<f64>,
            expr: &str,
        ) {
            let (node, rest) = match function(expr) {
                Left(err) => panic!("{}", err),
                Right(r) => r,
            };

            assert_eq!(rest, "");

            let binop = node_match_binop(&node, &format!("{}: Expected a binop node.", from));

            atom_match_keyword(
                &binop.op,
                op,
                &format!("{}: Expected a '{}' atom.", from, op),
            );
            assert_eq!(binop.values.len(), nb_values);

            for (i, value) in values.iter().enumerate() {
                node_match_float(
                    &binop.values[i],
                    *value,
                    &format!("{}: Expected a float atom.", from),
                );
            }
        }
    }
}
