#pragma once

#include <stdint.h>
#include <stdio.h>

typedef float flt_t;
typedef double dbl_t;

#define ALIGN __attribute__((aligned(8)))

#ifndef NDEBUG
#define DEBUG
#else
#define RELEASE
#endif

// release and debug asserts
#define ASSERT(x) \
{   if (!(x)) \
    {   const char *E = "(" #x ") was not true, exiting!\n"; \
        fprintf(stderr, "%s", E); \
        exit(1); \
    } \
}
#ifdef DEBUG
// just debug asserts
#define DEBUG_ASSERT(x) \
{   if (!(x)) \
    {   const char *E = "(" #x ") was not true (in debug), exiting!\n"; \
        fprintf(stderr, "%s", E); \
        exit(1); \
    } \
}
#else
#define DEBUG_ASSERT(x) {}
#endif

#define PRINT(f, type_t, x) \
{   type_t ## __print_(f, (x)); \
    fprintf(f, "\n"); \
}

#define COMMON /*
{ */ \
IMPL \
(   void flt_t__enscope_(flt_t *flt),, \
    {   *flt = 0.0; \
    } \
) \
IMPL \
(   void flt_t__descope_(flt_t *flt),, \
    {   /* nothing to do */ \
    } \
) \
IMPL \
(   void uint32_t__enscope_(uint32_t *uint32),, \
    {   *uint32 = 0; \
    } \
) \
IMPL \
(   void uint32_t__descope_(uint32_t *uint32),, \
    {   /* nothing to do */ \
    } \
) \
IMPL \
(   int uint32_t__equal_(uint32_t *a, uint32_t *b),, \
    {   return *a == *b; \
    } \
) \
IMPL \
(   void uint32_t__print_(FILE *f, uint32_t *uint32),, \
    {   fprintf(f, "%d", *uint32); \
    } \
) \
IMPL \
(   int flt_t__equal_(flt_t *a, flt_t *b),, \
    {   if ((*a != *a) && (*b != *b)) \
        {   /* we're breaking IEEE standard here but nan is nan. */ \
            return 1; \
        } \
        float abs_delta = fabs(*a - *b); \
        float abs_min = fmin(fabs(*a), fabs(*b)); \
        if (abs_min > 0.0) { \
            return abs_delta / abs_min < 1e-5; \
        } \
        /* if zero, then require absoluteness */ \
        return *a == *b; \
    } \
) \
IMPL \
(   void flt_t__print_(FILE *f, flt_t *flt),, \
    {   fprintf(f, "%f", *flt); \
    } \
) \
/*
} end COMMON */

#define IMPL(fn, attr, impl) fn;
COMMON
#undef IMPL
