// gcc reference_offsets.c -lm && ./a.out
#include "test.h"

#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#define STACK(data_t) \
    typedef struct stack_ ## data_t \
    {   data_t *data; \
        uint32_t capacity; \
        uint32_t count; \
    }       stack_ ## data_t;\
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

#define float_t float

void enscope_float_t_(float_t *f)
{   *f = 0.0;
}

void descope_float_t_(float_t *f)
{   // nothing to do
}

STACK(float_t)

int equal_uint32_t_(uint32_t *a, uint32_t *b)
{   return *a == *b;
}
void print_uint32_t_(FILE *f, uint32_t *uint32)
{   fprintf(f, "%d", *uint32);
}

int equal_float_t_(float_t *a, float_t *b)
{   float abs_delta = fabs(*a - *b);
    float abs_min = fmin(fabs(*a), fabs(*b));
    if (abs_min > 0.0) {
        return abs_delta / abs_min < 1e-5;
    }
    // if zero, then require absoluteness
    return *a == *b;
}
void print_float_t_(FILE *f, float_t *flt)
{   fprintf(f, "%f", *flt);
}

int main()
{   stack_float_t stack;
    enscope_stack_float_t_(&stack);
    append_default_stack_float_t_(&stack);
    append_default_stack_float_t_(&stack);
    append_default_stack_float_t_(&stack);
    ASSERT_EQUAL(uint32_t, stack.count, 3);
    stack.data[0] = 1.234;
    stack.data[1] = 2.345;
    stack.data[2] = 3.456;
    ASSERT_EQUAL(float_t, pop_stack_float_t_(&stack), 3.456);
    ASSERT_EQUAL(float_t, pop_stack_float_t_(&stack), 2.345);
    ASSERT_EQUAL(uint32_t, stack.count, 1);
    descope_stack_float_t_(&stack);
    return 0;
}
