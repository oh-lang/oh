#pragma once

#include "common.h"

#define STACK(T, C) /*
{ */ \
OH_HI \
(,  typedef struct stack_##T##_ \
    {   T##_p data; \
        C##_t capacity; \
        C##_t count; \
    }       stack_##T##_t \
,); \
OH_HI(,OH_TYPES(stack_##T),); \
OH_HI \
(   void stack_##T##_p__descope_(stack_##T##_p stack),, \
    {   while (stack->count > 0) \
        {   C##_t index = --stack->count; \
            T##_p__descope_(&stack->data[index]); \
        } \
        free(stack->data); \
    } \
); \
OH_HI \
(   void stack_##T##_p__enscope_(stack_##T##_p stack),, \
    {   stack->data = NULL; \
        stack->capacity = 0; \
        stack->count = 0; \
    } \
); \
/* TODO: add a `stack_capacity` overload that calls this overload and exits on failure. */ \
OH_HI \
(   success_t stack_##T##_p__capacity_t__success_rt_(stack_##T##_p stack, C##_t capacity),, \
    {   while (stack->count > capacity) \
        {   T##_t element = stack_##T##_p__pop_(stack); \
            T##_p__descope_(&element); \
        } \
        T##_p data; \
        if (capacity == 0) \
        {   data = NULL; \
        } \
        else \
        {   if (stack->capacity == 0) \
            {   data = malloc(capacity * sizeof(T##_t)); \
            } \
            else \
            {   data = realloc(stack->data, capacity * sizeof(T##_t)); \
            } \
            /* check both branches that we succeeded */ \
            if (data == NULL) \
            {   return 0; \
            } \
        } \
        stack->data = data; \
        stack->capacity = capacity; \
        return 1; \
    } \
); \
OH_HI \
(   success_t stack_##T##_p__append_default_(stack_##T##_p stack),, \
    {   if (stack->count == stack->capacity) \
        {   C##_t desired_capacity = stack->capacity * 2; \
            if (desired_capacity < stack->capacity) \
            {   return 0; \
            } \
            else if (desired_capacity == 0) \
            {   desired_capacity = 2; \
            } \
            if (stack_##T##_p__capacity_t__success_rt_(stack, desired_capacity) == 0) \
            {   return 0; \
            } \
        } \
        T##_p__enscope_(&stack->data[stack->count++]); \
    } \
); \
OH_HI \
(   T##_t stack_##T##_p__pop_(stack_##T##_p stack),, \
    {   if (stack->count == 0) \
        {   fprintf(stderr, "no elements in stack\n"); exit(1); \
        } \
        return stack->data[--stack->count]; \
    } \
); \
OH_HI \
(   bool_t stack_##T##_c__equal_(stack_##T##_c a, stack_##T##_c b),, \
    {   if (a->count != b->count) \
        {   return false; \
        } \
        for (C##_t index = 0; index < a->count; ++index) \
        {   if (!(T##_c__equal_(a->data + index, b->data + index))) \
            {   return false; \
            } \
        } \
        return true; \
    } \
); \
OH_HI \
(   void stack_##T##_c__print_(FILE *f, stack_##T##_c stack),, \
    {   fprintf(f, "["); \
        for (C##_t index = 0; index < stack->count; ++index) \
        {   T##_c__print_(f, &stack->data[index]); \
            fprintf(f, ", "); \
        } \
        fprintf(f, "]"); \
    } \
); \
OH_HI \
(   T##_p stack_##T##_p__offset_p_(stack_##T##_p stack, C##_p offset), ALIGN, \
    {   if (*offset == UINT32_MAX) \
        {   fprintf(stderr, "invalid offset: %d", UINT32_MAX); \
            exit(1); \
        } \
        while (*offset >= stack->count) \
        {   stack_##T##_p__append_default_(stack); \
        } \
        return &stack->data[*offset]; \
    } \
); \
OH_HI \
(   void refer_##T##_p__enscope_from_stack_(refer_p refer, stack_##T##_p stack, C##_t offset),, \
    {   reference_f reference_ = (reference_f)stack_##T##_p__offset_p_; \
        DEBUG_ASSERT((size_t)reference_ % 8 == 0); \
        *refer = \
        (   (refer_t) \
            {   .tagged_reference = ((size_t)reference_) | REFER_TAG_POINTER, \
                .start = (word_t){ .ptr = (size_t)stack }, \
                .maybe_descope_offset = REFER_TAG_VALUE, /* no need to free. */ \
                .offset = (word_t){ . C = offset }, \
            } \
        ); \
    } \
); \
OH_HI \
(   void refer_##T##_p__enscope_from_refer_stack_(refer_p refer, refer_t refer_stack, C##_t offset),, \
    /*
    TODO: if we add a `refer_p refer_stack` overload and `refer_stack` is an owned pointer,
    we can just use `start` as a pointer that equals the owned pointer.  (i.e., we won't own it.)
    */ \
    {   reference_f reference_ = (reference_f)stack_##T##_p__offset_p_; \
        DEBUG_ASSERT((size_t)reference_ % 8 == 0); \
        refer_p nested_refer = malloc(sizeof(refer_t)); \
        *nested_refer = refer_stack; \
        *refer = \
        (   (refer_t) \
            {   .tagged_reference = ((size_t)reference_) | REFER_TAG_OWNED_REFER, \
                .start = (word_t){ .refer = nested_refer }, \
                .maybe_descope_offset = REFER_TAG_VALUE, /* no need to free. */ \
                .offset = (word_t){ . C = offset }, \
            } \
        ); \
    } \
); \
OH_HI \
(   T##_p T##_p__from__refer_p_stack_(refer_p stack),, \
    {   return (T##_p)refer__resolve_(stack); \
    } \
) \
/*
} end STACK */
