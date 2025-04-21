#pragma once

#include "common.h"

#define REFERENCE_H /*
{   */ \
    typedef size_t tagged_t; \
    typedef void *(*reference_t_)(void *ctx); \
    typedef struct refer_t \
    {   reference_t_ reference_; \
        tagged_t tagged_ctx; \
    }       refer_t; \
    void *resolve_(refer_t refer); \
    /* 
} end REFERENCE_H */

#define REFERENCE_C /*
{   */ \
    void *resolve_(refer_t refer) \
    {   const size_t seven = 7; \
        void *ctx = (void *)(refer.tagged_ctx & ~seven); \
        int tag = refer.tagged_ctx & seven; \
        switch (tag) \
            case 0: \
                break; \
            case 1: \
            {   refer_t *earlier_refer = (refer_t *)ctx; \
                ctx = resolve_(*earlier_refer); \
                break; \
            } \
            default: \
                fprintf(stderr, "invalid tagged pointer: %d\n", tag); \
                exit(1); \
        } \
        return refer.reference_(ctx); \
    } \
    /*
} end REFERENCE_C */
