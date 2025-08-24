// gcc reference_offsets.c -lm && ./a.out
#define SINGLE_IMPORT
#include "../c/test.h"
#include "../c/refer.h"
#include "../c/stack.h"

#include <math.h>
#include <stdio.h>
#include <stdlib.h>

#define OH_HI(fn, attr, impl) OH_HI_HEADER(fn, attr, impl)
STACK(flt, u32);
STACK(stack_flt, u8);
#undef OH_HI
#define OH_HI(fn, attr, impl) OH_HI_IMPL(fn, attr, impl)
STACK(flt, u32);
STACK(stack_flt, u8);
#undef OH_HI

void add_values_(stack_stack_flt_p stack_stack)
{   u8_t offset = 1;
    stack_flt_p stack = stack_stack_flt_p__offset_p_(stack_stack, &offset);
    ASSERT_EQUAL(u32, stack_stack->count, 2);
    stack_flt_p__append_default_(stack);
    stack_flt_p__append_default_(stack);
    stack_flt_p__append_default_(stack);
    stack->data[0] = 1.234;
    stack->data[1] = 2.345;
    stack->data[2] = 3.456;
    ASSERT_EQUAL(u32, stack->count, 3);
}

int main()
{   stack_stack_flt_t stack_stack;
    stack_stack_flt_p__enscope_(&stack_stack);
    add_values_(&stack_stack);
    PRINT(stdout, stack_stack_flt, &stack_stack);
    stack_flt_t stack = stack_stack_flt_p__pop_(&stack_stack);
    PRINT(stdout, stack_flt, &stack);
    ASSERT_EQUAL(flt, stack_flt_p__pop_(&stack), 3.456);
    ASSERT_EQUAL(flt, stack_flt_p__pop_(&stack), 2.345);
    ASSERT_EQUAL(u32, stack.count, 1);

    refer_t flt;
    refer_flt_p__enscope_from_stack_(&flt, &stack, 10);
    ASSERT_EQUAL(u32, stack.count, 1); // doesn't change immediately
    *flt_p__from__refer_p_stack_(&flt) = 10.20;
    ASSERT_EQUAL(u32, stack.count, 11);
    PRINT(stdout, stack_flt, &stack);

    stack_stack_flt_p__descope_(&stack_stack);
    stack_flt_p__descope_(&stack);
    return 0;
}
