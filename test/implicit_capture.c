// gcc implicit_capture.c && ./a.out
/*
// For oh-lang code like this:
    # TODO: we probably should have an annotation that it's ok for this
    #       `counter` to come from a temporary, because it gets captured.
    counter_(counter; u64_): fn_(): u64_
        fn_(): ++counter

    # TODO: otherwise we probably should have compile errors here if
    #       using a temporary as a reference.
    fn_(): u64_ = counter_(counter; 123)
    print_(fn_())   # should print 124
    print_(fn_())   # should print 125

    # what happens if we expect a function to have no arguments?
    repeat_twice_(fn_(): u64_): null_
        print(fn_())
        print(fn_())

    repeat_twice_(uid_generator)

    # behind the scenes we create an overload something like this:
    repeat_twice_(~ctx, fn_(ctx`): u64_): null_
        print(fn_(ctx))
        print(fn_(ctx))
*/

#include <stdint.h>
#include <stdio.h>

typedef struct counter_fn_ctx_t
{   uint64_t *counter; // TODO: should become a "reference offsets" type
}       counter_fn_ctx_t;

uint64_t counter_fn_(counter_fn_ctx_t *ctx)
{   return ++*(ctx->counter);
};

// We don't actually need to return a function pointer here, just the context.
counter_fn_ctx_t counter_(uint64_t *counter)
{   return (counter_fn_ctx_t)
    {   .counter = counter,
    };
}

// standard overload, not actually needed/used.
// we probably should avoid adding overloads that aren't used
// to match expectations from generics.
void repeat_twice_(uint64_t (*fn_)(void))
{   printf("%ld\n", fn_());
    printf("%ld\n", fn_());
}

// built-in special overload, needed for our context-requiring function.
typedef uint64_t (*u64_fn_ctx_)(void *ctx);
void repeat_twice_ctx_(u64_fn_ctx_ fn_, void *ctx)
{   printf("%ld\n", fn_(ctx));
    printf("%ld\n", fn_(ctx));
}

int main()
{   // Note we create a unique var so lifetimes are correct;
    // It's hidden from the rest of the oh-lang block.
    uint64_t counter_unique_var = 123;
    counter_fn_ctx_t fn_ctx = counter_(&counter_unique_var);
    printf("%ld\n", counter_fn_(&fn_ctx));
    printf("%ld\n", counter_fn_(&fn_ctx));

    repeat_twice_ctx_((u64_fn_ctx_)counter_fn_, (void *)&fn_ctx);
    return 0;
}

