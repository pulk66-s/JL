#ifndef __JL_TYPES_FUNC_H__
#define __JL_TYPES_FUNC_H__

#include "types/bool.h"

#define Maybe(type) struct { type data; bool nothing; }

typedef Maybe(void *) Maybe;

#define Just(t, x) (t){.data = x, .nothing = false}
#define Nothing(t) (t){.nothing = true}

#define Either(l, r) struct { l left; r right; bool is_left; }
#define Left(t, x) (t){.left = x, .is_left = true}
#define Right(t, x) (t){.right = x, .is_left = false}

#endif
