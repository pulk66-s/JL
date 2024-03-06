// #[cfg(test)]
// pub mod tests {
//     pub mod function_decl_condition {
//         use either::Either::{Left, Right};

//         use crate::cst::{data::CstNode, function::function_decl::{create_cst_function_decl, tests::tests::function_decl::function_decl_abstract_test}};

//         #[test]
//         fn test_function_decl_condition() {
//             let (node, rest) = match create_cst_function_decl(
//                 "fn is_odd(number n) -> number {
//     if (n % 2 == 0) {
//         return 1;
//     }
//     return 0;
// }",
//             ) {
//                 Left(err) => panic!("test_function_decl_condition: {}", err),
//                 Right(r) => r,
//             };

//             function_decl_abstract_test((&node, rest), "number");

//             let decl = match node {
//                 CstNode::FUNCTION_DECL(n) => n,
//                 _ => panic!("test_function_decl_condition: node is not a function decl"),
//             };
//             let body = &decl.body;

//             assert_eq!(body.len(), 2);
//         }
//     }
// }
