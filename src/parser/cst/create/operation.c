#include "cst.h"
#include <stdlib.h>

static EitherCSTOrError parse_operation(
    char **file_content,
    EitherCSTOrError (*func)(char **),
    enum token_type type
) {
    EitherCSTOrError op = cst_parse_and((EitherCSTOrError (*[])(char **)) {
        parse_number, parse_maybe_spaces, func, parse_maybe_spaces,
        parse_number, NULL
    }, file_content);

    if (!op.is_left)
        return op;
    op.left->type = type;
    return op;
}

EitherCSTOrError parse_substration(char **file_content)
{
    return parse_operation(file_content, parse_substraction_atom, SUBSTRACTION);
}

EitherCSTOrError parse_addition(char **file_content)
{
    return parse_operation(file_content, parse_addition_atom, ADDITION);
}
