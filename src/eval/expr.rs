use either::Either;

use crate::ast::data::{AstBinop, AstNode, Binop};

pub fn eval_ast_binop(binop: AstBinop) -> Either<&'static str, f64> {
    let values = binop.values;
    let calc_function = match binop.op {
        Binop::Add => |a, b| a + b,
        Binop::Sub => |a, b| a - b,
        Binop::Mul => |a, b| a * b,
        Binop::Div => |a, b| a / b,
    };
    let cloned_first_value = values[0].clone();
    let mut result = match eval_expr(cloned_first_value) {
        Either::Right(v) => v,
        Either::Left(err) => return Either::Left(err),
    };

    for (i, value) in values.into_iter().enumerate() {
        if i == 0 {
            continue;
        }
        let value = match eval_expr(value) {
            Either::Right(v) => v,
            Either::Left(err) => return Either::Left(err),
        };
        result = calc_function(result, value);
    }
    Either::Right(result)
}

pub fn eval_expr(ast: AstNode) -> Either<&'static str, f64> {
    match ast {
        AstNode::Number(n) => Either::Right(n),
        AstNode::Binop(binop) => eval_ast_binop(binop),
    }
}
