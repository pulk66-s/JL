#ifndef __JL_AST_H__
#define __JL_AST_H__

#include "cst.h"

enum ast_type {
    AST_TYPE_NUMBER,
};

struct ast {
    enum ast_type type;
    union {
        int i;
    } value;
};

struct ast *create_ast(struct cst *cst);
void free_ast(struct ast *ast);

#endif
