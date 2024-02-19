#include "cst.h"
#include <stdlib.h>

static EitherCSTOrError parse_char_atom(
    char ch,
    char **file_content,
    enum token_type type
) {
    if ((*file_content)[0] != ch)
        return Right(EitherCSTOrError, error("Expected an addition sign.", JL_ERROR));
    (*file_content)++;

    struct cst *c = malloc(sizeof(struct cst));

    if (!c)
        return Right(EitherCSTOrError, error("Memory allocation failed.", JL_OUT_OF_MEMORY));
    *c = (struct cst) {
        .type = type
    };
    return Left(EitherCSTOrError, c);
}

EitherCSTOrError parse_addition_atom(char **file_content)
{
    return parse_char_atom(ADD_SIGN, file_content, ATOM_ADD);
}

EitherCSTOrError parse_substraction_atom(char **file_content)
{
    return parse_char_atom(SUB_SIGN, file_content, ATOM_SUB);
}
