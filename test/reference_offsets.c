// gcc reference_offsets.c -lm && ./a.out
#include "test.h"
#include "stack.h"

#include <math.h>
#include <stdio.h>
#include <stdlib.h>

COMMON_C

STACK_H(float_t)
STACK_C(float_t)

STACK_H(stack_float_t)
STACK_C(stack_float_t)

TEST_C

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
    print_stack_float_t_(stdout, &stack);
    fprintf(stdout, "\n");
    ASSERT_EQUAL(float_t, pop_stack_float_t_(&stack), 3.456);
    ASSERT_EQUAL(float_t, pop_stack_float_t_(&stack), 2.345);
    ASSERT_EQUAL(uint32_t, stack.count, 1);
    descope_stack_float_t_(&stack);
    return 0;
}
