#pragma once

#include "common.h"

#define ASSERT_EQUAL(type_t, a, b) \
{   type_t AE_a = (a); \
    type_t AE_b = (b); \
    if (!(type_t ## __equal_(&AE_a, &AE_b)) ) \
    {   fprintf(stderr, "l: [" #a "] was not equal to r: [" #b "]\n [l: "); \
        type_t ## __print_(stderr, &AE_a); \
        fprintf(stderr, "], [r: "); \
        type_t ## __print_(stderr, &AE_b); \
        fprintf(stderr, "]\n"); \
        exit(1); \
    } \
}

#define ASSERT_POINTER_EQUAL(type_t, a, b) \
{   type_t AE_a = *(a); \
    type_t AE_b = (b); \
    if (!(type_t ## __equal_(&AE_a, &AE_b)) ) \
    {   fprintf(stderr, "l: [*(" #a ")] was not equal to r: [" #b "]\n [l: "); \
        type_t ## __print_(stderr, &AE_a); \
        fprintf(stderr, "], [r: "); \
        type_t ## __print_(stderr, &AE_b); \
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
