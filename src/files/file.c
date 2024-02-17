#include "files.h"

EitherFileOrError open_file(const char *filename)
{
    FILE *file = fopen(filename, "r");

    if (!file)
        return Right(EitherFileOrError, error(fopen_errno(errno), JL_LIBC_ERROR));
    return Left(EitherFileOrError, file);
}

MaybeError close_file(FILE *file)
{
    if (fclose(file) == EOF)
        return Just(MaybeError, error(fopen_errno(errno), JL_LIBC_ERROR));
    return Nothing(MaybeError);
}
