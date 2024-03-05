pub mod args;
pub mod body;
pub mod tests;

use either::Either::{self, Left, Right};

use crate::cst::{
    char::{
        create_cst_closebrace_atom, create_cst_closepar_atom, create_cst_openbrace_atom,
        create_cst_openpar_atom,
    },
    data::{CstFunctionDecl, CstNode},
    keyword::{
        create_cst_function_decl_keyword, create_cst_function_return_arrow, create_cst_identifier,
        create_cst_spaces,
    },
};

use self::{args::create_cst_function_decl_args, body::create_cst_function_decl_body};

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
