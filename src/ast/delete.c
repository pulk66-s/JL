#include "ast.h"
#include <stdlib.h>

void free_ast(struct ast *ast)
{
    if (ast)
        free(ast);
}
