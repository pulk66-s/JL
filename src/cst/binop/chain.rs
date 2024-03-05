use either::Either::{self, Left, Right};

use crate::cst::{
    data::{CstAtom, CstNode},
    keyword::create_cst_spaces,
    number::create_cst_number,
};

pub fn create_binop_chained(
    expr: &str,
    atom_parse: fn(&str) -> Either<&str, (CstAtom, &str)>,
) -> (Vec<CstNode>, &str) {
    let mut nodes = vec![];
    let mut new_expr = expr;
    let mut number;

    loop {
        (_, new_expr) = match create_cst_spaces(new_expr) {
            Left(_) => break,
            Right(r) => r,
        };
        (_, new_expr) = match atom_parse(new_expr) {
            Left(_) => break,
            Right((CstAtom::CHAR(c), new_expr)) => (c, new_expr),
            Right(_) => break,
        };
        (_, new_expr) = match create_cst_spaces(new_expr) {
            Left(_) => break,
            Right(r) => r,
        };
        (number, new_expr) = match create_cst_number(new_expr) {
            Left(_) => break,
            Right(r) => r,
        };

        nodes.push(CstNode::ATOM(number));
    }
    (nodes, new_expr)
}
