mod cst;

use std::env;

use cst::parser::gen::generate_parser;

fn main() {
    let first_arg = match env::args().nth(1) {
        Some(arg) => arg,
        None => {
            println!("No file provided");
            return
        }
    };

    println!("Generating parser from file: {}", first_arg);

    let (parser, env) = match generate_parser(&first_arg) {
        Ok(parser) => parser,
        Err(err) => {
            println!("{}", err);
            return
        }
    };
    let test_string = "0".to_string();

    match parser.parse(&test_string, &env) {
        Ok(node) => println!("Parsed: {}", node.to_string()),
        Err(err) => println!("Error: {}", err),
    }
}
