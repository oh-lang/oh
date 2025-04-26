#pragma once

#include "common.h"

#define ASSERT_EQUAL(T, a, b) \
{   T##_t AE_a = (a); \
    T##_t AE_b = (b); \
    if (!(T##_p__equal_(&AE_a, &AE_b)) ) \
    {   fprintf(stderr, "l: [" #a "] was not equal to r: [" #b "]\n [l: "); \
        T##_p__print_(stderr, &AE_a); \
        fprintf(stderr, "], [r: "); \
        T##_p__print_(stderr, &AE_b); \
        fprintf(stderr, "]\n"); \
        exit(1); \
    } \
}

#define ASSERT_POINTER_EQUAL(T, a, b) \
{   T##_t AE_a = *(a); \
    T##_t AE_b = (b); \
    if (!(type_t ## __equal_(&AE_a, &AE_b)) ) \
    {   fprintf(stderr, "l: [*(" #a ")] was not equal to r: [" #b "]\n [l: "); \
        T##_p__print_(stderr, &AE_a); \
        fprintf(stderr, "], [r: "); \
        T##_p__print_(stderr, &AE_b); \
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
