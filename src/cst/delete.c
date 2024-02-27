#include "cst.h"
#include <stdlib.h>

void free_cst(struct cst *cst)
{
    if (cst)
        free(cst);
}
