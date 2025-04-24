// gcc reference_offsets.c -lm && ./a.out
#define ILL_DO_IT_MYSELF
#include "../c/test.h"
#include "../c/reference.h"
#include "../c/stack.h"

#include <math.h>
#include <stdio.h>
#include <stdlib.h>

COMMON_C

REFERENCE_H
REFERENCE_C

STACK_H(flt_t)
STACK_C(flt_t)

STACK_H(stack_flt_t)
STACK_C(stack_flt_t)

void add_values_(stack_stack_flt_t *stack_stack)
{   uint32_t offset = 1;
    stack_flt_t *stack = stack_stack_flt_t__element_(stack_stack, &offset);
    ASSERT_EQUAL(uint32_t, stack_stack->count, 2);
    stack_flt_t__append_default_(stack);
    stack_flt_t__append_default_(stack);
    stack_flt_t__append_default_(stack);
    stack->data[0] = 1.234;
    stack->data[1] = 2.345;
    stack->data[2] = 3.456;
    ASSERT_EQUAL(uint32_t, stack->count, 3);
}

int main()
{   stack_stack_flt_t stack_stack;
    stack_stack_flt_t__enscope_(&stack_stack);
    add_values_(&stack_stack);
    PRINT(stdout, stack_stack_flt_t, &stack_stack);
    stack_flt_t stack = stack_stack_flt_t__pop_(&stack_stack);
    PRINT(stdout, stack_flt_t, &stack);
    ASSERT_EQUAL(flt_t, stack_flt_t__pop_(&stack), 3.456);
    ASSERT_EQUAL(flt_t, stack_flt_t__pop_(&stack), 2.345);
    ASSERT_EQUAL(uint32_t, stack.count, 1);

    refer_t flt;
    refer_flt_t__enscope_from_stack_(&flt, &stack, 10);
    ASSERT_EQUAL(uint32_t, stack.count, 1); // doesn't change immediately
    *flt_t__resolve_from_stack_(&flt) = 10.20;
    ASSERT_EQUAL(uint32_t, stack.count, 11);
    PRINT(stdout, stack_flt_t, &stack);

    stack_stack_flt_t__descope_(&stack_stack);
    stack_flt_t__descope_(&stack);
    return 0;
}
