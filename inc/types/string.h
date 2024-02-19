#ifndef __JL_TYPES_STRING_H__
#define __JL_TYPES_STRING_H__

#include "types/func.h"
#include "types/error.h"

typedef Maybe(char *) MaybeString;
typedef Either(char *, Error) EitherStringOrError;

#endif