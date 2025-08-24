#pragma once

#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

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

#define OH_HI_HEADER(fn, attr, impl) fn attr
#define OH_HI_IMPL(fn, attr, impl) fn impl

#define OH_TYPES(t) \
typedef t##_t *t##_p; \
typedef const t##_t *t##_c;

#define PRIMITIVE_TYPE(t, zero, fmt) \
OH_HI(,OH_TYPES(t),); \
OH_HI \
(   void t##_p__enscope_(t##_p number),, \
    {   *number = zero; \
    } \
); \
OH_HI \
(   void t##_p__descope_(t##_p number),, \
    {   /* nothing to do */ \
    } \
); \
OH_HI \
(   void t##_c__print_(FILE *f, t##_c number),, \
    {   fprintf(f, fmt, *number); \
    } \
)

#define PRIMITIVE_PERFECT_EQUALITY(t) \
OH_HI \
(   bool_t t##_c__equal_(t##_c a, t##_c b),, \
    {   return *a == *b ? true : false; \
    } \
)

#define PRIMITIVE_APPROXIMATE_EQUALITY(t, ABS, MIN, EPSILON) \
OH_HI \
(   bool_t t##_c__equal_(t##_c a, t##_c b),, \
    {   if ((*a != *a) && (*b != *b)) \
        {   /* we're breaking IEEE standard here but nan is NaN. */ \
            return true; \
        } \
        t##_t abs_delta = ABS(*a - *b); \
        t##_t abs_min = MIN(ABS(*a), ABS(*b)); \
        if (abs_min > 0.0) { \
            return (abs_delta / abs_min < EPSILON) ? true : false; \
        } \
        /* if zero, then require absoluteness */ \
        return *a == *b ? true : false; \
    } \
)

#define ALIGN __attribute__((aligned(8)))

// TODO: redo as `print_ ## T ## _c_`
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
PRIMITIVE_TYPE(flt, 0.0, "%f"); \
PRIMITIVE_APPROXIMATE_EQUALITY(flt, fabs, fmin, 1e-5); \
PRIMITIVE_TYPE(dbl, 0.0, "%lf"); \
OH_HI(dbl_t dmin(dbl_t a, dbl_t b),, { return a < b ? a : b; }); \
OH_HI(dbl_t dmax(dbl_t a, dbl_t b),, { return a > b ? a : b; }); \
PRIMITIVE_APPROXIMATE_EQUALITY(dbl, abs, dmin, 1e-7); \
PRIMITIVE_TYPE(u64, 0, "%ld"); \
PRIMITIVE_PERFECT_EQUALITY(u64); \
PRIMITIVE_TYPE(u32, 0, "%d"); \
PRIMITIVE_PERFECT_EQUALITY(u32); \
PRIMITIVE_TYPE(u16, 0, "%d"); \
PRIMITIVE_PERFECT_EQUALITY(u16); \
PRIMITIVE_TYPE(u8, 0, "%d"); \
PRIMITIVE_PERFECT_EQUALITY(u8); \
/*
} end COMMON */

// header file:
#define OH_HI(fn, attr, impl) OH_HI_HEADER(fn, attr, impl)
COMMON
#undef OH_HI

// header + implementation file:
#ifdef SINGLE_IMPORT
#define OH_HI(fn, attr, impl) OH_HI_IMPL(fn, attr, impl)
COMMON
#undef OH_HI

#endif
