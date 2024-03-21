#[cfg(test)]
pub mod tests_digit {
    use crate::cst::parser::gen::generate_parser;

    #[test]
    fn test_zero() {
        let (mut parser, env) = generate_parser("src/tests/grammar/zero.grammar").unwrap();
        let test = "0".to_string();

        match parser.parse(&test, &env) {
            Ok(_) => {}
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_boolean() {
        let (mut parser, env) = generate_parser("src/tests/grammar/boolean.grammar").unwrap();
        let test1 = "0".to_string();
        let test2 = "1".to_string();

        match (parser.parse(&test1, &env), parser.parse(&test2, &env)) {
            (Ok(_), Ok(_)) => {}
            (Err(e), _) => panic!("Error: {}", e),
            (_, Err(e)) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_digit() {
        let (mut parser, env) = generate_parser("src/tests/grammar/number.grammar").unwrap();
        let tests = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

        for test in tests {
            match parser.parse(&test.to_string(), &env) {
                Ok(_) => {}
                Err(e) => panic!("Error: {}", e),
            }
        }
    }
}
