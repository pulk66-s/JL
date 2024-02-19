#include "cst.h"
#include <stdlib.h>

EitherCSTOrError cst_parse_or(EitherCSTFunc *funcs, char **file_content)
{
    for (size_t i = 0; funcs[i]; i++) {
        EitherCSTOrError cst = funcs[i](*file_content);

        if (cst.is_left)
            return cst;
    }
    return Right(EitherCSTOrError, error("No parser found in the or list of parsers.", JL_ERROR));
}

EitherCSTOrError parse_spaces(char **file_content)
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

static bool is_digit(char c)
{
    return c >= '0' && c <= '9';
}

EitherCSTOrError parse_number(char **file_content)
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

EitherCSTOrError parse_addition(char **file_content)
{
    EitherCSTOrError left, atom, right;
    char *save = *file_content;

    if (!((left = parse_number(file_content)).is_left)) {
        *file_content = save;
        return left;
    }
    parse_spaces(file_content);
    if (!((atom = parse_addition_atom(file_content)).is_left)) {
        *file_content = save;
        return atom;
    }
    parse_spaces(file_content);
    if (!((right = parse_number(file_content)).is_left)) {
        *file_content = save;
        return right;
    }

    struct cst *c = malloc(sizeof(struct cst));

    if (!c)
        return Right(EitherCSTOrError, error("Memory allocation failed.", JL_OUT_OF_MEMORY));
    c->children = malloc(3 * sizeof(struct cst *));
    if (!c->children)
        return Right(EitherCSTOrError, error("Memory allocation failed.", JL_OUT_OF_MEMORY));
    c->type = ADDITION;
    c->children[0] = left.left;
    c->children[1] = atom.left;
    c->children[2] = right.left;
    return Left(EitherCSTOrError, c);
}

EitherCSTOrError parse_program(char **file_content)
{
    return parse_addition(file_content);
}
