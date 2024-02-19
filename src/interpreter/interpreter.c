#include "interpreter.h"

static EitherIntOrError understand_expr(struct ast *ast);

static int add(int left, int right)
{
    return left + right;
}

static int sub(int left, int right)
{
    return left - right;
}

static EitherIntOrError understand_binop(struct ast_binop binop, int (*operation)(int, int))
{
    EitherIntOrError left, right;

    left = understand_expr(binop.left);
    if (!left.is_left)
        return left;
    right = understand_expr(binop.right);
    if (!right.is_left)
        return right;
    return Left(EitherIntOrError, operation(left.left, right.left));
}

static EitherIntOrError understand_add(struct ast_binop binop)
{
    return understand_binop(binop, add);
}

static EitherIntOrError understand_sub(struct ast_binop binop)
{
    return understand_binop(binop, sub);
}

static EitherIntOrError understand_expr(struct ast *ast)
{
    switch (ast->type) {
        case AST_NUMBER:
            return Left(EitherIntOrError, ast->value.number);
        case AST_ADD:
            return understand_add(ast->value.binop);
        case AST_SUB:
            return understand_sub(ast->value.binop);
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
