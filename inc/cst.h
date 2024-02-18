#ifndef __JL_CST_H__
#define __JL_CST_H__

/**
 * CST (Concrete Syntax Tree) is a tree representation of the source code
 * that is used to parse the source code and generate the AST (Abstract Syntax Tree)
*/

#include "types.h"

enum token_type {
    PROGRAM,
    STATEMENT,
    NUMBER,
    STRING,
    ADD,
};

struct cst {
    enum token_type type;
    union {
        int number;
        char *string;
    };
    struct cst **children;
};

#endif
