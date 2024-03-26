mod cst;
mod ast;
mod tests;

use std::env;

use cst::parser::gen::generate_parser;

use crate::ast::parse::create_ast;

fn main() {
    let first_arg = match env::args().nth(1) {
        Some(arg) => arg,
        None => {
            println!("No file provided");
            return;
        }
    };

    println!("Generating parser from file: {}", first_arg);

    let (mut parser, env) = match generate_parser(&first_arg) {
        Ok(parser) => parser,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };
    let test_string = "fn add(int x, int y) -> int { return x + y + 3; }}".to_string();

    println!("parser {}", parser.to_string());
    println!("env {}", env.to_string());
    let parser = match parser.parse(&test_string, &env) {
        Ok((node, rest)) => {
            println!("Parsed: {} rest {}", node.to_string(), rest);
            node
        },
        Err(err) => {
            println!("Error: {}", err);
            return;
        },
    };
    let ast = match create_ast(parser) {
        Ok(ast) => ast,
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };
    println!("ast {:?}", ast);
}
