#include "cst.h"
#include <stdlib.h>

static EitherCSTOrError cst_parse_operation(
    char **file_content,
    EitherCSTOrError (*func)(char **),
    enum token_type type
) {
    EitherCSTOrError op = cst_parse_and((EitherCSTOrError (*[])(char **)) {
        cst_parse_number, cst_parse_maybe_spaces, func, cst_parse_maybe_spaces,
        cst_parse_rvalue, NULL
    }, file_content);

    if (!op.is_left)
        return op;
    op.left->type = type;
    return op;
}

EitherCSTOrError cst_parse_substraction(char **file_content)
{
    return cst_parse_operation(file_content, cst_parse_substraction_atom, SUBSTRACTION);
}

EitherCSTOrError cst_parse_addition(char **file_content)
{
    return cst_parse_operation(file_content, cst_parse_addition_atom, ADDITION);
}
