#include "cst.h"
#include <stdlib.h>

static bool is_digit(char c)
{
    return c >= '0' && c <= '9';
}

EitherCSTOrError cst_parse_number(char **file_content)
{
    if (!is_digit((*file_content)[0]))
        return Right(EitherCSTOrError, error("Expected a number.", JL_ERROR));

    int number = 0;

    while (is_digit((*file_content)[0])) {
        number = number * 10 + (*file_content)[0] - '0';
        (*file_content)++;
    }

    struct cst *c = malloc(sizeof(struct cst));

    if (!c)
        return Right(EitherCSTOrError, error("Memory allocation failed.", JL_OUT_OF_MEMORY));
    *c = (struct cst) {
        .type = NUMBER,
        .value = {
            .number = number
        }
    };
    return Left(EitherCSTOrError, c);
}
