use either::Either::{self, Left, Right};

use crate::ast::data::{AstBinop, AstFunctionDecl, AstNode, Binop};

use super::env::data::Env;

pub fn eval_ast_binop(binop: AstBinop, env: &mut Env) -> Either<&'static str, (f64, &mut Env)> {
    let values = binop.values;
    let calc_function = match binop.op {
        Binop::Add => |a, b| a + b,
        Binop::Sub => |a, b| a - b,
        Binop::Mul => |a, b| a * b,
        Binop::Div => |a, b| a / b,
    };
    let cloned_first_value = values[0].clone();
    let (mut result, mut env) = match eval_expr(cloned_first_value, env) {
        Either::Right(v) => v,
        Either::Left(err) => return Either::Left(err),
    };

    for (i, value) in values.into_iter().enumerate() {
        if i == 0 {
            continue;
        }
        let (value, new_env) = match eval_expr(value, env) {
            Either::Right(v) => v,
            Either::Left(err) => return Either::Left(err),
        };
        result = calc_function(result, value);
        env = new_env;
    }
    Either::Right((result, env))
}

fn eval_function_decl(
    decl: AstFunctionDecl,
    env: &mut Env,
) -> Either<&'static str, (f64, &mut Env)> {
    env.add_function(decl.name);
    Right((0.0, env))
}

pub fn eval_expr(ast: AstNode, env: &mut Env) -> Either<&'static str, (f64, &mut Env)> {
    match ast {
        AstNode::Number(n) => Either::Right((n, env)),
        AstNode::Binop(binop) => eval_ast_binop(binop, env),
        AstNode::FunctionDecl(decl) => eval_function_decl(decl, env),
        AstNode::FunctionLine(_) => Either::Right((0.0, env))
    }
}
