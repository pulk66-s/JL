#include "cst.h"
#include <stdlib.h>

struct cst *create_cst_add_right_expr(char **expr)
{
    if (**expr == '\0')
        return NULL;

    char *save = *expr;
    struct cst *cst = create_cst_operation(expr);

    if (cst)
        return cst;
    cst = create_cst_number(expr);
    if (cst)
        return cst;
    *expr = save;
    return NULL;
}

struct cst *create_cst_operation(char **expr)
{
    char *save = *expr;
    struct cst *operation = malloc(sizeof(struct cst)),
                *left_expr = create_cst_number(expr),
                *spaces1 = create_cst_spaces(expr),
                *add_atom = create_cst_add_atom(expr),
                *spaces2 = create_cst_spaces(expr),
                *right_expr = create_cst_add_right_expr(expr),
                *spaces3 = create_cst_spaces(expr);

    if (
        !operation || !left_expr || !spaces1 || !add_atom || !spaces2 || !right_expr
    ) {
        *expr = save;
        return NULL;
    }
    operation->type = CST_TYPE_OPERATION;
    operation->value.operation = (struct cst_operation) {
        .left_expr = left_expr,
        .spaces1 = spaces1,
        .add_atom = add_atom,
        .spaces2 = spaces2,
        .right_expr = right_expr,
        .spaces3 = spaces3,
    };
    return operation;
}
