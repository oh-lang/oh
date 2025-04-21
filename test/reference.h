#pragma once

#include "common.h"

#define REFERENCE_H /*
{   */ \
    typedef void *(*reference_t_)(void *start, void *offset); \
    struct refer_t; \
    typedef union \
    {   dbl_t dbl; \
        flt_t flt; \
        uint64_t uint64; \
        uint32_t uint32; \
        struct refer_t *refer; \
    }       word_t; \
    typedef struct refer_t \
    {   size_t tagged_reference; \
        word_t start; \
        word_t offset; \
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
        if (tag > 3) \
        {   fprintf(stderr, "invalid tagged pointer: %d\n", tag); \
            exit(1); \
        } \
        void *start = &refer.start; \
        if (tag & 1) \
        {   refer_t *nested_refer = refer.start.refer; \
            start = resolve_(*nested_refer); \
        } \
        void *offset = &refer.offset; \
        if (tag & 2) \
        {   refer_t *nested_refer = refer.offset.refer; \
            offset = resolve_(*nested_refer); \
        } \
        return reference_(start, offset); \
    } \
    /*
} end REFERENCE_C */
