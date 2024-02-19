#include "ast.h"
#include <stdlib.h>

static MaybeError delete_ast_number(struct ast *ast)
{
    free(ast);
    return Nothing(MaybeError);
}

static MaybeError delete_ast_add(struct ast *ast)
{
    MaybeError err = delete_ast(ast->value.add.left);

    if (!err.nothing)
        return err;
    err = delete_ast(ast->value.add.right);
    if (!err.nothing)
        return err;
    free(ast);
    return Nothing(MaybeError);
}

MaybeError delete_ast(struct ast *ast)
{
    switch (ast->type) {
        case AST_ADD:
            return delete_ast_add(ast);
        case AST_NUMBER:
            return delete_ast_number(ast);
        default:
            return Just(MaybeError, error("Unknown AST type", JL_ERROR));
    }
}
