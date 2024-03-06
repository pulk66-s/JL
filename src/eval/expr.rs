use either::Either::{self, Left, Right};

use crate::{ast::data::{
    AstBinop, AstCondition, AstFunctionCall, AstFunctionDecl, AstFunctionLine, AstNode, AstVariableDecl, Binop
}, main};

use super::env::data::Env;

pub fn eval_ast_binop(binop: AstBinop, env: &mut Env) -> Either<&'static str, (f64, &mut Env)> {
    let values = binop.values;
    let calc_function = match binop.op {
        Binop::Add => |a, b| a + b,
        Binop::Sub => |a, b| a - b,
        Binop::Mul => |a, b| a * b,
        Binop::Div => |a, b| a / b,
        Binop::Eq => |a, b| if a == b { 1.0 } else { 0.0 },
        Binop::Gt => |a, b| if a > b { 1.0 } else { 0.0 },
        Binop::Ge => |a, b| if a >= b { 1.0 } else { 0.0 },
        Binop::Lt => |a, b| if a < b { 1.0 } else { 0.0 },
        Binop::Le => |a, b| if a <= b { 1.0 } else { 0.0 },
        Binop::Mod => |a, b| a % b,
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
    let name = match *decl.name {
        AstNode::Identifier(name) => name,
        _ => return Left("Function name must be an identifier."),
    };

    env.functions.add_function(name, decl.body);
    Right((0.0, env))
}

fn eval_lines(
    lines: Vec<AstNode>,
    env: &mut Env,
) -> Either<&'static str, (f64, &mut Env)> {
    let mut result = 0.0;
    let mut new_env = env;

    for line in lines {
        let (value, temp_env) = match eval_expr(line, new_env) {
            Either::Right(v) => v,
            Either::Left(err) => return Either::Left(err),
        };
        new_env = temp_env;
        result = value;
    }
    Either::Right((result, new_env))
}

fn eval_function_call(
    call: AstFunctionCall,
    env: &mut Env,
) -> Either<&'static str, (f64, &mut Env)> {
    let name = match *call.name {
        AstNode::Identifier(name) => name,
        _ => return Left("Function name must be an identifier."),
    };
    let function = match env.functions.get_function(name) {
        Some(f) => f,
        None => return Left("Function not found"),
    };
    let func = function.clone();

    eval_lines(func.body, env)
}

fn eval_variable_decl(
    decl: AstVariableDecl,
    env: &mut Env,
) -> Either<&'static str, (f64, &mut Env)> {
    let (value, env) = match eval_expr(*decl.value, env) {
        Either::Right(v) => v,
        Either::Left(err) => return Left(err),
    };
    let name = match *decl.name {
        AstNode::Identifier(name) => name,
        _ => return Left("Variable name must be an identifier."),
    };

    env.variables.add_variable(name, value);
    Right((value, env))
}

fn eval_ast_identifier(name: String, env: &mut Env) -> Either<&'static str, (f64, &mut Env)> {
    match env.variables.get_variable(name) {
        Some(v) => Either::Right((v.value, env)),
        None => Either::Left("Variable not found"),
    }
}

fn eval_condition(cond: AstCondition, env: &mut Env) -> Either<&'static str, (f64, &mut Env)> {
    let mut result = 0.0;
    let (condition_result, env) = match eval_expr(*cond.condition, env) {
        Either::Right(v) => v,
        Either::Left(err) => return Either::Left(err),
    };

    if condition_result == 1.0 {
        let (value, env) = match eval_lines(cond.body, env) {
            Either::Right(v) => v,
            Either::Left(err) => return Either::Left(err),
        };

        result = value;
        return Either::Right((result, env));
    }
    Either::Right((result, env))
}

pub fn eval_expr(ast: AstNode, env: &mut Env) -> Either<&'static str, (f64, &mut Env)> {
    match ast {
        AstNode::Number(n) => Either::Right((n, env)),
        AstNode::Binop(binop) => eval_ast_binop(binop, env),
        AstNode::Identifier(name) => eval_ast_identifier(name, env),
        AstNode::FunctionDecl(decl) => eval_function_decl(decl, env),
        AstNode::FunctionCall(call) => eval_function_call(call, env),
        AstNode::VariableDecl(decl) => eval_variable_decl(decl, env),
        AstNode::Condition(cond) => eval_condition(cond, env),
    }
}
