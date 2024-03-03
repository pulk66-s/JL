use either::Either::{self, Left, Right};

use super::{
    char::create_cst_equal_atom,
    data::{CstNode, CstVariableDecl},
    expr::create_cst_value_expr,
    keyword::{create_cst_identifier, create_cst_spaces, create_cst_variable_decl_keyword},
};

pub fn create_cst_variable_decl(expr: &str) -> Either<&str, (CstNode, &str)> {
    let (keyword, new_expr) = match create_cst_variable_decl_keyword(expr) {
        Left(err) => return Left(err),
        Right((keyword, new_expr)) => (keyword, new_expr),
    };
    let (_, new_expr) = match create_cst_spaces(new_expr) {
        Left(err) => return Left(err),
        Right((_, new_expr)) => ((), new_expr),
    };
    let (var_type, new_expr) = match create_cst_identifier(new_expr) {
        Left(err) => return Left(err),
        Right((var_type, new_expr)) => (var_type, new_expr),
    };
    let (_, new_expr) = match create_cst_spaces(new_expr) {
        Left(err) => return Left(err),
        Right((_, new_expr)) => ((), new_expr),
    };
    let (name, new_expr) = match create_cst_identifier(new_expr) {
        Left(err) => return Left(err),
        Right((name, new_expr)) => (name, new_expr),
    };
    let (_, new_expr) = match create_cst_spaces(new_expr) {
        Left(err) => return Left(err),
        Right((_, new_expr)) => ((), new_expr),
    };
    let (equal, new_expr) = match create_cst_equal_atom(new_expr) {
        Left(err) => return Left(err),
        Right((equal, new_expr)) => (equal, new_expr),
    };
    let (_, new_expr) = match create_cst_spaces(new_expr) {
        Left(err) => return Left(err),
        Right((_, new_expr)) => ((), new_expr),
    };
    let (value, new_expr) = match create_cst_value_expr(new_expr) {
        Left(err) => return Left(err),
        Right((value, new_expr)) => (value, new_expr),
    };

    Right((
        CstNode::VARIABLE_DECL(CstVariableDecl {
            keyword: keyword,
            var_type: var_type,
            name: name,
            equal: equal,
            value: Box::new(value),
        }),
        new_expr,
    ))
}
