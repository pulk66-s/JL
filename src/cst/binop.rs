use either::Either::{self, Left, Right};

use super::{
    char::{create_cst_add_atom, create_cst_div_atom, create_cst_mul_atom, create_cst_sub_atom}, data::{
        CstAtom, CstBinop, CstNode
    }, expr::create_cst_atom_value_expr, number::create_cst_number, keyword::create_cst_spaces
};

fn create_binop_chained(
    expr: &str,
    atom_parse: fn(&str) -> Either<&str, (CstAtom, &str)>
) -> (Vec<CstNode>, &str) {
    let mut nodes = vec![];
    let mut new_expr = expr;
    let mut number;

    loop {
        (_, new_expr) = match create_cst_spaces(new_expr) {
            Left(_) => break,
            Right(r) => r
        };
        (_, new_expr) = match atom_parse(new_expr) {
            Left(_) => break,
            Right((CstAtom::CHAR(c), new_expr)) => (c, new_expr),
            Right(_) => break
        };
        (_, new_expr) = match create_cst_spaces(new_expr) {
            Left(_) => break,
            Right(r) => r
        };
        (number, new_expr) = match create_cst_number(new_expr) {
            Left(_) => break,
            Right(r) => r
        };

        nodes.push(CstNode::ATOM(number));
    }
    (nodes, new_expr)
}

fn create_binop(
    expr: &str,
    atom_parse: fn(&str) -> Either<&str, (CstAtom, &str)>
) -> Either<&str, (CstNode, &str)> {
    let (left_number, new_expr) = match create_cst_number(expr) {
        Left(err) => return Left(err),
        Right(r) => r
    };
    let (_, new_expr) = match create_cst_spaces(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r
    };
    let (op_atom, new_expr) = match atom_parse(new_expr) {
        Left(err) => return Left(err),
        Right((CstAtom::CHAR(c), expr)) => (c, expr),
        Right(_) => return Left("atom_parse return a non-char atom.")
    };
    let (_, new_expr) = match create_cst_spaces(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r
    };
    let (right_number, new_expr) = match create_cst_atom_value_expr(new_expr) {
        Left(err) => return Left(err),
        Right(r) => r
    };
    let mut datas = vec![
        CstNode::ATOM(left_number),
        right_number
    ];
    let (rest, new_expr) = create_binop_chained(new_expr, atom_parse);

    datas.extend(rest);

    let binop = CstBinop {
        op: op_atom,
        values: Box::new(datas)
    };
    Right((CstNode::BINOP(binop), new_expr))
}
    
pub fn create_cst_addition(expr: &str) -> Either<&str, (CstNode, &str)> {
    create_binop(expr, create_cst_add_atom)
}

pub fn create_cst_subtraction(expr: &str) -> Either<&str, (CstNode, &str)> {
    create_binop(expr, create_cst_sub_atom)
}

pub fn create_cst_multiplication(expr: &str) -> Either<&str, (CstNode, &str)> {
    create_binop(expr, create_cst_mul_atom)
}

pub fn create_cst_division(expr: &str) -> Either<&str, (CstNode, &str)> {
    create_binop(expr, create_cst_div_atom)
}

pub fn create_cst_binop(expr: &str) -> Either<&str, (CstNode, &str)> {
    match create_cst_addition(expr) {
        Right(r) => return Right(r),
        Left(_) => {}
    };
    match create_cst_subtraction(expr) {
        Right(r) => return Right(r),
        Left(_) => {}
    };
    match create_cst_multiplication(expr) {
        Right(r) => return Right(r),
        Left(_) => {}
    };
    match create_cst_division(expr) {
        Right(r) => return Right(r),
        Left(_) => {}
    };
    Left("No binop found.")
}
