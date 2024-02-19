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
    ADDITION
};

struct cst {
    enum token_type type;
    union {
        int number;
        char *string;
    } value;
    struct cst **children;
};

typedef Either(struct cst, Error) EitherCSTOrError;
typedef EitherCSTOrError (*EitherCSTFunc)(char *);

EitherCSTOrError cst_parse_or(EitherCSTFunc *funcs, char **file_content);
EitherCSTOrError parse_spaces(char **file_content);
EitherCSTOrError parse_number(char **file_content);
EitherCSTOrError parse_addition_atom(char **file_content);
EitherCSTOrError parse_addition(char **file_content);

#endif
