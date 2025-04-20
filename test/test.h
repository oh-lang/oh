#pragma once

#include "common.h"

#define ASSERT_EQUAL(type_t, a, b) \
{   type_t AE_a = (a); \
    type_t AE_b = (b); \
    if (!(equal_ ## type_t ## _(&AE_a, &AE_b)) ) \
    {   fprintf(stderr, "l: [" #a "] was not equal to r: [" #b "]\n [l: "); \
        print_ ## type_t ## _(stderr, &AE_a); \
        fprintf(stderr, "], [r: "); \
        print_ ## type_t ## _(stderr, &AE_b); \
        fprintf(stderr, "]\n"); \
        exit(1); \
    } \
}

#define TEST_C \
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
/* end TEST_C */
