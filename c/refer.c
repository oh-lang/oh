#include "refer.h"

#ifndef SINGLE_IMPORT
#define IMPL(fn, attr, impl) fn impl
REFER
#undef IMPL
#endif
