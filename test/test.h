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

#define ASSERT_POINTER_EQUAL(type_t, a, b) \
{   type_t AE_a = *(a); \
    type_t AE_b = (b); \
    if (!(equal_ ## type_t ## _(&AE_a, &AE_b)) ) \
    {   fprintf(stderr, "l: [*(" #a ")] was not equal to r: [" #b "]\n [l: "); \
        print_ ## type_t ## _(stderr, &AE_a); \
        fprintf(stderr, "], [r: "); \
        print_ ## type_t ## _(stderr, &AE_b); \
        fprintf(stderr, "]\n"); \
        exit(1); \
    } \
}
