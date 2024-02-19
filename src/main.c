#include "status.h"
#include "files.h"
#include "cst.h"
#include "ast.h"
#include "interpreter.h"
#include <stdio.h>
#include <stdlib.h>

static int err_handling(Error err)
{
    print_error(err);
    return 1;
}

int main(int ac, const char **av)
{
    EitherFilesOrError files = open_files(av + 1, ac - 1);

    if (!files.is_left)
        return err_handling(files.right);

    EitherStringOrError res = read_file(files.left[0]);

    if (!res.is_left)
        return err_handling(res.right);

    char *content = res.left;
    EitherCSTOrError cst = parse_program(&content);

    if (!cst.is_left)
        return err_handling(cst.right); 

    EitherASTOrError ast = cst_to_ast(cst.left);

    if (!ast.is_left)
        return err_handling(ast.right);
    interpret(ast.left);
    free(res.left);

    MaybeError err = delete_cst(cst.left);

    if (!err.nothing)
        return err_handling(err.data);
    err = delete_ast(ast.left);
    if (!err.nothing)
        return err_handling(err.data);
    err = close_files(files.left);
    if (!err.nothing)
        return err_handling(err.data);
    return 0;
}
