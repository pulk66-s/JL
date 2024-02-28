#ifndef __JL_AST_H__
#define __JL_AST_H__

#include "cst.h"

enum ast_type {
    AST_TYPE_NUMBER,
    AST_TYPE_OPERATION,
};

struct ast {
    enum ast_type type;
    union {
        int i;
        struct {
            struct ast *left_expr;
            struct ast *right_expr;
            char op;
        } binop;
    } value;
};

struct ast *create_ast(struct cst *cst);
void free_ast(struct ast *ast);
struct ast *create_ast_int(struct cst *cst);
struct ast *create_ast_operation(struct cst *cst);

#endif
