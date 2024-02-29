use std::io::Write;

use either::Either::{Left, Right};

use crate::{ast::create::create_ast, cst::create::create_cst_from_string, eval::expr::eval_expr};

fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

fn eval_command(cmd: &str) {
    println!("Evaluating command: {}", cmd);
    let (cst, _) = match create_cst_from_string(cmd) {
        Left(err) => {
            println!("Error: {}", err);
            return;
        },
        Right(cst) => cst,
    };
    println!("CST: {:?}", cst);
    let ast = match create_ast(cst) {
        Left(ast) => ast,
        Right(err) => {
            println!("Error: {}", err);
            return;
        },
    };
    println!("AST: {:?}", ast);
    let eval = match eval_expr(ast) {
        Left(err) => {
            println!("Error: {}", err);
            return;
        },
        Right(eval) => eval,
    };
    println!("{}", eval);
}

pub fn check_command(input: &str) {
    match input {
        "exit" => std::process::exit(0),
        cmd => eval_command(cmd)
    }
}

pub fn launch_tty() {
    println!("Launching terminal...");
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        let input = get_input();
        println!("You entered: {}", input.trim());
        check_command(input.trim());
    }
}
