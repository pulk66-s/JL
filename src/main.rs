mod cst;
mod ast;
mod tests;
mod comp;

use std::{env, fs::read_to_string};

use cst::parser::gen::generate_parser;

use crate::{ast::parse::create_ast, comp::llvm::compile_with_llvm};

fn main() {
    let first_arg = match env::args().nth(1) {
        Some(arg) => arg,
        None => {
            println!("No file provided");
            return;
        }
    };
    let other_args = env::args().skip(2).collect::<Vec<String>>();

    println!("Generating parser from file: {}", first_arg);
    println!("Other args: {:?}", other_args);

    let (mut parser, env) = match generate_parser(&first_arg) {
        Ok(parser) => parser,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };
    let file = match other_args.get(0) {
        Some(file) => file,
        None => {
            println!("No file provided");
            return;
        }
    };
    let file_content = match read_to_string(file) {
        Ok(content) => content,
        Err(err) => {
            println!("Error reading file: {}", err);
            return;
        }
    };

    println!("parser {}", parser.to_string());
    println!("env {}", env.to_string());
    let parser = match parser.parse(&file_content, &env) {
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
    match compile_with_llvm(ast) {
        Ok(r) => println!("Result: {}", r),
        Err(e) => {
            println!("Err {}", e);
            return;
        }
    };
}
