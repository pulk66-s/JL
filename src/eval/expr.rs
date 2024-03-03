use either::Either::{self, Left, Right};

use crate::ast::data::{
    AstBinop, AstFunctionCall, AstFunctionDecl, AstFunctionLine, AstNode, Binop,
};

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
    env.functions.add_function(decl.name, decl.body);
    Right((0.0, env))
}

fn eval_function_line(
    line: AstFunctionLine,
    env: &mut Env,
) -> Either<&'static str, (f64, &mut Env)> {
    match line {
        AstFunctionLine::Line(expr) => eval_expr(*expr, env),
        AstFunctionLine::Return(expr) => eval_expr(*expr, env),
    }
}

fn eval_function_call(
    call: AstFunctionCall,
    env: &mut Env,
) -> Either<&'static str, (f64, &mut Env)> {
    let function = match env.functions.get_function(call.name) {
        Some(f) => f,
        None => return Left("Function not found"),
    };
    let mut return_value = 0.0;
    let func = function.clone();
    let mut new_env = env;

    for line in func.body {
        let (value, temp_env) = match eval_expr(line, new_env) {
            Either::Right(v) => v,
            Either::Left(err) => return Left(err),
        };
        new_env = temp_env;
        return_value = value;
    }
    Right((return_value, new_env))
}

pub fn eval_expr(ast: AstNode, env: &mut Env) -> Either<&'static str, (f64, &mut Env)> {
    match ast {
        AstNode::Number(n) => Either::Right((n, env)),
        AstNode::Binop(binop) => eval_ast_binop(binop, env),
        AstNode::FunctionDecl(decl) => eval_function_decl(decl, env),
        AstNode::FunctionLine(line) => eval_function_line(line, env),
        AstNode::FunctionCall(call) => eval_function_call(call, env),
    }
}
