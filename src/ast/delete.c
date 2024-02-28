#include "ast.h"
#include <stdlib.h>

static void free_ast_operation(struct ast *ast)
{
    if (!ast || ast->type != AST_TYPE_OPERATION)
        return;
    free_ast(ast->value.binop.left_expr);
    free_ast(ast->value.binop.right_expr);
    free(ast);
}

static void free_ast_number(struct ast *ast)
{
    if (!ast || ast->type != AST_TYPE_NUMBER)
        return;
    free(ast);
}

void free_ast(struct ast *ast)
{
    if (!ast)
        return;
    switch (ast->type) {
        case AST_TYPE_OPERATION:
            free_ast_operation(ast);
            break;
        case AST_TYPE_NUMBER:
            free_ast_number(ast);
            break;
        default:
            break;
    }
}
