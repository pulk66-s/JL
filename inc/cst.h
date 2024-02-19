#ifndef __JL_CST_H__
#define __JL_CST_H__

/**
 * CST (Concrete Syntax Tree) is a tree representation of the source code
 * that is used to parse the source code and generate the AST (Abstract Syntax Tree)
*/

#include "types.h"
#include <stdarg.h>

#define ADD_SIGN    '+'

enum token_type {
    PROGRAM,
    STATEMENT,
    NUMBER,
    STRING,
    ATOM_ADD,
    SPACES,
    ADDITION,
    UNKNOWN
};

struct cst {
    enum token_type type;
    union {
        int number;
        char *string;
    } value;
    struct cst **children;
};

typedef Either(struct cst *, Error) EitherCSTOrError;

EitherCSTOrError cst_parse_and(EitherCSTOrError (*funcs[])(char **), char **file_content);
EitherCSTOrError cst_parse_or(EitherCSTOrError (*funcs[])(char **), char **file_content);
EitherCSTOrError parse_spaces(char **file_content);
EitherCSTOrError parse_maybe_spaces(char **file_content);
EitherCSTOrError parse_number(char **file_content);
EitherCSTOrError parse_addition_atom(char **file_content);
EitherCSTOrError parse_addition(char **file_content);
EitherCSTOrError parse_program(char **file_content);
MaybeError delete_cst(struct cst *cst);

#endif
