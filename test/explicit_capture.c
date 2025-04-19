// gcc explicit_capture.c && ./a.out
/*
// For oh-lang code like this:
    outer_(u64.): fn_(): u64_
        inner_(ctx. [counter; u64_, xorer: u64_]): u64_
            ++ctx counter
            ctx counter >< ctx xorer
        inner_ with ctx. [counter. u64, xorer: 12345]

    # we don't even need a function pointer in the raw C code here,
    # we can just pass the `ctx` around as if it were a function:
    uid_generator_(): u64 = outer_(456)
    print_(uid_generator_())
    print_(uid_generator_())

    # what happens if we expect a function to have no arguments?
    needs_ids_(fn_(): u64_): null_
        print(fn_())
        print(fn_())

    needs_ids_(uid_generator)

    # behind the scenes we create an overload something like this:
    needs_ids_(~ctx, fn_(ctx`): u64_): null_
        print(fn_(ctx))
        print(fn_(ctx))
*/

#include <stdint.h>
#include <stdio.h>

typedef struct inner_ctx_t
{   uint64_t counter;
    uint64_t xorer;
}       inner_ctx_t;

uint64_t inner_(inner_ctx_t *ctx)
{   ++ctx->counter;
    return ctx->counter ^ ctx->xorer;
};

// We don't actually need to return a function pointer here;
// if we have an `inner_ctx_t` we know we need to pair with `inner_`.
//    inner_ctx_t uid_generator_ctx = outer_(456);
//    printf("%ld\n", inner_(&uid_generator_ctx));
//    printf("%ld\n", inner_(&uid_generator_ctx));
inner_ctx_t outer_(uint64_t uint64)
{   return (inner_ctx_t)
    {   .counter = uint64,
        .xorer = 12345,
    };
}

// standard overload, not actually needed/used.
// we probably should avoid adding overloads that aren't used
// to match expectations from generics.
void needs_ids_(uint64_t (*fn_)(void))
{   printf("%ld\n", fn_());
    printf("%ld\n", fn_());
}

// built-in special overload, needed for our context-requiring function.
typedef uint64_t (*u64_fn_ctx_)(void *ctx);
void needs_ids_ctx_(u64_fn_ctx_ fn_, void *ctx)
{   printf("%ld\n", fn_(ctx));
    printf("%ld\n", fn_(ctx));
}

int main()
{   inner_ctx_t uid_generator_ctx = outer_(456);
    printf("%ld\n", inner_(&uid_generator_ctx));
    printf("%ld\n", inner_(&uid_generator_ctx));

    needs_ids_ctx_((u64_fn_ctx_)inner_, (void *)&uid_generator_ctx);
    return 0;
}
