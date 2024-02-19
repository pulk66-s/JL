#ifndef __JL_FILES_H__
#define __JL_FILES_H__

/**
 * This files contains the function prototypes for the file handling functions
*/

#include "types.h"
#include "types/string.h"
#include <stdio.h>

typedef Either(FILE *, Error) EitherFileOrError;
typedef Either(FILE **, Error) EitherFilesOrError;

EitherFileOrError open_file(const char *filename);
EitherFilesOrError open_files(const char **filenames, size_t count);
MaybeError close_file(FILE *file);
MaybeError close_files(FILE **files);
EitherStringOrError read_file(FILE *file);

#endif
