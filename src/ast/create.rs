use either::Either::{self, Left, Right};

use crate::cst::data::{CstAtom, CstBinop, CstNode};

use super::data::{AstBinop, AstNode, Binop};

fn create_ast_from_atom(atom: CstAtom) -> Either<&'static str, AstNode> {
    match atom {
        CstAtom::NUMBER(n) => Right(AstNode::Number(n)),
        _ => Left("Unknown atom."),
    }
}

fn convert_op_sign(op: char) -> Option<Binop> {
    match op {
        '+' => Some(Binop::Add),
        '-' => Some(Binop::Sub),
        '*' => Some(Binop::Mul),
        '/' => Some(Binop::Div),
        _ => None,
    }
}

fn convert_values(values: Box<Vec<CstNode>>) -> Either<&'static str, Vec<AstNode>> {
    let ast_values = values.into_iter().map(|v| create_ast(v));
    let mut unpacked_values = Vec::new();

    for value in ast_values {
        match value {
            Right(ast) => unpacked_values.push(ast),
            Left(err) => return Left(err)
        }
    }
    Right(unpacked_values)
}

fn create_ast_from_binop(binop: CstBinop) -> Either<&'static str, AstNode> {
    match convert_op_sign(binop.op) {
        Some(op) => Right(AstNode::Binop(AstBinop {
            op: op,
            values: match convert_values(binop.values) {
                Right(v) => Box::new(v),
                Left(err) => return Left(err),
            }
        })),
        None => return Left("Unknown operator."),
    }
}

pub fn create_ast(cst: CstNode) -> Either<&'static str, AstNode> {
    match cst {
        CstNode::ATOM(atom) => create_ast_from_atom(atom),
        CstNode::BINOP(binop) => create_ast_from_binop(binop),
    }
}
