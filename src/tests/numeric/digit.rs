#[cfg(test)]
pub mod tests_digit {
    use crate::cst::parser::gen::generate_parser;

    #[test]
    fn test_zero() {
        let (parser, env) = generate_parser("src/tests/grammar/zero.grammar").unwrap();
        let test = "0".to_string();

        match parser.parse(&test, &env) {
            Ok(_) => {}
            Err(e) => panic!("Error: {}", e),
        }
    }
}
