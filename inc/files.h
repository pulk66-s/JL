#ifndef __JL_FILES_H__
#define __JL_FILES_H__

/**
 * This files contains the function prototypes for the file handling functions
*/

#include "types.h"
#include <stdio.h>

typedef Either(FILE *, Error) EitherFileOrError;

EitherFileOrError open_file(const char *filename);
MaybeError close_file(FILE *file);

#endif
