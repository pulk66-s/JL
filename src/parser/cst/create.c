#include "cst.h"
#include <stdlib.h>

EitherCSTOrError cst_parse_or(EitherCSTOrError (*funcs[])(char **), char **file_content)
{
    for (size_t i = 0; funcs[i]; i++) {
        EitherCSTOrError cst = funcs[i](file_content);

        printf("index %zu, buffer: (%s)\n", i, *file_content);
        if (cst.is_left)
            return cst;
    }
    return Right(EitherCSTOrError, error("No parser found in the or list of parsers.", JL_ERROR));
}

EitherCSTOrError cst_parse_and(EitherCSTOrError (*funcs[])(char **), char **file_content)
{
    char *save = *file_content;
    size_t size = 0;

    for (size_t i = 0; funcs[i]; i++)
        size++;

    struct cst **csts = malloc((size + 1) * sizeof(struct cst *));

    if (!csts)
        return Right(EitherCSTOrError, error("Memory allocation failed.", JL_OUT_OF_MEMORY));
    for (size_t i = 0; i < size; i++) {
        EitherCSTOrError cst = funcs[i](file_content);

        if (!cst.is_left) {
            for (size_t j = 0; j < i; j++)
                free(csts[j]);
            free(csts);
            *file_content = save;
            return cst;
        }
        csts[i] = cst.left;
    }
    csts[size] = NULL;

    struct cst *c = malloc(sizeof(struct cst));

    if (!c)
        return Right(EitherCSTOrError, error("Memory allocation failed.", JL_OUT_OF_MEMORY));
    *c = (struct cst) {
        .type = PROGRAM,
        .children = csts
    };
    return Left(EitherCSTOrError, c);
}

EitherCSTOrError cst_parse_rvalue(char **file_content)
{
    return cst_parse_or((EitherCSTOrError (*[])(char **)) {
        cst_parse_binop, cst_parse_number, NULL
    }, file_content);
}

EitherCSTOrError cst_parse_binop(char **file_content)
{
    return cst_parse_or((EitherCSTOrError (*[])(char **)) {
        cst_parse_addition, cst_parse_substraction, NULL
    }, file_content);
}

EitherCSTOrError cst_parse_program(char **file_content)
{
    return cst_parse_binop(file_content);
}
