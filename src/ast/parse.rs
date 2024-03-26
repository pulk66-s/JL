pub mod keyword;

use either::Either;

use crate::cst::{
    atom::Atom,
    parser::{Parser, ParserDataType},
};

use self::keyword::Tokens;

use super::{func::definition::create_ast_func_defs, Ast};


pub fn create_ast(parser: ParserDataType) -> Result<Ast, String> {
    Ok(Ast {
        functions: match create_ast_func_defs(&mut Tokens::new(parser)) {
            Ok(f) => f,
            Err(e) => return Err(e),
        },
    })
}
