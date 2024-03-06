#[cfg(test)]
pub mod tests {
    pub mod function {
        pub mod decl {
            use either::Either::{Left, Right};

            use crate::cst::{
                binop::expr::tests::exprs::node_match_float,
                char::tests::atom_match_char,
                data::{CstFunctionDecl, CstFunctionLineExpr, CstNode},
                function::function_decl::create_cst_function_decl,
                keyword::tests::{identifier::atom_match_identifier, keyword::atom_match_keyword},
            };

            pub fn node_match_function_decl<'a>(node: &'a CstNode) -> &'a CstFunctionDecl {
                match node {
                    CstNode::FUNCTION_DECL(n) => n,
                    _ => panic!("node_match_function_decl: node is not a function decl"),
                }
            }

            fn function_decl_abstract_test<'a>(
                (node, rest): (&'a CstNode, &'a str),
                return_type: &str,
            ) {
                assert_eq!(rest, "");

                let function_decl = node_match_function_decl(&node);

                atom_match_keyword(
                    &function_decl.keyword,
                    "fn",
                    "test_empty_function: fn keyword does not match",
                );
                atom_match_identifier(
                    &function_decl.name,
                    "test",
                    "test_empty_function: name does not match",
                );
                atom_match_char(
                    &function_decl.open_par,
                    '(',
                    "test_empty_function: open_par keyword does not match",
                );
                atom_match_char(
                    &function_decl.close_par,
                    ')',
                    "test_empty_function: close_par keyword does not match",
                );
                atom_match_keyword(
                    &function_decl.return_arrow,
                    "->",
                    "test_empty_function: return_arrow keyword does not match",
                );
                atom_match_identifier(
                    &function_decl.return_type,
                    return_type,
                    "test_empty_function: return_type does not match",
                );
                atom_match_char(
                    &function_decl.open_brace,
                    '{',
                    "test_empty_function: open_brace keyword does not match",
                );
                atom_match_char(
                    &function_decl.close_brace,
                    '}',
                    "test_empty_function: close_brace keyword does not match",
                );
            }

            #[test]
            pub fn test_empty_function() {
                let (node, rest) = match create_cst_function_decl("fn test() -> void \n{}") {
                    Right((node, rest)) => (node, rest),
                    Left(err) => panic!("test_empty_function: {}", err),
                };

                function_decl_abstract_test((&node, rest), "void");
            }

            #[test]
            pub fn test_return_function() {
                let (node, rest) =
                    match create_cst_function_decl("fn test() -> number \n{\n\treturn 1;\n}") {
                        Right((node, rest)) => (node, rest),
                        Left(err) => panic!("test_return_function: {}", err),
                    };

                function_decl_abstract_test((&node, rest), "number");

                let body = match &node {
                    CstNode::FUNCTION_DECL(n) => &n.body,
                    _ => panic!("test_return_function: node is not a function decl"),
                };

                assert_eq!(body.len(), 1);

                match &body[0] {
                    CstFunctionLineExpr::RETURN(ret) => {
                        atom_match_keyword(
                            &ret.keyword,
                            "return",
                            "test_return_function: expect return keyword",
                        );
                        atom_match_char(&ret.endline, ';', "test_return_function: expect ;");
                        node_match_float(&ret.value, 1.0, "test_return_function: expect 1;")
                    }
                    _ => panic!("test_return_function: expect return"),
                }
            }

            #[test]
            pub fn test_variable_declaration_function() {
                let (node, rest) = match create_cst_function_decl(
                    "fn test() -> number
{
    let number a = 1;
    return a;
}",
                ) {
                    Right((node, rest)) => (node, rest),
                    Left(err) => panic!("test_variable_declaration_function: {}", err),
                };

                function_decl_abstract_test((&node, rest), "number");

                let body = match &node {
                    CstNode::FUNCTION_DECL(n) => &n.body,
                    _ => panic!("test_variable_declaration_function: node is not a function decl"),
                };

                assert_eq!(body.len(), 2);
            }
        }
    }
}
