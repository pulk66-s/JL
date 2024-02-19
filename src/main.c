#include "status.h"
#include "files.h"
#include "cst.h"
#include "ast.h"
#include "interpreter.h"
#include <stdio.h>

static int err_handling(Error err)
{
    print_error(err);
    return 1;
}

int main(int ac, char **av)
{
    EitherFilesOrError files = open_files((const char **)av + 1, ac - 1);

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
    return 0;
}
