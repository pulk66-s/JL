#include "status.h"
#include "files.h"
#include "cst.h"
#include <stdio.h>

int main(int ac, char **av)
{
    EitherFilesOrError files = open_files((const char **)av + 1, ac - 1);

    if (!files.is_left) {
        print_error(files.right);
        return 1;
    }

    EitherStringOrError res = read_file(files.left[0]);

    if (!res.is_left) {
        print_error(res.right);
        return 1;
    }

    char *content = res.left;
    EitherCSTOrError spaces = parse_addition(&content);

    if (!spaces.is_left) {
        print_error(spaces.right);
        return 1;
    }
    printf("success %d\n", spaces.left.type);
    close_files(files.left);
    return 0;
}
