use either::Either::{self, Left, Right};

use super::{
    char::create_cst_add_atom,
    data::{
        CstAtom, CstBinop, CstNode
    },
    number::create_cst_number,
    string::create_cst_spaces
};

pub fn create_cst_addition(expr: &str) -> Either<&str, (CstNode, &str)> {
    let (left_number, new_expr) = match create_cst_number(expr) {
        Left(err) => return Left(err),
        Right(r) => r
    };
    let (_, new_expr) = match create_cst_spaces(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r
    };
    let (op_atom, new_expr) = match create_cst_add_atom(new_expr) {
        Left(err) => return Left(err),
        Right((CstAtom::CHAR(c), expr)) => (c, expr),
        Right(_) => return Left("create_cst_add_atom return a non-char atom.")
    };
    let (_, new_expr) = match create_cst_spaces(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r
    };
    let (right_number, new_expr) = match create_cst_number(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r
    };

    Right((CstNode::BINOP(CstBinop {
        op: op_atom,
        left: Box::new(CstNode::ATOM(left_number)),
        right: Box::new(CstNode::ATOM(right_number)),
    }), new_expr))
}
