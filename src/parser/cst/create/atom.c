#include "cst.h"
#include <stdlib.h>

EitherCSTOrError parse_addition_atom(char **file_content)
{
    if ((*file_content)[0] != ADD_SIGN)
        return Right(EitherCSTOrError, error("Expected an addition sign.", JL_ERROR));
    (*file_content)++;

    struct cst *c = malloc(sizeof(struct cst));

    if (!c)
        return Right(EitherCSTOrError, error("Memory allocation failed.", JL_OUT_OF_MEMORY));
    *c = (struct cst) {
        .type = ATOM_ADD
    };
    return Left(EitherCSTOrError, c);
}
