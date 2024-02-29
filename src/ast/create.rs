use either::Either::{self, Left, Right};

use crate::cst::data::{CstAtom, CstBinop, CstNode};

use super::data::{AstBinop, AstNode, Binop};

fn create_ast_from_atom(atom: CstAtom) -> Either<AstNode, &'static str> {
    match atom {
        CstAtom::NUMBER(n) => Left(AstNode::Number(n)),
        _ => Right("Unknown atom."),
    }
}

fn create_ast_from_binop(binop: CstBinop) -> Either<AstNode, &'static str> {
    let left = create_ast(*binop.left);
    let right = create_ast(*binop.right);

    match (left, right) {
        (Left(left), Left(right)) => Left(AstNode::Binop(AstBinop {
            left: Box::new(left),
            op: match binop.op {
                '+' => Binop::Add,
                '-' => Binop::Sub,
                '*' => Binop::Mul,
                '/' => Binop::Div,
                _ => return Right("Unknown operator."),
            },
            right: Box::new(right),
        })),
        (Left(_), Right(err)) => Right(err),
        (Right(err), _) => Right(err),
    }
}

pub fn create_ast(cst: CstNode) -> Either<AstNode, &'static str> {
    match cst {
        CstNode::ATOM(atom) => create_ast_from_atom(atom),
        CstNode::BINOP(binop) => create_ast_from_binop(binop),
    }
}