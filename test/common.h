#pragma once

#include <stdint.h>

typedef float float_t;

#define COMMON_C /*
{ */ \
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
    int equal_uint32_t_(uint32_t *a, uint32_t *b) \
    {   return *a == *b; \
    } \
    void print_uint32_t_(FILE *f, uint32_t *uint32) \
    {   fprintf(f, "%d", *uint32); \
    } \
    int equal_float_t_(float_t *a, float_t *b) \
    {   float abs_delta = fabs(*a - *b); \
        float abs_min = fmin(fabs(*a), fabs(*b)); \
        if (abs_min > 0.0) { \
            return abs_delta / abs_min < 1e-5; \
        } \
        /* if zero, then require absoluteness */ \
        return *a == *b; \
    } \
    void print_float_t_(FILE *f, float_t *flt) \
    {   fprintf(f, "%f", *flt); \
    } \
    /*
} end COMMON_C */
