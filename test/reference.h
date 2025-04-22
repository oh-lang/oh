#pragma once

#include "common.h"

// tagged field is a value (non-referential) type.
#define REFER_TAG_VALUE 0
// tagged field is a pointer type.
#define REFER_TAG_POINTER 1
// tagged field is a refer type.
#define REFER_TAG_REFER 2

#define REFERENCE_H /*
{   */ \
    typedef void *(*reference_t_)(void *start, void *offset); \
    typedef void (*descope_t_)(void *object); \
    struct refer_t; \
    typedef union \
    {   dbl_t dbl; \
        flt_t flt; \
        uint64_t uint64; \
        uint32_t uint32; \
        /* non-referential values above, referential values below. */ \
        /* see logic in `resolve_word_` for how `ptr` is used. */ \
        size_t ptr; \
        struct refer_t *refer; \
    }       word_t; \
    /*
    NOTE: we probably don't need to do runtime tagging for `refer_t`;
    we should be able to create separate types for "value"/"pointer"/"refer"
    `start`s and `offset`s.  but this  would be a nice way to do `one_of_`
    for all of the above types when at runtime we don't know what it is.
    and it would be a nice way to do `one_of_` for other referential types.
    */ \
    typedef struct refer_t \
    {   /* has tag for `start` OR'd into a `reference_t_` function pointer. */ \
        size_t tagged_reference; \
        /* has tag for `offset` OR'd into a `descope_t_` function pointer. */ \
        size_t tagged_descope_offset; \
        word_t start; \
        word_t offset; \
    }       refer_t; \
    void *resolve_(refer_t *refer); \
    void *resolve_word_(uint32_t tag, word_t *word); \
    void descope_refer_t_(refer_t *refer); \
    /* TODO: print_ and equal_ methods. */ \
    /* 
} end REFERENCE_H */

#define REFERENCE_C /*
{   */ \
    void *resolve_(refer_t *refer) \
    {   const size_t seven = 7; \
        reference_t_ reference_ = (reference_t_)(refer->tagged_reference & ~seven); \
        uint32_t start_tag = refer->tagged_reference & seven; \
        uint32_t offset_tag = refer->tagged_descope_offset & seven; \
        void *start = resolve_word_(start_tag, &refer->start); \
        void *offset = resolve_word_(offset_tag, &refer->offset); \
        return reference_(start, offset); \
    } \
    void *resolve_word_(uint32_t tag, word_t *word) \
    {   switch (tag) \
        {   case REFER_TAG_VALUE: \
                return word; \
            case REFER_TAG_POINTER: \
                /* we'll avoid making users do `**ptr` in `reference_` code. */ \
                return (void *)word->ptr; \
            case REFER_TAG_REFER: \
                return (void *)word->refer; \
            default: \
                fprintf(stderr, "invalid tagged reference: %d\n", tag); \
                exit(1); \
                return NULL; \
        } \
    } \
    void descope_refer_t_(refer_t *refer) \
    {   const size_t seven = 7; \
        uint32_t start_tag = refer->tagged_reference & seven; \
        uint32_t offset_tag = refer->tagged_descope_offset & seven; \
        if (offset_tag == REFER_TAG_REFER) \
        {   descope_refer_t_(refer->offset.refer); \
        } \
        if (start_tag == REFER_TAG_REFER) \
        {   descope_refer_t_(refer->start.refer); \
        } \
    } \
    /*
} end REFERENCE_C */
