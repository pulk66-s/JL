#include "cst.h"
#include <stdlib.h>

struct cst *create_cst_spaces(char **expr)
{
    if (**expr == '\0')
        return NULL;

    char *save = *expr;
    struct cst *cst = malloc(sizeof(struct cst));

    if (!cst) {
        *expr = save;
        return NULL;
    }
    cst->type = CST_TYPE_STRING;
    cst->value.s = malloc(2);
    cst->value.s[0] = ' ';
    cst->value.s[1] = '\0';
    while (**expr == ' ')
        (*expr)++;
    return cst;
}
