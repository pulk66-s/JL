#include "cst.h"
#include <stdlib.h>

EitherCSTOrError cst_parse_spaces(char **file_content)
{
    if ((*file_content)[0] != ' ')
        return Right(EitherCSTOrError, error("Expected a space.", JL_ERROR));
    while (*file_content[0] == ' ')
        (*file_content)++;

    struct cst *c = malloc(sizeof(struct cst));

    if (!c)
        return Right(EitherCSTOrError, error("Memory allocation failed.", JL_OUT_OF_MEMORY));
    *c = (struct cst) {
        .type = SPACES
    };
    return Left(EitherCSTOrError, c);
}

EitherCSTOrError cst_parse_maybe_spaces(char **file_content)
{
    EitherCSTOrError res = cst_parse_spaces(file_content);
    struct cst *c = malloc(sizeof(struct cst));

    if (res.is_left)
        delete_cst(res.left);
    if (!c)
        return Right(EitherCSTOrError, error("Memory allocation failed.", JL_OUT_OF_MEMORY));
    *c = (struct cst) {
        .type = SPACES
    };
    return Left(EitherCSTOrError, c);
}
