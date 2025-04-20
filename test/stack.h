#pragma once

#include "common.h"

#define STACK_H(data_t) \
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
/* end STACK_H */

#define STACK_C(data_t) \
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
    {   data_t *data; \
        if (stack->capacity == 0) \
        {   data = malloc(uint32 * sizeof(data_t)); \
        } \
        else \
        {   data = realloc(stack->data, uint32 * sizeof(data_t)); \
        } \
        if (data == NULL) \
        {   return 0; \
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
/* end STACK_C */
