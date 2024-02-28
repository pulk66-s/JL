#include "cst.h"
#include <stdlib.h>

static struct cst *create_cst_char(char **expr, char c)
{
    if (**expr != c)
        return NULL;

    struct cst *cst = malloc(sizeof(struct cst));

    if (!cst)
        return NULL;
    cst->type = CST_TYPE_CHAR;
    cst->value.c = c;
    (*expr)++;
    return cst;
}

struct cst *create_cst_space(char **expr)
{
    return create_cst_char(expr, ' ');
}

struct cst *create_cst_add_atom(char **expr)
{
    return create_cst_char(expr, '+');
}

struct cst *create_cst_endline(char **expr)
{
    return create_cst_char(expr, ';');
}
