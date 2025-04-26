#pragma once

#include <math.h>
#include <stdint.h>
#include <stdio.h>

typedef float flt_t;
typedef double dbl_t;
// TODO: something like this probably, or wrap success_t in a [[nodiscard]] struct
// and do `if (ok_(my_function__success_rt_(...)))` or `if (!ok_(...))`
typedef int success_t;
// #define success_rt [[nodiscard("check this function for success!")]] success_t

typedef enum bool_t
{   false = 0,
    true = 1,
}       bool_t;

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

#define PRINT(f, T, x) \
{   T##_p__print_(f, (x)); \
    fprintf(f, "\n"); \
}

#define COMMON /*
{ */ \
IMPL \
(   void flt_p__enscope_(flt_t *flt),, \
    {   *flt = 0.0; \
    } \
) \
IMPL \
(   void flt_p__descope_(flt_t *flt),, \
    {   /* nothing to do */ \
    } \
) \
IMPL \
(   void uint32_p__enscope_(uint32_t *uint32),, \
    {   *uint32 = 0; \
    } \
) \
IMPL \
(   void uint32_p__descope_(uint32_t *uint32),, \
    {   /* nothing to do */ \
    } \
) \
IMPL \
(   bool_t uint32_p__equal_(uint32_t *a, uint32_t *b),, \
    {   return *a == *b ? true : false; \
    } \
) \
IMPL \
(   void uint32_p__print_(FILE *f, uint32_t *uint32),, \
    {   fprintf(f, "%d", *uint32); \
    } \
) \
IMPL \
(   bool_t flt_p__equal_(flt_t *a, flt_t *b),, \
    {   if ((*a != *a) && (*b != *b)) \
        {   /* we're breaking IEEE standard here but nan is nan. */ \
            return true; \
        } \
        float abs_delta = fabs(*a - *b); \
        float abs_min = fmin(fabs(*a), fabs(*b)); \
        if (abs_min > 0.0) { \
            return (abs_delta / abs_min < 1e-5) ? true : false; \
        } \
        /* if zero, then require absoluteness */ \
        return *a == *b ? true : false; \
    } \
) \
IMPL \
(   void flt_p__print_(FILE *f, flt_t *flt),, \
    {   fprintf(f, "%f", *flt); \
    } \
) \
/*
} end COMMON */

#define IMPL(fn, attr, impl) fn attr;
COMMON
#undef IMPL

#ifdef SINGLE_IMPORT
#define IMPL(fn, attr, impl) fn impl
COMMON
#undef IMPL
#endif
