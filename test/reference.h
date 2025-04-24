#pragma once

#include "common.h"

// tagged field is a pointer type.
#define REFER_TAG_POINTER 0
// tagged field is a value (non-referential) type.
#define REFER_TAG_VALUE 1
// tagged field is an unowned refer type.
#define REFER_TAG_REFER 2
// tagged field is an owned refer type.
#define REFER_TAG_OWNED_REFER 3
// some reserved tags.
#define REFER_TAG_RESERVED0 4
#define REFER_TAG_RESERVED1 5
#define REFER_TAG_RESERVED2 6
#define REFER_TAG_RESERVED3 7

#define OWNED_POINTER_H(data_t) /*
{   */ \
    refer_t refer_ ## data_t ## _owned_(data_t data, descope_t_ descope_); \
    /*
} end OWNED_POINTER_H */
#define OWNED_POINTER_C(data_t) /*
{   */ \
    refer_t refer_ ## data_t ## _owned_(data_t data, descope_t_ descope_) \
    {   DEBUG_ASSERT((size_t)descope_ % 8 == 0); \
        data_t *owned_pointer = malloc(sizeof(data_t)); \
        return (refer_t) \
        {   .tagged_reference = REFER_TAG_POINTER, \
            .start = (word_t){ .ptr = 0 }, \
            .maybe_descope_offset = (size_t)descope_, \
            .offset = (word_t){ .ptr = (size_t)owned_pointer }, \
        }; \
    } \
    /*
} end OWNED_POINTER_C */

#define REFERENCE_H /*
{   */ \
    /*
    `reference_` definitions may need to check `start` for null,
    but not `offset`, which is guaranteed to be non-null.
    WARNING! these functions need to be `ALIGN`ed so they can be tagged.
    */ \
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
        /* see logic in `resolve_start_` for how `ptr` is used. */ \
        size_t ptr; \
        struct refer_t *refer; \
    }       word_t; \
    /*
    NOTE: we probably don't need to do runtime tagging for `refer_t`;
    we should be able to create separate types for "value"/"pointer"/"refer"
    `start`s and `offset`s.  but this  would be a nice way to do `one_of_`
    for all of the above types when at runtime we don't know what it is.
    and it would be a nice way to do `one_of_` for other referential types.
    WARNING! `reference_` functions *must* be `ALIGN`ed so that they
    can be used for pointer tagging.
    */ \
    typedef struct refer_t \
    {   /*
        has tag for `start` OR'd into a `reference_t_` function pointer.
        underlying function MUST BE `ALIGN`ed.
        */ \
        size_t tagged_reference; \
        word_t start; \
        /*
        if small (e.g., < 8), then should be interpreted as a tag for `offset`,
        otherwise should be interpreted as a `descope_t_` function pointer
        for `offset` which is an owned pointer type.
        */ \
        size_t maybe_descope_offset; \
        word_t offset; \
    }       refer_t; \
    void *resolve_(refer_t *refer); \
    void *refer_resolve_start_(uint32_t tag, word_t *word); \
    void *refer_resolve_offset_(size_t maybe_descope_offset, word_t *word); \
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
        void *offset = refer_resolve_offset_(refer->maybe_descope_offset, &refer->offset); \
        if (offset == NULL) return NULL; \
        reference_t_ reference_ = (reference_t_)(refer->tagged_reference & ~seven); \
        uint32_t start_tag = refer->tagged_reference & seven; \
        void *start = refer_resolve_start_(start_tag, &refer->start); \
        return reference_(start, offset); \
    } \
    void *refer_resolve_start_(uint32_t tag, word_t *word) \
    {   switch (tag) \
        {   case REFER_TAG_POINTER: \
                /* we'll avoid making users do `**ptr` in `reference_` code. */ \
                return (void *)word->ptr; \
            case REFER_TAG_VALUE: \
                return word; \
            case REFER_TAG_REFER: \
            case REFER_TAG_OWNED_REFER: \
                return resolve_(word->refer); \
            default: \
                fprintf(stderr, "invalid tagged reference: %d\n", tag); \
                exit(1); \
                return NULL; \
        } \
    } \
    void *refer_resolve_offset_(size_t maybe_descope_offset, word_t *word) \
    {   switch (maybe_descope_offset) \
        {   case REFER_TAG_POINTER: \
                /* we'll avoid making users do `**ptr` in `reference_` code. */ \
                return (void *)word->ptr; \
            case REFER_TAG_VALUE: \
                return word; \
            case REFER_TAG_REFER: \
            case REFER_TAG_OWNED_REFER: \
                return resolve_(word->refer); \
            case REFER_TAG_RESERVED0: \
            case REFER_TAG_RESERVED1: \
            case REFER_TAG_RESERVED2: \
            case REFER_TAG_RESERVED3: \
                fprintf(stderr, "invalid tagged reference: %d\n", (uint32_t)maybe_descope_offset); \
                exit(1); \
                return NULL; \
            default: \
                /* this is an owned pointer */ \
                return (void *)word->ptr; \
        } \
    } \
    void refer_t__enscope_(refer_t *refer) \
    {   refer->tagged_reference = REFER_TAG_POINTER; \
        refer->maybe_descope_offset = REFER_TAG_POINTER; \
    } \
    void refer_t__descope_(refer_t *refer) \
    {   const size_t seven = 7; \
        switch (refer->maybe_descope_offset) \
        {   case REFER_TAG_POINTER: /* an unowned pointer */ \
            case REFER_TAG_VALUE: /* a value we don't need to free */ \
            case REFER_TAG_REFER: /* an unowned refer */ \
                break; \
            case REFER_TAG_OWNED_REFER: \
                refer_t__descope_(refer->offset.refer); \
                free(refer->offset.refer); \
                break; \
            case REFER_TAG_RESERVED0: \
            case REFER_TAG_RESERVED1: \
            case REFER_TAG_RESERVED2: \
            case REFER_TAG_RESERVED3: \
                break; \
            default: \
            {   descope_t_ descope_ = (descope_t_)(refer->maybe_descope_offset); \
                void *ptr = (void *)refer->offset.ptr; \
                descope_(ptr); \
                free(ptr); \
            } \
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
