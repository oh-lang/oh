#pragma once

#include <stdint.h>

typedef float flt_t;
typedef double dbl_t;

#define ALIGN __attribute__((aligned(8)))

#define ASSERT(x) \
{   if (!(x)) \
    {   const char *E = "(" #x ") was not true, exiting!\n"; \
        fprintf(stderr, "%s", E); \
        exit(1); \
    } \
}
#ifndef NDEBUG
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
{   print_ ## type_t ## _(f, (x)); \
    fprintf(f, "\n"); \
}

#define COMMON_C /*
{ */ \
    void enscope_flt_t_(flt_t *flt) \
    {   *flt = 0.0; \
    } \
    void descope_flt_t_(flt_t *flt) \
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
    int equal_flt_t_(flt_t *a, flt_t *b) \
    {   float abs_delta = fabs(*a - *b); \
        float abs_min = fmin(fabs(*a), fabs(*b)); \
        if (abs_min > 0.0) { \
            return abs_delta / abs_min < 1e-5; \
        } \
        /* if zero, then require absoluteness */ \
        return *a == *b; \
    } \
    void print_flt_t_(FILE *f, flt_t *flt) \
    {   fprintf(f, "%f", *flt); \
    } \
    /*
} end COMMON_C */
