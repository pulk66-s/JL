#include "types/string.h"
#include "cst.h"
#include "ast.h"
#include "eval.h"
#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

static int err_handling(Error err)
{
    print_error(err);
    return 1;
}

static int eval_input(char *input)
{
    EitherCSTOrError cst = cst_parse_program(&input);

    if (!cst.is_left)
        return err_handling(cst.right); 

    EitherASTOrError ast = cst_to_ast(cst.left);

    if (!ast.is_left)
        return err_handling(ast.right);
    eval(ast.left);

    MaybeError err = delete_cst(cst.left);

    if (!err.nothing)
        return err_handling(err.data);
    err = delete_ast(ast.left);
    if (!err.nothing)
        return err_handling(err.data);
    return 0;
}

MaybeString read_line(void)
{
    char *line = malloc(sizeof(char) * 1024);
    size_t max_size = 1024;
    size_t size = 0;
    char ch;

    while (read(STDIN_FILENO, &ch, 1) > 0) {
        if (ch == '\n') {
            line[size] = '\0';
            return Just(MaybeString, line);
        }
        line[size++] = ch;
        if (size == max_size) {
            max_size *= 2;
            line = realloc(line, max_size);
        }
    }
    free(line);
    return Nothing(MaybeString);
}

EitherStringOrError loop_tty(void)
{
    while (true) {
        MaybeString line = read_line();

        if (line.nothing)
            return Right(EitherStringOrError, "reading error");
        if (line.data[0] == '\0')
            return Left(EitherStringOrError, line.data);
        printf("=> %s\n", line.data);
        if (strcmp(line.data, "exit") == 0)
            return Left(EitherStringOrError, "exit");

        int ret = eval_input(line.data);

        if (ret != 0)
            return Right(EitherStringOrError, "eval error");
        free(line.data);
    }
    return Right(EitherStringOrError, "loop error");
}
