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
        /* see logic in `resolve_word_` for how `ptr` is used. */ \
        size_t ptr; \
        struct refer_t *refer; \
    }       word_t; \
    void *resolve_word_(uint32_t tag, word_t *word); \
    typedef struct refer_t \
    {   size_t tagged_reference; \
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
        void *start = resolve_word_(tag & 3, &refer->start); \
        void *offset = resolve_word_(tag >> 2, &refer->offset); \
        return reference_(start, offset); \
    } \
    void *resolve_word_(uint32_t tag, word_t *word) \
    {   switch (tag & 3) \
        {   case 0: \
                return word; \
            case 1: \
                /* we'll avoid making users do `**ptr` in `reference_` code. */ \
                return (void *)word->ptr; \
            case 2: \
                /* nesting... */ \
                return resolve_(word->refer); \
            case 3: \
                fprintf(stderr, "invalid tagged word: %d\n", tag); \
                exit(1); \
        } \
    } \
    /*
} end REFERENCE_C */
