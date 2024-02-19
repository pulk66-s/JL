#include "ast.h"
#include <stdlib.h>

static EitherASTOrError ast_parse_operation(struct cst *cst, enum ast_type type)
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
        .type = type,
        .value = {
            .binop = {
                .left = left.left,
                .right = right.left
            }
        }
    };
    return Left(EitherASTOrError, ast);
}

EitherASTOrError ast_parse_add(struct cst *cst)
{
    return ast_parse_operation(cst, AST_ADD);
}

EitherASTOrError ast_parse_sub(struct cst *cst)
{
    return ast_parse_operation(cst, AST_SUB);
}
