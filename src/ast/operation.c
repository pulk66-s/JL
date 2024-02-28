#include "ast.h"
#include <stdlib.h>

struct ast *create_ast_operation(struct cst *cst)
{
    if (cst->type != CST_TYPE_OPERATION)
        return NULL;

    struct ast *ast = malloc(sizeof(struct ast));

    if (!ast)
        return NULL;
    ast->type = AST_TYPE_OPERATION;
    ast->value.binop.left_expr = create_ast(cst->value.operation.left_expr);
    ast->value.binop.right_expr = create_ast(cst->value.operation.right_expr);
    ast->value.binop.op = cst->value.operation.add_atom->value.c;
    return ast;
}
