use std::vec;

use either::Either::{self, Left, Right};

use crate::cst::char::{create_cst_closebrace_atom, create_cst_openbrace_atom};

use super::{
    char::{
        create_cst_closepar_atom, create_cst_comma_atom,
        create_cst_openpar_atom,
    },
    data::{
        CstAtom, CstFunctionChainArg, CstFunctionDecl, CstFunctionDeclArg, CstFunctionDeclArgs, CstFunctionLineExpr, CstNode
    },
    expr::create_cst_function_expr,
    keyword::{
        create_cst_function_decl_keyword, create_cst_function_return_arrow, create_cst_identifier,
        create_cst_spaces,
    },
};

fn create_cst_function_decl_arg(expr: &str) -> Either<&str, (CstFunctionDeclArg, &str)> {
    let (arg_type, new_expr) = match create_cst_identifier(expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };
    let (_, new_expr) = match create_cst_spaces(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };
    let (arg_name, new_expr) = match create_cst_identifier(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };

    Right((
        CstFunctionDeclArg {
            arg_type: arg_type,
            name: arg_name,
        },
        new_expr,
    ))
}

fn create_cst_function_decl_chained_args(expr: &str) -> (Vec<CstFunctionChainArg>, &str) {
    let mut args = vec![];
    let mut new_expr = expr;
    let mut name;
    let mut arg_type;
    let mut comma;

    loop {
        let mut temp_expr = new_expr;
        (_, temp_expr) = match create_cst_spaces(temp_expr) {
            Left(_) => break,
            Right(r) => r,
        };
        (arg_type, temp_expr) = match create_cst_identifier(temp_expr) {
            Left(_) => break,
            Right(r) => r,
        };
        (_, temp_expr) = match create_cst_spaces(temp_expr) {
            Left(_) => break,
            Right(r) => r,
        };
        (name, temp_expr) = match create_cst_identifier(temp_expr) {
            Left(_) => break,
            Right(r) => r,
        };
        (_, temp_expr) = match create_cst_spaces(temp_expr) {
            Left(_) => (CstAtom::CHAR(' '), temp_expr),
            Right(r) => r,
        };
        (comma, temp_expr) = match create_cst_comma_atom(temp_expr) {
            Left(_) => break,
            Right(r) => r,
        };

        args.push(CstFunctionChainArg {
            arg_type: arg_type,
            name: name,
            comma: comma,
        });
        new_expr = temp_expr;
    }
    (args, new_expr)
}

fn create_cst_function_decl_args(expr: &str) -> Either<&str, (CstFunctionDeclArgs, &str)> {
    let (chained_args, new_expr) = create_cst_function_decl_chained_args(expr);
    let (_, new_expr) = match create_cst_spaces(new_expr) {
        Left(_) => (CstAtom::CHAR(' '), new_expr),
        Right(r) => r,
    };
    let (last_arg, new_expr) = match create_cst_function_decl_arg(new_expr) {
        Left(err) => return Left(err),
        Right((arg, new_expr)) => (arg, new_expr),
    };

    Right((
        CstFunctionDeclArgs {
            arg_chains: chained_args,
            last_arg: last_arg,
        },
        new_expr,
    ))
}

pub fn create_cst_function_decl_body(expr: &str) -> (Vec<CstFunctionLineExpr>, &str) {
    let mut lines = vec![];
    let mut line;
    let mut new_expr = expr;

    loop {
        (_, new_expr) = match create_cst_spaces(new_expr) {
            Left(_) => break,
            Right(r) => r,
        };
        (line, new_expr) = match create_cst_function_expr(new_expr) {
            Left(_) => break,
            Right(r) => r,
        };
        (_, new_expr) = match create_cst_spaces(new_expr) {
            Left(_) => break,
            Right(r) => r,
        };
        lines.push(match line {
            CstNode::FUNCTION_LINE(line) => line,
            _ => break
        });
    }
    (lines, new_expr)
}

pub fn create_cst_function_decl(expr: &str) -> Either<&str, (CstNode, &str)> {
    let (keyword, new_expr) = match create_cst_function_decl_keyword(expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };
    let (_, new_expr) = match create_cst_spaces(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };
    let (name, new_expr) = match create_cst_identifier(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };
    let (open_par, new_expr) = match create_cst_openpar_atom(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };
    let (args, new_expr) = match create_cst_function_decl_args(new_expr) {
        Left(_) => (None, new_expr),
        Right((atom, new_expr)) => (Some(atom), new_expr),
    };
    let (close_par, new_expr) = match create_cst_closepar_atom(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };
    let (_, new_expr) = match create_cst_spaces(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };
    let (return_arrow, new_expr) = match create_cst_function_return_arrow(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };
    let (_, new_expr) = match create_cst_spaces(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };
    let (return_type, new_expr) = match create_cst_identifier(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };
    let (_, new_expr) = match create_cst_spaces(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };
    let (open_brace, new_expr) = match create_cst_openbrace_atom(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };
    let (body, new_expr) = create_cst_function_decl_body(new_expr);
    let (close_brace, new_expr) = match create_cst_closebrace_atom(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r,
    };

    Right((
        CstNode::FUNCTION_DECL(CstFunctionDecl {
            keyword: keyword,
            open_par: open_par,
            name: name,
            args: args,
            close_par: close_par,
            return_arrow: return_arrow,
            return_type: return_type,
            open_brace: open_brace,
            body: body,
            close_brace: close_brace,
        }),
        new_expr,
    ))
}
