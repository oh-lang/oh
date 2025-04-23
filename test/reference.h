#pragma once

#include "common.h"

// tagged field is a pointer type.
#define REFER_TAG_POINTER 0
// tagged field is a value (non-referential) type.
#define REFER_TAG_VALUE 1
// tagged field is a refer type.
#define REFER_TAG_REFER 2
// tagged field is an owned refer type.
#define REFER_TAG_OWNED_REFER 3

// TODO: smart pointers: `start` is null but `offset` is an allocated pointer
#define REFERENCE_H /*
{   */ \
    /* `reference_` definitions will need to check `start` for null, but not `offset`. */ \
    typedef void *(*reference_t_)(void *start, void *offset); \
    void *resolve_to_offset_(void *start, void *offset) ALIGN; \
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
    WARNING! `reference_` and `descope_offset_` functions *must* be `ALIGN`ed
    so that they can be used for pointer tagging.
    */ \
    typedef struct refer_t \
    {   /*
        has tag for `start` OR'd into a `reference_t_` function pointer.
        underlying function MUST BE `ALIGN`ed.
        */ \
        size_t tagged_reference; \
        word_t start; \
        /*
        has tag for `offset` OR'd into a `descope_t_` function pointer.
        underlying function MUST BE `ALIGN`ed.
        */ \
        size_t tagged_descope_offset; \
        word_t offset; \
    }       refer_t; \
    void *resolve_(refer_t *refer); \
    void *resolve_word_(uint32_t tag, word_t *word); \
    /* enscopes a NULL reference, i.e., resolving it will return NULL. */ \
    void refer_t__enscope_(refer_t *refer); \
    void refer_t__descope_(refer_t *refer); \
    /* TODO: print_ and equal_ methods. */ \
    /* 
} end REFERENCE_H */

#define REFERENCE_C /*
{   */ \
    void *resolve_to_offset_(void *start, void *offset) \
    {   return offset; \
    } \
    void *resolve_(refer_t *refer) \
    {   const size_t seven = 7; \
        uint32_t offset_tag = refer->tagged_descope_offset & seven; \
        void *offset = resolve_word_(offset_tag, &refer->offset); \
        if (offset == NULL) return NULL; \
        reference_t_ reference_ = (reference_t_)(refer->tagged_reference & ~seven); \
        uint32_t start_tag = refer->tagged_reference & seven; \
        void *start = resolve_word_(start_tag, &refer->start); \
        return reference_(start, offset); \
    } \
    void *resolve_word_(uint32_t tag, word_t *word) \
    {   switch (tag) \
        {   case REFER_TAG_POINTER: \
                /* we'll avoid making users do `**ptr` in `reference_` code. */ \
                return (void *)word->ptr; \
            case REFER_TAG_VALUE: \
                return word; \
            case REFER_TAG_REFER: \
            case REFER_TAG_OWNED_REFER: \
                return (void *)word->refer; \
            default: \
                fprintf(stderr, "invalid tagged reference: %d\n", tag); \
                exit(1); \
                return NULL; \
        } \
    } \
    void refer_t__enscope_(refer_t *refer) \
    {   refer->tagged_reference = REFER_TAG_POINTER; \
        refer->start.ptr = 0; \
        refer->tagged_descope_offset = REFER_TAG_POINTER; \
        refer->offset.ptr = 0; \
    } \
    void refer_t__descope_(refer_t *refer) \
    {   const size_t seven = 7; \
        uint32_t offset_tag = refer->tagged_descope_offset & seven; \
        if (offset_tag == REFER_TAG_OWNED_REFER) \
        {   refer_t__descope_(refer->offset.refer); \
            free(refer->offset.refer); \
        } \
        uint32_t start_tag = refer->tagged_reference & seven; \
        if (start_tag == REFER_TAG_OWNED_REFER) \
        {   refer_t__descope_(refer->start.refer); \
            free(refer->start.refer); \
        } \
        /* don't free `refer` itself, that might be stack-allocated. */ \
    } \
    /*
} end REFERENCE_C */
