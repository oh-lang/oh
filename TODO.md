# for a byte-code interpreter

stack of call-frames for functions, but `Block`s can jump back more than one.
this is almost continuation passing, but i think we can get away with
jumping back a certain number of call frames, e.g., with `Block exit(3)`.

probably only want one `Block` per function argument.

when determining what overload to call for a function, we probably
want to still have stack-based arguments, but alphabetized.  so we
probably need to normalize:  `fn(B, C: some_function(), A: other_function())`
needs to become something like `fn(A: other_function(), B, C: some_function())`.
HOWEVER we need to keep the call order correct (e.g., `some_function()` should
be called before `other_function()`).  either we'll need stack twizzling
or references, e.g., stack: `..., B, C: some_function(), A: other_function(),`
followed by `Ref_to_A: ref(-1), Ref_to_B: ref(-4), Ref_to_C: ref(-4)`.

do stacks make sense for strongly-typed languages?  would likely have some
indirection, e.g., a pointer stack, a type stack, and a value stack.  values
can take up more space on the value stack based on size of the type.

# reference implementation

can we use [Futamura projection](https://en.wikipedia.org/wiki/Partial_evaluation)
to implement the compiler and interpreter in one go?
the idea is that you create an interpreter `I` for a language `L`, then feed it
the program `P` you want to compile.  but instead of compiling the interpreter `I`
and then running the program `P`, i.e., `c(I)(P)`, you compile with the interpreter
and code combined, i.e., `c(I * P)`; thus making the compiler aware of what
transformations you're trying to do in `P` via `I` to native code.  however, to
specify `I * P`, you need to write `I` a special way.  if you feed in `P` as
the compiler itself (written in language `L`), then you get your compiler at
"native" (and not interpreted) speeds.

this doesn't feel all that much different from transpiling to e.g., C++.

unfortunately performance isn't that great:
https://news.ycombinator.com/item?id=17946026

but there are some interesting tools out there:
https://github.com/BuildIt-lang/buildit

Deegen, e.g., https://github.com/luajit-remake/luajit-remake, seems nice
but also relies on LLVM.  But could be a model for writing a JIT.

# ideas from vlang

v-lang is super nice from a compile-time perspective.  for debug builds,
they use [tcc](https://repo.or.cz/w/tinycc.git) since it's fast.  for
release builds they use gcc or whatever's available.  under the hood,
AFAIU they transpile to a `.c` file which is then consumed by tcc or gcc.
transpiling in oh-lang might be the easiest thing to do right now to get the best
performance.  also transpiling to `.c` would allow us to use cosmocc
(i.e., [cosmopolitan](https://github.com/jart/cosmopolitan)) for cross-platform
builds.  that wouldn't strictly work for GUI applications, however.

this approach wouldn't allow for any JIT'ing, however, at least not without
LICENSE infection, since we wouldn't be able to ship a compiler in the code.
however, security should be considered before we go the full JIT route anyway.
we could avoid shipping a compiler and compile by firing off a `tcc tmp_file.c`
request from inside the code, and then loading and running the file separately...
*actually* looks like tinycc is mostly relicensed to MIT:
https://news.ycombinator.com/item?id=41763624
so we could ship the parts of it that we care about (e.g., not arm-gen.c).

# if we transpile to c

we're going to need forward declarations for all functions and structs.

we'll need to resolve overloads at compile-time if possible, run-time if necessary.

for libraries/projects with lots of files, we'll want to figure out how to
speed up recompilation by only compiling new changes.  potentially we could
have `.my_file_name.generated.c`/`.h` and `.my_file_name.generated.o` if need be,
in the same directory, or in a `.generated` directory (without the `.generated` infix
on the files).  i'm a fan of the local `.my_file.generated.c` approach so that
generated code can be inspected easily.

## generics

i'm a big fan of zig's comptime type system, which defers type checking 'til
after specialization (a sort of duck-typing).  we can support this by generating
code for generics only after someone tries to use it in real code.

```
multiply(A: ~t, B: t): t
    A * B

multiply(A: -5, B: 20)      # should return -100
multiply(A: "a", B: "B")    # should fail at compilation
```

as a first pass, we can just specialize the generic code *in the file it's used*.
this isn't great but you could do casts between equivalent templates (e.g., across file boundaries).
ideally we'd keep track of it somewhere, e.g., in `.my_file.generated.c`/`.h` files,
where `my_file.oh` is where the generic *itself* is defined, so that we can re-use them
across the codebase.  we'd need to support this for library functions as well.
maybe library files need to be copied over anyway into their own directory (at least 3rd party code).

alternatively, as an even rougher first pass, we could put everything into one big `.c` file,
with some DAG logic for where things should be defined.

## inheritance

to resolve a method on a class, we first look if there's a local definition for it;
we look at class functions (static functions) first, then instance methods next.
if there is none, it will go through the parent classes in order and look if they define it.

```
my_class: all_of[m: [My_data;], parent1, parent2]
{   ::my_method(): null
}

My_class; my_class
My_class my_method()    # calls my_class::my_method
My_class other_method() # looks for parent1::other_method(), then parent2::other_method()
```

## function signatures

we're going to need to create overloads ourselves with unique names.
because capital letters aren't allowed to start a function (and because
internal underscores cannot be repeated), we'll use them to namespace in
the generated code so there aren't any collisions.
we'll also alphabetize input (and output) arguments.

```
# in oh-lang, in file `my_file.oh`:
my_function(Y: dbl, X: int): str

# in C:
OH_str MY_FILE_OH__my_function__X__Y__return__Str(const OH_int *X, const double *Y);
```

an alternative is to have pointers for each return type.  e.g., 

```
void MY_FILE_OH__my_function__X__Y__return__Str(const OH_int *X, const double *Y, OH_str *Str);
```

this would be convenient if we were targeting C++, since we could just throw
all the output arguments into the function and get C++ to get the right overload
for us.

### errors

we'll need to standardize on how we'll return errors.  one idea, functions
that never error out return void, and functions that can error out return
the error type.  however, we'd need to require that the error type can be something
that isn't an error (e.g., if `er: int`, then `Er: 0` should be not an error).
this also would require special handling for `hm` types when we need `one_of`s
for other things anyway.  probably can do the standard thing and return the
struct.

