#include "refer.h"

#ifndef SINGLE_IMPORT
#define OH_HI(fn, attr, impl) OH_HI_IMPL(fn, attr, impl)
REFER
#undef OH_HI
#endif
