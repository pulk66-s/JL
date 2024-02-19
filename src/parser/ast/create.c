#include "ast.h"
#include <stdlib.h>

static EitherASTOrError ast_parse_number(int number)
{
    struct ast *ast = malloc(sizeof(struct ast));

    if (!ast)
        return Right(EitherASTOrError, error("Memory allocation failed.", JL_OUT_OF_MEMORY));
    *ast = (struct ast) {
        .type = AST_NUMBER,
        .value = {
            .number = number
        }
    };
    return Left(EitherASTOrError, ast);
}

EitherASTOrError ast_parse_expr(struct cst *cst)
{
    switch (cst->type) {
        case NUMBER:
            return ast_parse_number(cst->value.number);
        case ADDITION:
            return ast_parse_add(cst);
        case SUBSTRACTION:
            return ast_parse_sub(cst);
        default:
            printf("Unknown CST type. %d\n", cst->type);
            return Right(EitherASTOrError, error("Unknown CST type.", JL_ERROR));
    }
}

EitherASTOrError cst_to_ast(struct cst *cst)
{
    return ast_parse_expr(cst);
}
