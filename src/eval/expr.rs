use either::Either;

use crate::ast::data::{AstBinop, AstNode, Binop};

pub fn eval_ast_binop(binop: AstBinop) -> Either<&'static str, f64> {
    let left = eval_expr(*binop.left);
    let right = eval_expr(*binop.right);

    match (left, right) {
        (Either::Right(left), Either::Right(right)) => match binop.op {
            Binop::Add => Either::Right(left + right),
            Binop::Sub => Either::Right(left - right),
            Binop::Mul => Either::Right(left * right),
            Binop::Div => Either::Right(left / right),
        },
        (Either::Right(_), Either::Left(err)) => Either::Left(err),
        (Either::Left(err), _) => Either::Left(err),
    }
}

pub fn eval_expr(ast: AstNode) -> Either<&'static str, f64> {
    match ast {
        AstNode::Number(n) => Either::Right(n),
        AstNode::Binop(binop) => eval_ast_binop(binop),
    }
}
