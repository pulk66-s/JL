#include "interpreter.h"

static EitherIntOrError understand_expr(struct ast *ast);

static EitherIntOrError understand_add(struct ast_add add)
{
    EitherIntOrError left, right;

    left = understand_expr(add.left);
    if (!left.is_left)
        return left;
    right = understand_expr(add.right);
    if (!right.is_left)
        return right;
    return Left(EitherIntOrError, left.left + right.left);
}

static EitherIntOrError understand_expr(struct ast *ast)
{
    switch (ast->type) {
        case AST_NUMBER:
            return Left(EitherIntOrError, ast->value.number);
        case AST_ADD:
            return understand_add(ast->value.add);
        default:
            return Right(EitherIntOrError, error("Unknown AST type.", JL_ERROR));
    }
}

void interpret(struct ast *ast)
{
    EitherIntOrError result = understand_expr(ast);

    if (result.is_left)
        printf("%d\n", result.left);
    else
        printf("Error: %s\n", result.right.message);
}
