#include "eval.h"

int eval_expr(struct ast *ast)
{
    switch (ast->type) {
        case AST_TYPE_NUMBER:
            return ast->value.i;
        default:
            return 0;
    }
}
