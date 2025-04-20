#pragma once

#include <stdint.h>

typedef float float_t;

#define COMMON_C \
    void enscope_float_t_(float_t *f) \
    {   *f = 0.0; \
    } \
    void descope_float_t_(float_t *f) \
    {   /* nothing to do */ \
    } \
    void enscope_uint32_t_(uint32_t *uint32) \
    {   *uint32 = 0; \
    } \
    void descope_uint32_t_(uint32_t *uint32) \
    {   /* nothing to do */ \
    } \
/* end COMMON_C */
