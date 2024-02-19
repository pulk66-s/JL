#include "cst.h"
#include <stdlib.h>

static MaybeError delete_cst_number(struct cst *cst)
{
    free(cst);
    return Nothing(MaybeError);
}

static MaybeError delete_cst_binop(struct cst *cst)
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

static MaybeError delete_char_atom(struct cst *cst)
{
    free(cst);
    return Nothing(MaybeError);
}

static MaybeError delete_spaces(struct cst *cst)
{
    free(cst);
    return Nothing(MaybeError);
}

MaybeError delete_cst(struct cst *cst)
{
    switch (cst->type) {
        case SUBSTRACTION:
        case ADDITION:
            return delete_cst_binop(cst);
        case NUMBER:
            return delete_cst_number(cst);
        case ATOM_ADD:
        case ATOM_SUB:
            return delete_char_atom(cst);
        case SPACES:
            return delete_spaces(cst);
        default:
            return Just(MaybeError, error("Unknown CST type", JL_ERROR));
    }
}
