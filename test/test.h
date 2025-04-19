#pragma once

#define ASSERT_EQUAL(type_t, a, b) \
    type_t AE_a = (a); \
    type_t AE_b = (b); \
    if (AE_a != AE_b) \
    {   fprintf(stderr, #a " was not equal to " #b "\n"); \
        exit(1); \
    }
