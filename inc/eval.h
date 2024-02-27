#ifndef __JL_EVAL_H__
#define __JL_EVAL_H__

#include "ast.h"
#include "types/func.h"

void eval(struct ast *ast);

typedef Either(int, Error) EitherIntOrError;

#endif
