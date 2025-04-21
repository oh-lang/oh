#pragma once

#include "common.h"

#define STACK_H(data_t) /*
{   */ \
    typedef struct stack_ ## data_t \
    {   data_t *data; \
        uint32_t capacity; \
        uint32_t count; \
    }       stack_ ## data_t; \
    void descope_stack_ ## data_t ## _(stack_ ## data_t *stack); \
    void enscope_stack_ ## data_t ## _(stack_ ## data_t *stack); \
    int capacity_stack_ ## data_t ## _(stack_ ## data_t *stack, uint32_t uint32); \
    int append_default_stack_ ## data_t ## _(stack_ ## data_t *stack); \
    data_t pop_stack_ ## data_t ## _(stack_ ## data_t *stack); \
    int equal_stack_ ## data_t ## _(stack_ ## data_t *a, stack_ ## data_t *b); \
    int print_stack_ ## data_t ## _(FILE *f, stack_ ## data_t *stack); \
    data_t *element_stack_ ## data_t ## _(stack_ ## data_t *stack, uint32_t *offset); \
    refer_t refer_stack_ ## data_t ## _(stack_ ## data_t *stack, uint32_t offset); \
    /*
} end STACK_H */

#define STACK_C(data_t) /*
{   */ \
    void descope_stack_ ## data_t ## _(stack_ ## data_t *stack) \
    {   while (stack->count > 0) \
        {   uint32_t index = --stack->count; \
            descope_ ## data_t ## _(&stack->data[index]); \
        } \
        free(stack->data); \
    } \
    void enscope_stack_ ## data_t ## _(stack_ ## data_t *stack) \
    {   stack->data = NULL; \
        stack->capacity = 0; \
        stack->count = 0; \
    } \
    int capacity_stack_ ## data_t ## _(stack_ ## data_t *stack, uint32_t uint32) \
    {   while (stack->count > uint32) \
        {   data_t data = pop_stack_ ## data_t ## _(stack); \
            descope_ ## data_t ## _(&data); \
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
    int append_default_stack_ ## data_t ## _(stack_ ## data_t *stack) \
    {   if (stack->count == stack->capacity) \
        {   uint32_t desired_capacity = stack->capacity * 2; \
            if (desired_capacity < stack->capacity) \
            {   return 0; \
            } \
            else if (desired_capacity == 0) \
            {   desired_capacity = 2; \
            } \
            if (capacity_stack_ ## data_t ## _(stack, desired_capacity) == 0) \
            {   return 0; \
            } \
        } \
        enscope_ ## data_t ## _(&stack->data[stack->count++]); \
    } \
    data_t pop_stack_ ## data_t ## _(stack_ ## data_t *stack) \
    {   if (stack->count == 0) \
        {   fprintf(stderr, "no elements in stack\n"); exit(1); \
        } \
        return stack->data[--stack->count]; \
    } \
    int equal_stack_ ## data_t ## _(stack_ ## data_t *a, stack_ ## data_t *b) \
    {   if (a->count != b->count) \
        {   return 0; \
        } \
        for (uint32_t index = 0; index < a->count; ++index) \
        {   if (!(equal_ ## data_t ## _(&a->data[index], &b->data[index]))) \
            {   return 0; \
            } \
        } \
        return 1; \
    } \
    int print_stack_ ## data_t ## _(FILE *f, stack_ ## data_t *stack) \
    {   fprintf(f, "["); \
        for (uint32_t index = 0; index < stack->count; ++index) \
        {   print_ ## data_t ## _(f, &stack->data[index]); \
            fprintf(f, ", "); \
        } \
        fprintf(f, "]"); \
    } \
    data_t *element_stack_ ## data_t ## _(stack_ ## data_t *stack, uint32_t *offset) \
    {   if (*offset == UINT32_MAX) \
        {   fprintf(stderr, "invalid offset: %d", UINT32_MAX); \
            exit(1); \
        } \
        while (*offset >= stack->count) \
        {   append_default_stack_ ## data_t ## _(stack); \
        } \
        return &stack->data[*offset]; \
    } \
    refer_t refer_stack_ ## data_t ## _(stack_ ## data_t *stack, uint32_t offset) \
    {   reference_t_ reference_ = (reference_t_)element_stack_ ## data_t ## _; \
        return (refer_t) \
        {   .tagged_reference = ((size_t)reference_) | REFER_POINTER_VALUE, \
            .start = (word_t){ .ptr = (size_t)stack }, \
            .offset = (word_t){ .uint32 = offset }, \
        }; \
    } \
    /*
} end STACK_C */
