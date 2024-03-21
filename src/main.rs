mod cst;
mod tests;

use std::env;

use cst::parser::gen::generate_parser;

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
    let test_string = "int main".to_string();

    println!("parser {}", parser.to_string());
    println!("env {}", env.to_string());
    match parser.parse(&test_string, &env) {
        Ok((node, rest)) => println!("Parsed: {} rest {}", node.to_string(), rest),
        Err(err) => println!("Error: {}", err),
    }
}
