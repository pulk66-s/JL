#include "cst.h"
#include <stdlib.h>

// EitherCSTOrError parse_addition(char **file_content)
// {
//     EitherCSTOrError left, atom, right;
//     char *save = *file_content;

//     if (!((left = parse_number(file_content)).is_left)) {
//         *file_content = save;
//         return left;
//     }
//     parse_spaces(file_content);
//     if (!((atom = parse_addition_atom(file_content)).is_left)) {
//         *file_content = save;
//         return atom;
//     }
//     parse_spaces(file_content);
//     if (!((right = parse_number(file_content)).is_left)) {
//         *file_content = save;
//         return right;
//     }

//     struct cst *c = malloc(sizeof(struct cst));

//     if (!c)
//         return Right(EitherCSTOrError, error("Memory allocation failed.", JL_OUT_OF_MEMORY));
//     c->children = malloc(4 * sizeof(struct cst *));
//     if (!c->children)
//         return Right(EitherCSTOrError, error("Memory allocation failed.", JL_OUT_OF_MEMORY));
//     c->type = ADDITION;
//     c->children[0] = left.left;
//     c->children[1] = atom.left;
//     c->children[2] = right.left;
//     c->children[3] = NULL;
//     return Left(EitherCSTOrError, c);
// }

EitherCSTOrError parse_addition(char **file_content)
{
    EitherCSTOrError add = cst_parse_and((EitherCSTOrError (*[])(char **)) {
        parse_number, parse_maybe_spaces, parse_addition_atom, parse_maybe_spaces,
        parse_number, NULL
    }, file_content);

    if (!add.is_left)
        return add;
    add.left->type = ADDITION;
    return add;
}
