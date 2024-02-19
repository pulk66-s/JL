#ifndef __JL_INTERPRETER_H__
#define __JL_INTERPRETER_H__

#include "ast.h"
#include "types/func.h"

void interpret(struct ast *ast);

typedef Either(int, Error) EitherIntOrError;

#endif
