#include "cst.h"
#include <stdlib.h>

static void free_char(struct cst *cst)
{
    if (!cst || cst->type != CST_TYPE_CHAR)
        return;
    free(cst);
}

static void free_int(struct cst *cst)
{
    if (!cst || cst->type != CST_TYPE_INT)
        return;
    free(cst);
}

static void free_string(struct cst *cst)
{
    if (!cst || cst->type != CST_TYPE_STRING)
        return;
    free(cst->value.s);
    free(cst);
}

static void free_operation(struct cst *cst)
{
    if (!cst || cst->type != CST_TYPE_OPERATION)
        return;
    free_cst(cst->value.operation.left_expr);
    free_cst(cst->value.operation.spaces1);
    free_cst(cst->value.operation.add_atom);
    free_cst(cst->value.operation.spaces2);
    free_cst(cst->value.operation.right_expr);
    free_cst(cst->value.operation.spaces3);
    free(cst);
}

void free_cst(struct cst *cst)
{
    if (!cst)
        return;
    switch (cst->type) {
        case CST_TYPE_OPERATION:
            free_operation(cst);
            break;
        case CST_TYPE_STRING:
            free_string(cst);
            break;
        case CST_TYPE_INT:
            free_int(cst);
            break;
        case CST_TYPE_CHAR:
            free_char(cst);
            break;
        default:
            break;
    }
}
