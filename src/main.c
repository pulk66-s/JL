#include "status.h"
#include "files.h"
#include <stdio.h>

int main(int ac, char **av)
{
    EitherFilesOrError files = open_files((const char **)av + 1, ac - 1);

    if (!files.is_left) {
        print_error(files.right);
        return 1;
    }
    close_files(files.left);
    return 0;
}
