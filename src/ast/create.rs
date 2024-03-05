use std::vec;

use either::Either::{self, Left, Right};

use crate::cst::data::{
    CstAtom, CstBinop, CstCondition, CstFunctionCall, CstFunctionDecl, CstFunctionDeclArgs,
    CstFunctionLineExpr, CstNode, CstVariableDecl,
};

use super::data::{
    AstBinop, AstCondition, AstFunctionCall, AstFunctionDecl, AstFunctionDeclArg, AstNode, AstType,
    AstVariableDecl, Binop,
};

fn create_ast_from_atom(atom: CstAtom) -> Either<&'static str, AstNode> {
    match atom {
        CstAtom::NUMBER(n) => Right(AstNode::Number(n)),
        CstAtom::IDENTIFIER(name) => Right(AstNode::Identifier(name)),
        _ => Left("Unknown atom"),
    }
}

fn convert_op_sign(op: CstAtom) -> Option<Binop> {
    match op {
        CstAtom::CHAR(op) => match op {
            '+' => Some(Binop::Add),
            '-' => Some(Binop::Sub),
            '*' => Some(Binop::Mul),
            '/' => Some(Binop::Div),
            _ => None,
        },
        CstAtom::KEYWORD(op) => match op.as_str() {
            "==" => Some(Binop::Eq),
            ">=" => Some(Binop::Ge),
            "<=" => Some(Binop::Le),
            ">" => Some(Binop::Gt),
            "<" => Some(Binop::Lt),
            _ => None,
        },
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
                CstAtom::IDENTIFIER(name) => Box::new(AstNode::Identifier(name)),
                _ => return Left("Function argument name must be an identifier."),
            },
        });
    }
    ast_args.push(AstFunctionDeclArg {
        arg_type: AstType::Number,
        name: match args.last_arg.name {
            CstAtom::IDENTIFIER(name) => Box::new(AstNode::Identifier(name)),
            _ => return Left("Function argument name must be an identifier."),
        },
    });
    Right(ast_args)
}

fn create_ast_function_body(body: Vec<CstFunctionLineExpr>) -> Either<&'static str, Vec<AstNode>> {
    let mut ast_body = vec![];

    for line in body {
        match line {
            CstFunctionLineExpr::LINE(line) => ast_body.push(match create_ast(*line.expr) {
                Right(ast) => ast,
                Left(err) => return Left(err),
            }),
            CstFunctionLineExpr::RETURN(ret) => ast_body.push(match create_ast(*ret.value) {
                Right(ast) => ast,
                Left(err) => return Left(err),
            }),
            CstFunctionLineExpr::CONDITION(cond) => ast_body.push(match create_ast_condition(cond) {
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
            CstAtom::IDENTIFIER(name) => Box::new(AstNode::Identifier(name)),
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
        CstFunctionLineExpr::CONDITION(cond) => {
            let s = "Condition not implemented yet 2.";

            Left(s)
        }
    }
}

fn create_ast_function_call(call: CstFunctionCall) -> Either<&'static str, AstNode> {
    let name = match call.name {
        CstAtom::IDENTIFIER(name) => name,
        _ => return Left("Function name must be an identifier."),
    };
    let mut ast_args = vec![];

    match call.args {
        None => (),
        Some(args) => {
            for arg in args.arg_chain {
                match create_ast(*arg.arg) {
                    Right(ast) => ast_args.push(ast),
                    Left(err) => return Left(err),
                }
            }
            match create_ast(*args.last_arg) {
                Right(ast) => ast_args.push(ast),
                Left(err) => return Left(err),
            }
        }
    }
    Right(AstNode::FunctionCall(AstFunctionCall {
        name: Box::new(AstNode::Identifier(name)),
        args: ast_args,
    }))
}

fn create_ast_variable_decl(decl: CstVariableDecl) -> Either<&'static str, AstNode> {
    let var_type = AstType::Number;
    let name = match decl.name {
        CstAtom::IDENTIFIER(name) => name,
        _ => return Left("Variable name must be an identifier."),
    };
    let value = match create_ast(*decl.value) {
        Right(ast) => ast,
        Left(err) => return Left(err),
    };

    Right(AstNode::VariableDecl(AstVariableDecl {
        var_type: var_type,
        name: Box::new(AstNode::Identifier(name)),
        value: Box::new(value),
    }))
}

fn create_ast_condition(cond: CstCondition) -> Either<&'static str, AstNode> {
    let condition = match create_ast(*cond.condition) {
        Right(ast) => ast,
        Left(err) => return Left(err),
    };
    let mut ast_body = vec![];
    let cst_body = cond.body;

    for line in cst_body {
        match line {
            CstFunctionLineExpr::LINE(line) => match create_ast(*line.expr) {
                Right(ast) => ast_body.push(ast),
                Left(err) => return Left(err),
            },
            CstFunctionLineExpr::RETURN(ret) => match create_ast(*ret.value) {
                Right(ast) => ast_body.push(ast),
                Left(err) => return Left(err),
            },
            CstFunctionLineExpr::CONDITION(cond) => {
                let s = "Condition not implemented yet 3.";

                return Left(s);
            }
        }
    }
    Right(AstNode::Condition(AstCondition {
        condition: Box::new(condition),
        body: ast_body,
        else_condition: None,
    }))
}

pub fn create_ast(cst: CstNode) -> Either<&'static str, AstNode> {
    match cst {
        CstNode::ATOM(atom) => create_ast_from_atom(atom),
        CstNode::BINOP(binop) => create_ast_from_binop(binop),
        CstNode::FUNCTION_DECL(decl) => create_ast_function_decl(decl),
        CstNode::FUNCTION_CALL(call) => create_ast_function_call(call),
        CstNode::FUNCTION_LINE(line) => create_ast_function_line_expr(line),
        CstNode::VARIABLE_DECL(decl) => create_ast_variable_decl(decl),
        CstNode::CONDITION(cond) => create_ast_condition(cond),
    }
}
