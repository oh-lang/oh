#pragma once

#include "common.h"

// `start` is a pointer, `offset` is a value (non-referential) type.
#define REFER_POINTER_VALUE 0

#define REFERENCE_H /*
{   */ \
    typedef void *(*reference_t_)(void *start, void *offset); \
    struct refer_t; \
    typedef union \
    {   dbl_t dbl; \
        flt_t flt; \
        uint64_t uint64; \
        uint32_t uint32; \
        /* non-referential values above, referential values below. */ \
        /* see logic in `resolve_` for how `ptr` is used. */ \
        size_t ptr; \
        struct refer_t *refer; \
    }       word_t; \
    typedef struct refer_t \
    {   /*
        has tags from 0-7 OR'd into the `reference_t_` function pointer.
        */ \
        size_t tagged_reference; \
        word_t start; \
        word_t offset; \
    }       refer_t; \
    void *resolve_(refer_t *refer); \
    /* 
} end REFERENCE_H */

#define REFERENCE_C /*
{   */ \
    void *resolve_(refer_t *refer) \
    {   const size_t seven = 7; \
        reference_t_ reference_ = (reference_t_)(refer->tagged_reference & ~seven); \
        uint32_t tag = refer->tagged_reference & seven; \
        void *start; \
        void *offset; \
        switch (tag) \
        {   case REFER_POINTER_VALUE: \
                /* we'll avoid making users do `**ptr` in `reference_` code. */ \
                start = (void *)refer->start.ptr; \
                offset = &refer->offset; \
                break; \
            default: \
                fprintf(stderr, "invalid tagged reference: %d\n", tag); \
                exit(1); \
                return NULL; \
        } \
        return reference_(start, offset); \
    } \
    /*
} end REFERENCE_C */
