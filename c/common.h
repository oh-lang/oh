#pragma once

#include <math.h>
#include <stdint.h>
#include <stdio.h>

// TODO: switch to [[nodiscard]] with C23:
// enum [[nodiscard]] success_t { ... };
typedef enum success_
{   er = 0,
    ok = 1,
}       success_t;

typedef enum bool_
{   false = 0,
    true = 1,
}       bool_t;

typedef float flt_t;
typedef double dbl_t;
typedef uint8_t u8_t;
typedef uint16_t u16_t;
typedef uint32_t u32_t;
typedef uint64_t u64_t;

#define TYPES(t) \
typedef t##_t *t##_p; \
typedef const t##_t *t##_c;

#define PRIMITIVE_TYPE(t, zero, fmt) \
IMPL(,TYPES(t),) \
IMPL \
(   void t##_p__enscope_(t##_p number),, \
    {   *number = zero; \
    } \
) \
IMPL \
(   void t##_p__descope_(t##_p number),, \
    {   /* nothing to do */ \
    } \
) \
IMPL \
(   void t##_c__print_(FILE *f, t##_c number),, \
    {   fprintf(f, fmt, *number); \
    } \
) \

#define PRIMITIVE_PERFECT_EQUALITY(t) \
IMPL \
(   bool_t t##_c__equal_(t##_c a, t##_c b),, \
    {   return *a == *b ? true : false; \
    } \
) \

#define ALIGN __attribute__((aligned(8)))

#define PRINT(f, T, x) \
{   T##_c__print_(f, (x)); \
    fprintf(f, "\n"); \
}

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

#define COMMON /*
{ */ \
PRIMITIVE_TYPE(flt, 0.0, "%f") \
IMPL \
(   bool_t flt_c__equal_(flt_c a, flt_c b),, \
    {   if ((*a != *a) && (*b != *b)) \
        {   /* we're breaking IEEE standard here but nan is NaN. */ \
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
PRIMITIVE_TYPE(dbl, 0.0, "%lf") \
PRIMITIVE_TYPE(u64, 0, "%ld") \
PRIMITIVE_PERFECT_EQUALITY(u64) \
PRIMITIVE_TYPE(u32, 0, "%d") \
PRIMITIVE_PERFECT_EQUALITY(u32) \
PRIMITIVE_TYPE(u16, 0, "%d") \
PRIMITIVE_PERFECT_EQUALITY(u16) \
PRIMITIVE_TYPE(u8, 0, "%d") \
PRIMITIVE_PERFECT_EQUALITY(u8) \
/*
} end COMMON */

// header file:
#define IMPL(fn, attr, impl) fn attr;
COMMON
#undef IMPL

// header + implementation file:
#ifdef SINGLE_IMPORT
#define IMPL(fn, attr, impl) fn impl
COMMON
#undef IMPL
#endif
