#ifndef __JL_AST_H__
#define __JL_AST_H__

/**
 * Abstract Syntax Tree that represents the structure of the JL.
*/

#include "types/func.h"
#include "types/error.h"
#include "cst.h"

enum ast_type {
    AST_NUMBER,
    AST_ADD,
};

struct ast;

struct ast_add {
    struct ast *left;
    struct ast *right;
};

struct ast {
    enum ast_type type;
    union {
        int number;
        struct ast_add add;
    } value;
};

typedef Either(struct ast *, Error) EitherASTOrError;

EitherASTOrError cst_to_ast(struct cst *cst);
MaybeError delete_ast(struct ast *ast);

#endif
