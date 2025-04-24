#include "common.h"

#ifndef ILL_DO_IT_MYSELF
#define IMPL(x, y) x y
COMMON
#undef IMPL
#endif
