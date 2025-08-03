#pragma once

#include "common.h"

#include <stdlib.h>

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

/*
returns a pointer to some data based on a start value and an offset.
`reference_` definitions may need to check `start` for null,
but not `offset`, which is guaranteed to be non-null.
WARNING! these functions need to be `ALIGN`ed so they can be tagged.
*/
typedef void *(*reference_f)(void *start, void *offset);
/*
destructor function.
*/
typedef void (*descope_f)(void *object);

struct refer_;
typedef union word_
{   dbl_t dbl;
    flt_t flt;
    u64_t u64;
    u32_t u32;
    u16_t u16;
    u8_t u8;
    /* non-referential values above, referential values below. */
    /* see logic in `resolve_start_` for how `ptr` is used. */
    size_t ptr;
    struct refer_ *refer;
}       word_t;
TYPES(word)

/*
NOTE: we probably don't need to do runtime tagging for `refer_t`;
we should be able to create separate types for "value"/"pointer"/"refer"
`start`s and `offset`s.  but this  would be a nice way to do `one_of_`
for all of the above types when at runtime we don't know what it is.
and it would be a nice way to do `one_of_` for other referential types.
WARNING! `reference_` functions *must* be `ALIGN`ed so that they
can be used for pointer tagging.
*/
typedef struct refer_
{   /*
    has tag for `start` OR'd into a `reference_f` function pointer.
    underlying function MUST BE `ALIGN`ed.
    */
    size_t tagged_reference;
    word_t start;
    /*
    if small (e.g., < 8), then should be interpreted as a tag for `offset`,
    otherwise should be interpreted as a `descope_f` function pointer
    for `offset` which is an owned pointer type.
    */
    size_t maybe_descope_offset;
    word_t offset;
}       refer_t;
TYPES(refer)

#define OWNED_POINTER(d) /*
{ */ \
IMPL \
(   refer_t owned__##d##_x_(d##_t data, descope_f descope_),, \
    {   d##_p owned_pointer = malloc(sizeof(d##_t)); \
        size_t reference = (size_t)refer__ignore_start_return_offset_; \
        DEBUG_ASSERT(reference % 8 == 0); \
        return (refer_t) \
        {   .tagged_reference = reference | REFER_TAG_POINTER, \
            .start = (word_t){ .ptr = 0 }, \
            .maybe_descope_offset = (size_t)descope_, \
            .offset = (word_t){ .ptr = (size_t)owned_pointer }, \
        }; \
    } \
) \
/*
} end OWNED_POINTER */

#define REFER /*
{ */ \
IMPL \
(   void *refer__ignore_start_return_offset_(void *start, void *offset), ALIGN, \
    {   return offset; \
    } \
) \
IMPL \
(   void *refer__resolve_(refer_p refer),, \
    {   const size_t seven = 7; \
        void *offset = refer__resolve_offset_(refer->maybe_descope_offset, &refer->offset); \
        if (offset == NULL) return NULL; \
        reference_f reference_ = (reference_f)(refer->tagged_reference & ~seven); \
        uint32_t start_tag = refer->tagged_reference & seven; \
        void *start = refer__resolve_start_(start_tag, &refer->start); \
        return reference_(start, offset); \
    } \
) \
IMPL \
(   void *refer__resolve_start_(uint32_t tag, word_p word),, \
    {   switch (tag) \
        {   case REFER_TAG_POINTER: \
                /* we'll avoid making users do `**ptr` in `reference_` code. */ \
                return (void *)word->ptr; \
            case REFER_TAG_VALUE: \
                return word; \
            case REFER_TAG_REFER: \
            case REFER_TAG_OWNED_REFER: \
                return refer__resolve_(word->refer); \
            default: \
                fprintf(stderr, "invalid tagged reference: %d\n", tag); \
                exit(1); \
                return NULL; \
        } \
    } \
)   \
IMPL \
(   void *refer__resolve_offset_(size_t maybe_descope_offset, word_p word),, \
    {   switch (maybe_descope_offset) \
        {   case REFER_TAG_POINTER: \
                /* we'll avoid making users do `**ptr` in `reference_` code. */ \
                return (void *)word->ptr; \
            case REFER_TAG_VALUE: \
                return word; \
            case REFER_TAG_REFER: \
            case REFER_TAG_OWNED_REFER: \
                return refer__resolve_(word->refer); \
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
) \
IMPL \
(   /* enscopes a NULL reference, i.e., resolving it will return NULL. */ \
    void refer_p__enscope_(refer_p refer),, \
    {   refer->tagged_reference = REFER_TAG_POINTER; \
        refer->start.ptr = 0; \
        refer->maybe_descope_offset = REFER_TAG_POINTER; \
        refer->offset.ptr = 0; \
    } \
) \
IMPL \
(   void refer_p__descope_(refer_p refer),, \
    {   switch (refer->maybe_descope_offset) \
        {   case REFER_TAG_POINTER: /* an unowned pointer */ \
            case REFER_TAG_VALUE: /* a value we don't need to free */ \
            case REFER_TAG_REFER: /* an unowned refer */ \
                break; \
            case REFER_TAG_OWNED_REFER: \
                refer_p__descope_(refer->offset.refer); \
                free(refer->offset.refer); \
                break; \
            case REFER_TAG_RESERVED0: \
            case REFER_TAG_RESERVED1: \
            case REFER_TAG_RESERVED2: \
            case REFER_TAG_RESERVED3: \
                break; \
            default: \
            {   descope_f descope_ = (descope_f)(refer->maybe_descope_offset); \
                void *ptr = (void *)refer->offset.ptr; \
                descope_(ptr); \
                free(ptr); \
            } \
        } \
        const size_t seven = 7; \
        uint32_t start_tag = refer->tagged_reference & seven; \
        if (start_tag == REFER_TAG_OWNED_REFER) \
        {   refer_p__descope_(refer->start.refer); \
            free(refer->start.refer); \
        } \
        /* don't free `refer` itself, that might be stack-allocated. */ \
    } \
) \
    /* TODO: print_ and equal_ methods. */ \
    /* 
} end REFER */

#define IMPL(fn, attr, impl) IMPL_DECLARE(fn, attr, impl)
REFER
#undef IMPL

#ifdef SINGLE_IMPORT
#define IMPL(fn, attr, impl) IMPL_DEFINE(fn, attr, impl)
REFER
#undef IMPL
#endif
