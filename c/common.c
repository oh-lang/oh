#include "common.h"

#ifndef SINGLE_IMPORT
#define IMPL(fn, attr, impl) fn impl
COMMON
#undef IMPL
#endif
