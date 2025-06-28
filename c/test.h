#pragma once

#include "common.h"

#define ASSERT_EQUAL(T, a, b) \
{   T##_t AE_a = (a); \
    T##_t AE_b = (b); \
    if (!(T##_c__equal_(&AE_a, &AE_b)) ) \
    {   fprintf(stderr, "l: [" #a "] was not equal to r: [" #b "]\n [l: "); \
        T##_c__print_(stderr, &AE_a); \
        fprintf(stderr, "], [r: "); \
        T##_c__print_(stderr, &AE_b); \
        fprintf(stderr, "]\n"); \
        exit(1); \
    } \
}

#define ASSERT_POINTER_EQUAL(T, a, b) \
{   T##_t APE_a = *(a); \
    T##_t APE_b = (b); \
    if (!(type_t ## __equal_(&APE_a, &APE_b)) ) \
    {   fprintf(stderr, "l: [*(" #a ")] was not equal to r: [" #b "]\n [l: "); \
        T##_c__print_(stderr, &APE_a); \
        fprintf(stderr, "], [r: "); \
        T##_c__print_(stderr, &APE_b); \
        fprintf(stderr, "]\n"); \
        exit(1); \
    } \
}

#ifndef NDEBUG
/*
{ DEBUG
*/

/*
END DEBUG 
} */
#else
/*
{ RELEASE
*/

/*
END RELEASE
} */
#endif

#define TEST_H    
