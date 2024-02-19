#include "files.h"
#include <stdlib.h>
#include <errno.h>

EitherFileOrError open_file(const char *filename)
{
    FILE *file = fopen(filename, "r");

    if (!file)
        return Right(EitherFileOrError, error(fopen_errno(errno), JL_LIBC_ERROR));
    return Left(EitherFileOrError, file);
}

EitherFilesOrError open_files(const char **filenames, size_t count)
{
    FILE **files = malloc((count + 1) * sizeof(FILE *));

    if (!files)
        return Right(EitherFilesOrError, error("out of memory", JL_OUT_OF_MEMORY));
    for (size_t i = 0; i < count; i++) {
        EitherFileOrError file = open_file(filenames[i]);

        if (!file.is_left) {
            for (size_t j = 0; j < i; j++)
                close_file(files[j]);
            free(files);
            return Right(EitherFilesOrError, file.right);
        }
        files[i] = file.left;
    }
    files[count] = NULL;
    return Left(EitherFilesOrError, files);
}

MaybeError close_file(FILE *file)
{
    if (fclose(file) == EOF)
        return Just(MaybeError, error(fopen_errno(errno), JL_LIBC_ERROR));
    return Nothing(MaybeError);
}

MaybeError close_files(FILE **files)
{
    for (size_t i = 0; files[i]; i++) {
        MaybeError error = close_file(files[i]);

        if (!error.nothing)
            return error;
    }
    free(files);
    return Nothing(MaybeError);
}

EitherStringOrError read_file(FILE *file)
{
    char *buffer = NULL;
    size_t size = 0;
    ssize_t read;

    do {
        buffer = realloc(buffer, size + 4096);
        if (!buffer)
            return Right(EitherStringOrError, error("out of memory", JL_OUT_OF_MEMORY));
        read = fread(buffer + size, 1, 4096, file);
        size += read;
    } while (read == 4096);
    if (ferror(file)) {
        free(buffer);
        return Right(EitherStringOrError, error(fopen_errno(errno), JL_LIBC_ERROR));
    }
    buffer = realloc(buffer, size + 1);
    buffer[size] = '\0';
    return Left(EitherStringOrError, buffer);
}
