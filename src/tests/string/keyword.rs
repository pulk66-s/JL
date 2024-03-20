#[cfg(test)]
pub mod tests_keyword {
    use crate::cst::parser::gen::generate_parser;

    #[test]
    fn test_keyword() {
        let (parser, env) = generate_parser("src/tests/grammar/string.grammar").unwrap();
        let test = "if".to_string();

        match parser.parse(&test, &env) {
            Ok(_) => {}
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_char() {
        let (parser, env) = generate_parser("src/tests/grammar/char.grammar").unwrap();
        let test = "a".to_string();

        match parser.parse(&test, &env) {
            Ok(_) => {}
            Err(e) => panic!("Error: {}", e),
        }
    }
}
