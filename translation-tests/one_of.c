// gcc one_of.c && ./a.out

#include <stdint.h>
#include <stdio.h>
typedef double dbl_t;

// oh-lang code like this:
// `status_: one_of_[unknown, invalid, ready]`
// always gets split into a tag enum and a struct, even when the
// `one_of_` would only need an enum and no union in the struct.
typedef enum FILE_TAG__status_
{   FILE_TAG__status__unknown,
    FILE_TAG__status__invalid,
    FILE_TAG__status__ready,
}       FILE_TAG__status_t;
typedef struct FILE__status_
{   // this tag is namespaced because it's not available to oh-lang code.
    // only need 2 bits because the `FILE_TAG__status_` enum fits in 2 bits.
    uint8_t OH_TAG : 2;
}         FILE__status_t;

// oh-lang code:
// `update_: one_of_[status;, position_x; dbl_, position_y; dbl_, position_z; dbl_, speed; dbl_]`
typedef enum FILE_TAG__update_
{   FILE_TAG__update__status,
    FILE_TAG__update__position_x,
    FILE_TAG__update__position_y,
    FILE_TAG__update__position_z,
    FILE_TAG__update__speed,
}       FILE_TAG__update_t;
typedef struct FILE__update_
{   union
    {   FILE__status_t status;
        dbl_t position_x;
        dbl_t position_y; 
        dbl_t position_z; 
        dbl_t speed; 
    };
    // this tag is namespaced because it's not available to oh-lang code.
    // need 3 bits because the `FILE_TAG__update_` enum fits in 3 bits.
    uint8_t OH_TAG : 3;
}         FILE__update_t;

// oh-lang code:
// `multi_status_: one_of_[parent_status; status_, child_status; status_]`
typedef enum FILE_TAG__multi_status_
{   FILE_TAG__multi_status__parent_status,
    FILE_TAG__multi_status__child_status,
}       FILE_TAG__multi_status_t;
typedef struct FILE__multi_status_
{   union
    {   FILE__status_t parent_status;
        FILE__status_t child_status;
    };
    // this tag is namespaced because it's not available to oh-lang code.
    // need 1 bit because the `FILE_TAG__multi_status_` enum fits in 1 bit.
    uint8_t OH_TAG : 1;
}         FILE__multi_status_t;

int main()
{   printf("status one_of has size %ld\n", sizeof(FILE__status_t));
    printf("update one_of has size %ld\n", sizeof(FILE__update_t));
    // NOTE: this is a bit surprising (should be 2 bytes), but it does mean
    // that we don't need to worry about keeping `OH_TAG` persistent when
    // users update `parent_status` or `child_status` with a `u8_ *` pointer.
    // We could improve this by using a `u8_ *` pointer along with a bit mask
    // for many operations, but we'd want to avoid slowing down operations
    // on larger memory structs which don't need that.
    printf("multi_status one_of has size %ld\n", sizeof(FILE__multi_status_t));
    return 0;
}

