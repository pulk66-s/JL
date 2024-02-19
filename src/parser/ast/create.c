#include "ast.h"
#include <stdlib.h>

static EitherASTOrError ast_parse_expr(struct cst *cst);

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

static EitherASTOrError ast_parse_add(struct cst *cst)
{
    EitherASTOrError left = ast_parse_expr(cst->children[0]);
    EitherASTOrError right = ast_parse_expr(cst->children[4]);

    if (!left.is_left)
        return left;
    if (!right.is_left)
        return right;

    struct ast *ast = malloc(sizeof(struct ast));

    if (!ast)
        return Right(EitherASTOrError, error("Memory allocation failed.", JL_OUT_OF_MEMORY));

    *ast = (struct ast) {
        .type = AST_ADD,
        .value = {
            .add = {
                .left = left.left,
                .right = right.left
            }
        }
    };
    return Left(EitherASTOrError, ast);
}

static EitherASTOrError ast_parse_expr(struct cst *cst)
{
    switch (cst->type) {
        case NUMBER:
            return ast_parse_number(cst->value.number);
        case ADDITION:
            return ast_parse_add(cst);
        default:
            printf("Unknown CST type. %d\n", cst->type);
            return Right(EitherASTOrError, error("Unknown CST type.", JL_ERROR));
    }
}

EitherASTOrError cst_to_ast(struct cst *cst)
{
    return ast_parse_expr(cst);
}
