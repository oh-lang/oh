// gcc reference_offsets.c -lm && ./a.out
#include "test.h"
#include "reference.h"
#include "stack.h"

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
{   append_default_stack_stack_flt_t_(stack_stack);
    append_default_stack_stack_flt_t_(stack_stack);
    ASSERT_EQUAL(uint32_t, stack_stack->count, 2);
    stack_flt_t *stack = &stack_stack->data[1];
    append_default_stack_flt_t_(stack);
    append_default_stack_flt_t_(stack);
    append_default_stack_flt_t_(stack);
    stack->data[0] = 1.234;
    stack->data[1] = 2.345;
    stack->data[2] = 3.456;
    ASSERT_EQUAL(uint32_t, stack->count, 3);
}

int main()
{   stack_stack_flt_t stack_stack;
    enscope_stack_stack_flt_t_(&stack_stack);
    add_values_(&stack_stack);
    print_stack_stack_flt_t_(stdout, &stack_stack);
    fprintf(stdout, "\n");
    stack_flt_t stack = pop_stack_stack_flt_t_(&stack_stack);
    print_stack_flt_t_(stdout, &stack);
    fprintf(stdout, "\n");
    ASSERT_EQUAL(flt_t, pop_stack_flt_t_(&stack), 3.456);
    ASSERT_EQUAL(flt_t, pop_stack_flt_t_(&stack), 2.345);
    ASSERT_EQUAL(uint32_t, stack.count, 1);

    descope_stack_stack_flt_t_(&stack_stack);
    descope_stack_flt_t_(&stack);
    return 0;
}
