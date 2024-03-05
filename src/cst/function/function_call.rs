use either::Either::{self, Left, Right};

use crate::cst::{
    char::{create_cst_closepar_atom, create_cst_comma_atom, create_cst_openpar_atom},
    data::{CstAtom, CstFunctionCall, CstFunctionCallArgChain, CstFunctionCallArgs, CstNode},
    expr::create_cst_value_expr,
    keyword::{create_cst_identifier, create_cst_spaces},
};

fn create_cst_function_call_chained_args(expr: &str) -> (Vec<CstFunctionCallArgChain>, &str) {
    let mut args = Vec::new();
    let mut new_expr = expr;

    loop {
        let temp_expr = new_expr;
        let (_, temp_expr) = match create_cst_spaces(temp_expr) {
            Left(_) => (CstAtom::CHAR(' '), temp_expr),
            Right(r) => r,
        };
        let (arg, temp_expr) = match create_cst_value_expr(temp_expr) {
            Left(_) => break,
            Right(r) => r,
        };
        let (_, temp_expr) = match create_cst_spaces(temp_expr) {
            Left(_) => (CstAtom::CHAR(' '), temp_expr),
            Right(r) => r,
        };
        let (comma, temp_expr) = match create_cst_comma_atom(temp_expr) {
            Left(_) => break,
            Right(r) => r,
        };

        new_expr = temp_expr;
        args.push(CstFunctionCallArgChain {
            arg: Box::new(arg),
            comma: comma,
        });
    }
    (args, new_expr)
}

fn create_cst_function_call_args(expr: &str) -> (Option<CstFunctionCallArgs>, &str) {
    let (chained_args, new_expr) = create_cst_function_call_chained_args(expr);
    let (_, new_expr) = match create_cst_spaces(new_expr) {
        Left(_) => (CstAtom::CHAR(' '), new_expr),
        Right(r) => r,
    };
    let (last_arg, new_expr) = match create_cst_value_expr(new_expr) {
        Left(_) => (None, new_expr),
        Right((node, new_epxr)) => (Some(node), new_epxr),
    };

    match last_arg {
        Some(last_arg) => (
            Some(CstFunctionCallArgs {
                arg_chain: chained_args,
                last_arg: Box::new(last_arg),
            }),
            new_expr,
        ),
        None => (None, new_expr),
    }
}

pub fn create_cst_function_call(expr: &str) -> Either<&str, (CstNode, &str)> {
    let (identifier, new_expr) = match create_cst_identifier(expr) {
        Left(identifier) => return Left(identifier),
        Right(r) => r,
    };
    let (_, new_expr) = match create_cst_spaces(new_expr) {
        Left(spaces) => return Left(spaces),
        Right(r) => r,
    };
    let (open_par, new_expr) = match create_cst_openpar_atom(new_expr) {
        Left(open_parn) => return Left(open_parn),
        Right(r) => r,
    };
    let (args, new_expr) = create_cst_function_call_args(new_expr);
    let (close_par, new_expr) = match create_cst_closepar_atom(new_expr) {
        Left(close_parn) => return Left(close_parn),
        Right(r) => r,
    };

    Right((
        CstNode::FUNCTION_CALL(CstFunctionCall {
            name: identifier,
            open_par: open_par,
            args: args,
            close_par: close_par,
        }),
        new_expr,
    ))
}
