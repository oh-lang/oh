#include "common.h"

#ifndef ILL_DO_IT_MYSELF
#define IMPL(fn, attr, impl) fn attr impl
COMMON
#undef IMPL
#endif
