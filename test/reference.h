#pragma once

#include "common.h"

#define REFERENCE_H /*
{   */ \
    typedef void *(*reference_t_)(void *ctx); \
    typedef struct refer_t \
    {   size_t tagged_reference; \
        union \
        {   dbl_t dbl; \
            flt_t flt; \
            uint64_t uint64; \
            uint32_t uint32; \
            void *ptr; \
            struct refer_t *refer; \
        }       ctx;\
    }       refer_t; \
    void *resolve_(refer_t refer); \
    /* 
} end REFERENCE_H */

#define REFERENCE_C /*
{   */ \
    void *resolve_(refer_t refer) \
    {   const size_t seven = 7; \
        reference_t_ reference_ = (reference_t_)(refer.tagged_reference & ~seven); \
        int tag = refer.tagged_reference & seven; \
        void *ctx = &refer.ctx; \
        switch (tag) \
        {   case 0: \
                break; \
            case 1: \
            {   refer_t *earlier_refer = refer.ctx.refer; \
                ctx = resolve_(*earlier_refer); \
                break; \
            } \
            default: \
                fprintf(stderr, "invalid tagged pointer: %d\n", tag); \
                exit(1); \
        } \
        return reference_(ctx); \
    } \
    /*
} end REFERENCE_C */
