#include "cst.h"
#include <stdlib.h>

static MaybeError delete_cst_number(struct cst *cst)
{
    free(cst);
    return Nothing(MaybeError);
}

static MaybeError delete_cst_add(struct cst *cst)
{
    for (int i = 0; cst->children[i] != NULL; i++) {
        MaybeError err = delete_cst(cst->children[i]);

        if (!err.nothing) {
            return err;
        }
    }
    free(cst->children);
    free(cst);
    return Nothing(MaybeError);
}

static MaybeError delete_add_atom(struct cst *cst)
{
    free(cst);
    return Nothing(MaybeError);
}

MaybeError delete_cst(struct cst *cst)
{
    switch (cst->type) {
        case ADDITION:
            return delete_cst_add(cst);
        case NUMBER:
            return delete_cst_number(cst);
        case ATOM_ADD:
            return delete_add_atom(cst);
        default:
            return Just(MaybeError, error("Unknown CST type", JL_ERROR));
    }
}
