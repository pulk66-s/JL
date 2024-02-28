#include "eval.h"

static int eval_add(struct ast *ast)
{
    return eval_expr(ast->value.binop.left_expr) + eval_expr(ast->value.binop.right_expr);
}

static int eval_binop(struct ast *ast)
{
    if (!ast || ast->type != AST_TYPE_OPERATION)
        return 0;
    switch (ast->value.binop.op) {
        case '+':
            return eval_add(ast);
        default:
            return 0;
    }
}

int eval_expr(struct ast *ast)
{
    switch (ast->type) {
        case AST_TYPE_NUMBER:
            return ast->value.i;
        case AST_TYPE_OPERATION:
            return eval_binop(ast);
        default:
            return 0;
    }
}
