#include "status.h"
#include "files.h"
#include <stdio.h>

int main(int ac, char **av)
{
    for (int i = 1; i < ac; i++) {
        EitherFileOrError file = open_file(av[i]);

        if (file.is_left) {
            close_file(file.left);
        } else {
            printf("Error: %s\n", file.right.message);
        }
    }
    return 0;
}
