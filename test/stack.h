#pragma once

#include "common.h"

#define STACK_H(data_t) /*
{   */ \
    typedef struct stack_ ## data_t \
    {   data_t *data; \
        uint32_t capacity; \
        uint32_t count; \
    }       stack_ ## data_t; \
    void stack_ ## data_t ## __descope_(stack_ ## data_t *stack); \
    void stack_ ## data_t ## __enscope_(stack_ ## data_t *stack); \
    int stack_ ## data_t ## __capacity_(stack_ ## data_t *stack, uint32_t uint32); \
    int stack_ ## data_t ## __append_default_(stack_ ## data_t *stack); \
    data_t stack_ ## data_t ## __pop_(stack_ ## data_t *stack); \
    int stack_ ## data_t ## __equal_(stack_ ## data_t *a, stack_ ## data_t *b); \
    int stack_ ## data_t ## __print_(FILE *f, stack_ ## data_t *stack); \
    data_t *stack_ ## data_t ## __element_(stack_ ## data_t *stack, uint32_t *offset) ALIGN; \
    void refer_ ## data_t ## __enscope_from_stack_(refer_t *refer, stack_ ## data_t *stack, uint32_t offset); \
    void refer_ ## data_t ## __enscope_from_refer_stack_(refer_t *refer, refer_t refer_stack, uint32_t offset); \
    data_t *data_t ## __resolve_from_stack_ (refer_t *refer); \
    /*
} end STACK_H */

#define STACK_C(data_t) /*
{   */ \
    void stack_ ## data_t ## __descope_(stack_ ## data_t *stack) \
    {   while (stack->count > 0) \
        {   uint32_t index = --stack->count; \
            data_t ## __descope_(&stack->data[index]); \
        } \
        free(stack->data); \
    } \
    void stack_ ## data_t ## __enscope_(stack_ ## data_t *stack) \
    {   stack->data = NULL; \
        stack->capacity = 0; \
        stack->count = 0; \
    } \
    int stack_ ## data_t ## __capacity_(stack_ ## data_t *stack, uint32_t uint32) \
    {   while (stack->count > uint32) \
        {   data_t data = stack_ ## data_t ## __pop_(stack); \
            data_t ## __descope_(&data); \
        } \
        data_t *data; \
        if (uint32 == 0) \
        {   data = NULL; \
        } \
        else \
        {   if (stack->capacity == 0) \
            {   data = malloc(uint32 * sizeof(data_t)); \
            } \
            else \
            {   data = realloc(stack->data, uint32 * sizeof(data_t)); \
            } \
            /* check both branches that we succeeded */ \
            if (data == NULL) \
            {   return 0; \
            } \
        } \
        stack->data = data; \
        stack->capacity = uint32; \
        return 1; \
    } \
    int stack_ ## data_t ## __append_default_(stack_ ## data_t *stack) \
    {   if (stack->count == stack->capacity) \
        {   uint32_t desired_capacity = stack->capacity * 2; \
            if (desired_capacity < stack->capacity) \
            {   return 0; \
            } \
            else if (desired_capacity == 0) \
            {   desired_capacity = 2; \
            } \
            if (stack_ ## data_t ## __capacity_(stack, desired_capacity) == 0) \
            {   return 0; \
            } \
        } \
        data_t ## __enscope_(&stack->data[stack->count++]); \
    } \
    data_t stack_ ## data_t ## __pop_(stack_ ## data_t *stack) \
    {   if (stack->count == 0) \
        {   fprintf(stderr, "no elements in stack\n"); exit(1); \
        } \
        return stack->data[--stack->count]; \
    } \
    int stack_ ## data_t ## __equal_(stack_ ## data_t *a, stack_ ## data_t *b) \
    {   if (a->count != b->count) \
        {   return 0; \
        } \
        for (uint32_t index = 0; index < a->count; ++index) \
        {   if (!(data_t ## __equal_(&a->data[index], &b->data[index]))) \
            {   return 0; \
            } \
        } \
        return 1; \
    } \
    int stack_ ## data_t ## __print_(FILE *f, stack_ ## data_t *stack) \
    {   fprintf(f, "["); \
        for (uint32_t index = 0; index < stack->count; ++index) \
        {   data_t ## __print_(f, &stack->data[index]); \
            fprintf(f, ", "); \
        } \
        fprintf(f, "]"); \
    } \
    data_t *stack_ ## data_t ## __element_(stack_ ## data_t *stack, uint32_t *offset) \
    {   if (*offset == UINT32_MAX) \
        {   fprintf(stderr, "invalid offset: %d", UINT32_MAX); \
            exit(1); \
        } \
        while (*offset >= stack->count) \
        {   stack_ ## data_t ## __append_default_(stack); \
        } \
        return &stack->data[*offset]; \
    } \
    void refer_ ## data_t ## __enscope_from_stack_(refer_t *refer, stack_ ## data_t *stack, uint32_t offset) \
    {   reference_t_ reference_ = (reference_t_)stack_ ## data_t ## __element_; \
        DEBUG_ASSERT((size_t)reference_ % 8 == 0); \
        *refer = (refer_t) \
        {   .tagged_reference = ((size_t)reference_) | REFER_TAG_POINTER, \
            .start = (word_t){ .ptr = (size_t)stack }, \
            .tagged_descope_offset = REFER_TAG_VALUE, /* no need to free. */ \
            .offset = (word_t){ .uint32 = offset }, \
        }; \
    } \
    void refer_ ## data_t ## __enscope_from_refer_stack_(refer_t *refer, refer_t refer_stack, uint32_t offset) \
    {   reference_t_ reference_ = (reference_t_)stack_ ## data_t ## __element_; \
        DEBUG_ASSERT((size_t)reference_ % 8 == 0); \
        refer_t *nested_refer = malloc(sizeof(refer_t)); \
        *nested_refer = refer_stack; \
        *refer = (refer_t) \
        {   .tagged_reference = ((size_t)reference_) | REFER_TAG_REFER, \
            .start = (word_t){ .refer = nested_refer }, \
            .tagged_descope_offset = REFER_TAG_VALUE, /* no need to free. */ \
            .offset = (word_t){ .uint32 = offset }, \
        }; \
    } \
    data_t *data_t ## __resolve_from_stack_(refer_t *refer) \
    {   return (data_t *)resolve_(refer); \
    } \
    /*
} end STACK_C */
