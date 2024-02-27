#include "ast.h"
#include <stdlib.h>

static struct ast *create_int(struct cst *cst)
{
    struct ast *ast = malloc(sizeof(struct ast));

    if (!ast)
        return NULL;
    ast->type = AST_TYPE_NUMBER;
    ast->value.i = cst->value.i;
    return ast;
}

static struct ast *create_expr(struct cst *cst)
{
    switch (cst->type) {
        case CST_TYPE_INT:
            return create_int(cst);
        default:
            return NULL;
    }
}

struct ast *create_ast(struct cst *cst)
{
    return create_expr(cst);
}
