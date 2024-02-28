#include "tty.h"
#include "eval.h"
#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

static void eval_tty_expr(char *expr)
{
    struct cst *cst = create_cst(expr);

    if (!cst) {
        printf("Error while creating cst\n");
        return;
    }

    struct ast *ast = create_ast(cst);

    if (!ast) {
        printf("Error while creating ast\n");
        return;
    }

    int result = eval_expr(ast);

    printf("%d\n", result);
    free_cst(cst);
    free_ast(ast);
}

static char *read_line(void)
{
    size_t size = 0;
    size_t max_size = 1024;
    char *line = malloc(max_size * sizeof(char));

    if (!line)
        return NULL;
    while (1) {
        char c = getchar();

        if (c == '\n') {
            line[size] = '\0';
            return line;
        }
        line[size++] = c;
        if (size >= max_size) {
            max_size *= 2;
            line = realloc(line, max_size * sizeof(char));
            if (!line)
                return NULL;
        }
    }
    return line;
}

int tty_loop(void)
{
    while (1) {
        char *line = read_line();

        if (!line)
            return 1;
        if (strcmp(line, "exit") == 0) {
            free(line);
            return 0;
        }
        eval_tty_expr(line);
        free(line);
    }
}
