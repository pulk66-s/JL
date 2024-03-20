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

    #[test]
    fn test_boolean() {
        let (parser, env) = generate_parser("src/tests/grammar/boolean.grammar").unwrap();
        let test1 = "0".to_string();
        let test2 = "1".to_string();

        match (parser.parse(&test1, &env), parser.parse(&test2, &env)) {
            (Ok(_), Ok(_)) => {}
            (Err(e), _) => panic!("Error: {}", e),
            (_, Err(e)) => panic!("Error: {}", e),
        }
    }
}
