#include "cst.h"
#include <stdlib.h>

struct cst *create_cst_number(char **expr)
{
    if (**expr == '\0')
        return NULL;

    char *save = *expr;
    int found = 0;
    int value = 0;

    while (**expr >= '0' && **expr <= '9') {
        value = value * 10 + **expr - '0';
        found = 1;
        (*expr)++;
    }
    if (!found) {
        *expr = save;
        return NULL;
    }

    struct cst *cst = malloc(sizeof(struct cst));

    if (!cst)
        return NULL;
    cst->type = CST_TYPE_INT;
    cst->value.i = value;
    return cst;
}

static struct cst *create_expr(char **expr)
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

struct cst *create_cst(char *expr)
{
    return create_expr(&expr);
}
