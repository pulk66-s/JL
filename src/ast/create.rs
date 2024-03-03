use std::vec;

use either::Either::{self, Left, Right};

use crate::cst::data::{CstAtom, CstBinop, CstFunctionDecl, CstFunctionDeclArgs, CstFunctionLine, CstFunctionLineExpr, CstNode};

use super::data::{AstBinop, AstFunctionDecl, AstFunctionDeclArg, AstNode, AstType, Binop};

fn create_ast_from_atom(atom: CstAtom) -> Either<&'static str, AstNode> {
    match atom {
        CstAtom::NUMBER(n) => Right(AstNode::Number(n)),
        _ => Left("Unknown atom."),
    }
}

fn convert_op_sign(op: char) -> Option<Binop> {
    match op {
        '+' => Some(Binop::Add),
        '-' => Some(Binop::Sub),
        '*' => Some(Binop::Mul),
        '/' => Some(Binop::Div),
        _ => None,
    }
}

fn convert_values(values: Box<Vec<CstNode>>) -> Either<&'static str, Vec<AstNode>> {
    let ast_values = values.into_iter().map(|v| create_ast(v));
    let mut unpacked_values = Vec::new();

    for value in ast_values {
        match value {
            Right(ast) => unpacked_values.push(ast),
            Left(err) => return Left(err),
        }
    }
    Right(unpacked_values)
}

fn create_ast_from_binop(binop: CstBinop) -> Either<&'static str, AstNode> {
    match convert_op_sign(binop.op) {
        Some(op) => Right(AstNode::Binop(AstBinop {
            op: op,
            values: match convert_values(binop.values) {
                Right(v) => Box::new(v),
                Left(err) => return Left(err),
            },
        })),
        None => return Left("Unknown operator."),
    }
}

fn create_ast_function_decl_args(
    args: CstFunctionDeclArgs,
) -> Either<&'static str, Vec<AstFunctionDeclArg>> {
    let mut ast_args = vec![];

    for arg in args.arg_chains {
        ast_args.push(AstFunctionDeclArg {
            arg_type: AstType::Number,
            name: match arg.name {
                CstAtom::IDENTIFIER(name) => name,
                _ => return Left("Function argument name must be an identifier."),
            },
        });
    }
    ast_args.push(AstFunctionDeclArg {
        arg_type: AstType::Number,
        name: match args.last_arg.name {
            CstAtom::IDENTIFIER(name) => name,
            _ => return Left("Function argument name must be an identifier."),
        },
    });
    Right(ast_args)
}

fn create_ast_function_body(body: Vec<CstFunctionLine>) -> Either<&'static str, Vec<AstNode>> {
    let mut ast_body = vec![];

    for line in body {
        match line.expr {
            CstFunctionLineExpr::LINE(line) => ast_body.push(match create_ast(*line.expr) {
                Right(ast) => ast,
                Left(err) => return Left(err),
            }),
            CstFunctionLineExpr::RETURN(ret) => ast_body.push(match create_ast(*ret.value) {
                Right(ast) => ast,
                Left(err) => return Left(err),
            }),
        }
    }
    Right(ast_body)
}

fn create_ast_function_decl(decl: CstFunctionDecl) -> Either<&'static str, AstNode> {
    Right(AstNode::FunctionDecl(AstFunctionDecl {
        name: match decl.name {
            CstAtom::IDENTIFIER(name) => name,
            _ => return Left("Function name must be an identifier."),
        },
        args: match decl.args {
            Some(args) => match create_ast_function_decl_args(args) {
                Right(a) => a,
                Left(err) => return Left(err),
            },
            None => vec![],
        },
        return_type: AstType::Number,
        body: match create_ast_function_body(decl.body) {
            Right(b) => b,
            Left(err) => return Left(err),
        },
    }))
}

fn create_ast_function_line_expr(line: CstFunctionLineExpr) -> Either<&'static str, AstNode> {
    match line {
        CstFunctionLineExpr::LINE(line) => create_ast(*line.expr),
        CstFunctionLineExpr::RETURN(ret) => create_ast(*ret.value),
    }
}

pub fn create_ast(cst: CstNode) -> Either<&'static str, AstNode> {
    match cst {
        CstNode::ATOM(atom) => create_ast_from_atom(atom),
        CstNode::BINOP(binop) => create_ast_from_binop(binop),
        CstNode::FUNCTION_DECL(decl) => create_ast_function_decl(decl),
        CstNode::FUNCTION_CALL(_) => Left("Function calls are not supported."),
        CstNode::FUNCTION_LINE_EXPR(line) => create_ast_function_line_expr(line),
    }
}
