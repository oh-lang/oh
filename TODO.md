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

## reference implementation

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

## function signatures

we're going to need to create overloads ourselves with unique names.
because capital letters aren't allowed to start a function (and because
internal underscores cannot be repeated), we'll use them to namespace in
the generated code so there aren't any collisions.
we'll also alphabetize input (and output) arguments.

```
# in oh-lang, in file `my_file.oh`:
my_function(Y: dbl, X; int): str

# in C:
OH_str MY_FILE_OH__my_function__X__rw_int__Y__ro_dbl__return__Str(OH_int *X, const double *Y);
```

we also need to supply a few different function signatures for when
references are more complicated than just a pointer (e.g., via the `refer` class).

### lambda functions

we'll need to support lambda functions by pulling them into the root scope
with an additional `Context` argument that gets automatically supplied
if there are any captures.

### errors

we'll need to standardize on how we'll return errors.  one idea, functions
that never error out return void, and functions that can error out return
the error type.  however, we'd need to require that the error type can be something
that isn't an error (e.g., if `er: int`, then `Er: 0` should be not an error).
this also would require special handling for `hm` types when we need `one_of`s
for other things anyway.  probably can do the standard thing and return the
struct.

### generics

i'm a big fan of zig's comptime type system, which defers type checking 'til
after specialization (a sort of duck-typing).  we can support this by generating
code for generics only after someone tries to use it in real code.

```
multiply(A: ~t, B: t): t
    A * B

multiply(A: -5, B: 20)      # should return -100
multiply(A: "a", B: "B")    # should fail at compilation

# in C, specialized after we infer `int`
OH_int FILE_OH__multiply__A__ro_int__B__ro_int__return__Int(const OH_int *A, const OH_int *B)
{   return OH__multiply__Int__ro__Int__ro__return__Int(A, B);
}
```

as a first pass, we can just specialize the generic code *in the file it's used*.
this isn't great but you could do casts between equivalent generic structs (e.g., across file boundaries).
ideally we'd keep track of it somewhere, e.g., in `.my_file.generated.c`/`.h` files,
where `my_file.oh` is where the generic *itself* is defined, so that we can re-use them
across the codebase.  we'd need to support this for library functions as well.
maybe library files need to be copied over anyway into their own directory (at least 3rd party code).
it would require recompiling your library whenever you add a child (see inheritance).
but this is ok or even expected because oh-lang does things at comptime.

alternatively, as an even rougher first pass, we could put everything into one big `.c` file,
with some DAG logic for where things should be defined.

## classes

### type layout

structs are laid out like the oh-lang class bodies (in `[]`).

```
# in oh-lang, define a class in file `linear.oh`:
vector3: [X: dbl, Y: dbl, Z: dbl]
{   ::length(): dbl
        sqrt(M X^2 + M Y^2 + M Z^2)
}

# in C:
typedef struct
{   double X;
    double Y;
}       LINEAR_OH__vector3;

double LINEAR_OH__length__vector3__return__double(const LINEAR_OH__vector3 *M)
{   return sqrt(M->X * M->X + M->Y * M->Y + M->Z * M->Z);
}
```

### inheritance

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

if we have a dynamic class (e.g., not `@only`), we need to keep track of what child class it is.
this will require putting a type word after/before it in memory, to provide the vtable.
e.g., usually applies only to pointers in C++, but in oh-lang we allow child classes up to a certain size locally.
note we shouldn't need a full type; we could use a smart/small type since we know it descends from the parent.

when creating a class, we give it a positive `Type_id` (probably `u_arch`) only if it
*has any fields* (e.g., instance variables or instance functions defined in `[]`).
otherwise we'll set the `Type_id` to be 0; it's abstract and shouldn't perform any
of the following logic.  when we notice anyone is inheriting from a non-zero `Type_id`,
i.e., via an `child_type_: all_of_[parent_type_, m_: child_fields_]`, we add to a list
keyed on `parent_type_ type_id` (and same for any ancestors of `parent_type`).
this can be a "short tag" when we know the type is at least `parent_type_` already.
the short tag should be 8 bits only, but we probably can make this configurable (e.g., for 32 bit arch).
if we don't mark a non-final instance variable as `@only`, it will take up at least 64 bits of space.
this is because we'll use 56 bits to store a pointer to a child in the worst case, with 8 bits as the
short tag afterwards.  if we are storing a variable as `any` type, we'll use the full `u_arch`
size to store `type_id` then the variable data.

TODO: does this short pointer work logic work for multiple parents?  they'd need to have the same
offset/value, no?

TODO: we need to disallow `all_of` if `parent1` and `parent2` have any of the same instance fields.

when classes *implicitly* inherit from a parent, e.g., for duck typing, we don't add them to
the parent's vtable at first...

```
# `hashable_`'s vtable includes `explicitly_hashable_`:
explicitly_hashable_: all_of_[hashable_, m_: [str;]]
{   ::hash_(~builder): builder hash_(m str)
}

x; hashable_, ..., x = explicitly_hashable_("hi")
print_(hash_(x)) # OK

# `hashable_`'s vtable doesn't include `implicitly_hashable_`:
implicitly_hashable_: [str;]
{   ::hash_(~builder): builder hash_(m str)
}

# you can still use `hash_` here:
y; implicitly_hashable_("hi"J)
print_(hash_(y)) # OK
```

however, if we see that there's the possibility of using the implicit child as the parent class,
we'll add it to the parent's vtable.

```
do_something_[of_: hashable_](~of): null_
    print_(hash_(of))

# this triggers adding `implicitly_hashable_` to the `hashable_` vtable.
do_something_(implicitly_hashable_("asdf"))
# so would this:
x; hashable_, ..., x = implicitly_hashable_("hi")
```

### generics

generic templates with generic methods is disallowed by C++, but we should
be able to make it happen because we specialize at comptime, not before.

```
generic[of]: [Value; of]
{   ::method(~U): u
        U_Value: u = (U * Value) ?? panic()
        U + U_value
}
specific[of: number]: all_of[generic[of], m: [Scale; of]]
{   ;;renew(M Scale. of = 1, Generic Value. of): {}

    ::method(~U): u
        Parent_result: Generic::method(U)
        Scale * Parent_result
}

`Generic` is actually wrapping a `specific[i8]` here:
Generic[i8]; specific(Value: 10_i8, Scale: 2_i8)
print(Generic method(0.5)) # should print "11" via `2 * (0.5 + dbl(0.5 * 10))`
```

would transpile to this:
TODO: how much do we want to wrap with getters/property-setters?
probably need reference functions (not getters/setters) to comply
with all situations.

```
// defined because of `specific[i8]` needing this as a parent.
struct oh__generic_of_i8
{   struct
    {   oh__i8 Value;
    }       M;
};
// defined because of `Specific method(0.5)` usage needing `Generic::method(0.5)`:
dbl oh__method__Generic_of_i8__Dbl__return__Dbl
(   const oh__generic_of_i8 *Generic,
    double Dbl
)
{   dbl U_value = Dbl * Generic->M.Value;
    return Dbl + U_value;
}

// defined because of `specific[i8]`
struct oh__specific_of_i8
{   struct
    {   oh__i8 Value;
    }       Generic;
    struct
    {   oh__i8 Scale;
    }       M;
};

// defined because of `Specific method(0.5)` usage:
dbl oh__method__Specific_of_i8__Dbl__return__Dbl
(   const oh__specific_of_i8 *Specific,
    double Dbl
)
{   dbl Parent_result = oh_method__Generic_of_i8__Dbl__return__Dbl
    (   // NOTE: in general we need this offset if `specific[i8]` is `all_of[m: [Scale; i8], generic[i8]]`
        (const oh_generic_of_i8 *)&(Specific->Generic),
        Dbl
    );
    return Specific->M.Scale * Parent_result;
}

typedef void *oh__unknown;

struct oh_dynamic__generic_of_i8
{   usize Type_id;
    union
    {   oh__generic_of_i8 Generic_of_i8;
        oh__specific_of_i8 Specific_of_i8;
        oh__unknown Unknown;
    };
};

// for dynamic dispatch
// TODO: i think we need to pass around `Type_id` somehow for internal calls,
// and possibly the full `dynamic` type, because we don't know how to get offsets for everyone.
dbl oh__method__DYNAMIC_Generic_of_i8__Dbl__return__Dbl
(   const oh_dynamic__generic_of_i8 *Dynamic__generic_of_i8,
    double Dbl
)
{   switch (Dynamic__generic_i8->Type_id)
    {   case Oh_type_id__generic_of_i8:
            return oh__method__Generic_of_i8__Dbl__return__Dbl(&Some__generic_i8->Generic_of_i8, Dbl);
        case Oh_type_id__specific_of_i8:
            return oh__method__Specific_of_i8__Dbl__return__Dbl(&Some__generic_i8->Specific_of_i8, Dbl);
    }
    // for classes which are JIT:
    dbl (*method__Dbl__return__Dbl)(const oh__unknown *M, double Dbl)
        =   oh_look_up__method__Dbl__return__Dbl(Dynamic__generic_of_i8->Type_id);
    if (method__Dbl__return__Dbl)
    {   return method__Dbl__return__Dbl(&Dynamic__generic_of_i8->Unknown, Dbl);
    }
    exit(1);
}
```

TODO: we want to include a "lambda" type.  if it fits into the size of `oh_dynamic__generic_of_i8` union,
we can inline all the lambda methods as pointers there.  if not, we use 
a pointer to an allocated lambda type.

## big ints

some options:
* https://github.com/wbhart/bsdnt
* https://github.com/adam-mcdaniel/bigint

## tagged unions

`one_of_` needs some special help.  i'd almost like to optimize when we *know* what
a type is, versus when we don't (and need the dynamic dispatch `type_id`).  if we
have the `type_id` present, we don't need a secondary tag for what the `one_of` is;
if we have `one_of_[int_, dbl_]`, we only need 2 `type_id`s, e.g., `a`: the instance
is `int_` but it's part of the `one_of_[int_, dbl_]` type, and `b` it's a `dbl` but 
same qualification.  for completeness, we'll probably have a third `type_id` for
"it's a `one_of_[int_, dbl_]`, check the local tag", which we may also reuse in a
type system.  local tags will go at the end of a `union`ed struct and can be less
than a word in size.  e.g., `one_of[int_, dbl_]` can have a `u1_` tag, something
with 4 types can have a `u2_` tag, etc.

i'd also like code to be self-aware of whether we're using `type_id` or local tags.
e.g., if you want to squeeze out more space in your struct when `type_id` is present
and reduce it for local tags so your entire type fits within a certain number of bytes.

let's maybe converge on a name for them, e.g., "partial types" and "full types".

## jit

this seems fairly tricky.  we could ship part of `tcc` (not `arm-gen.c`) inside of
the compiled binary, but i'm not sure how we'd get it to interface with existing code.
e.g., we want this to be possible:

```
# vector.oh:
vector3[of]: [X; of, Y; of, Z; of]
vector3i: vector3[i64] # this specialization gets compiled in `.vector.generated.c`

# some_other_file.oh
compile_vector3d(): null
    Lines;
    [   "[vector3]: \/vector"
        "vector3d: vector3[dbl]"    # new specialization
        "print(vector3d(X: 0.5, Y: 0.3, Z: 0.1))"
    ]
    Executable: compile(Lines;) ?? panic("should have been ok")
    Executable run()

compile_vector3i(): null
    Lines;
    [   "[vector3i]: \/vector"
        "print(vector3i(X: 3, Y: 4, Z: 5))"
    ]
    Executable: compile(Lines;) ?? panic("should have been ok")
    Executable run()
```

this would require some advanced linking logic.  would this require compiling
everything but `main` into a library, then linking `main` with that library?
then any new `compile` calls can rely on the library.

## captures

we probably shouldn't hide captures as an internal detail; we should
be able to build them as well.  maybe something like `fn_(args, ctx. [...]): type_`,
where `ctx` is a special keyword which is set once at the beginning like:
`fn_(args): type_ = fn_ with ctx. [...]`.
TODO: we can probably introduce `with` and use it to bind arguments
more generally.  e.g., `fn_ with a. 123` or `fn_ with (b: 4, c: 3)`.

see [explicit capture test](https://github.com/oh-lang/oh/blob/main/test/explicit_capture.c)
for how we might translate into C code for captures explicitly, or implicitly via
the [implicit capture test](https://github.com/oh-lang/oh/blob/main/test/implicit_capture.c).

## reference offsets

the main thing that we'd like to avoid is requiring a new method for each
logical reference thing, plus any recursion.  e.g., array, lot, etc.
we want to do is be able to abstract over "here's a function, get a reference
at this offset", which we can recurse with if necessary.
i think we can store it like this, where each pointer from the root feeds
into the next function to get the next pointer.
`[root_ptr: ptr_[null_], array_[fn_(ptr[null_]): ptr_[null_]]`
where each `fn_` is a lambda capture which also includes a hidden "context"
with the correct `offset` for an `array` or an `at` key for a `lot`.
the function itself knows to consider the context as an array context
or a lot context.

the following approach is probably worth trying only after doing the simple approach
of using a pointer to the array and an offset (as a u64):
we could use something like tagged pointers for "small" offsets, like indexing into
an array with a small index.  when we make a reference to the third element, we hold
`[ptr_[array_[~]], offset: 2]` *in one pointer if we can*.  the pointer tag
will hold something like 0 = no offset, 1 = 4 bits in offset, 2 = 8 bits in offset,
etc. up to 7 = 28 bits in offset.
see https://en.wikipedia.org/wiki/X86-64#Virtual_address_space_details for why we
should be able to do stuff like this.

TODO: for the default `lot` (i.e., `insertion_ordered_lot`), can we give the
user a (u64) offset rather than pass the key (possibly string, large) around?
probably not, as referencing the lot at any point can create a new entry,
which can get deleted by something else, etc.  potentially we could resolve
as an offset for a certain block that we are sure that we aren't borrowing
any of the parents/ancestors in a mutable way.

## high precision numbers

could use `x << e` where `x` and `e` are `int` types, with `e` negative to represent
small numbers.  however, if we start adding large and small numbers together, `x`
will get really large (`e` will be the min exponent).  can we do something like
where we represent numbers like this? `x << a + y << b`, and try to manage the
size of all `int`s involved?

## translation process

we need to keep track of which names are needed where.

```
outer_: [a; i64_, inner;]
inner_: [b; dbl_, super_inner;]
super_inner_: [c; str_]

# needs to get ordered as
typedef struct super_inner_t { str_t c; } super_inner_t;
typedef struct inner_t { dbl_t b; super_inner_t super_inner; } inner_t;
typedef struct outer_t { i64_t a; inner_t inner; } outer_t;
```

note that if a reference or pointer to a struct is used, we don't
add it to the list of dependencies.

```
type_name_: str_
type_dependency_: type_name_
type_data_:
[   name;
    dependencies; stack_[dependency_]
]
{   name_: type_name_
    dependency_: type_dependency_
}

type_datas[type_data_, at_: type_name_];
# populate the dependencies, when looking at declarations inside the type.
...
    type_data ... each dependency
        type_datas[type_data name] dependencies append_(dependency)


type_names; set_[vertex_]
type_datas each type_data:
    never_visited;;[type_data name]
dag_sort_(vertex_set. type_names, 

# returns DAG in dependency order.
dag_sort_
(   vertex_set. set_[~vertex_]
    edges_(vertex): (stack_[vertex_]:)
): hm_[stack_[vertex_], dag_ er_]
    (never_visited;) = @hide vertex_set
    # for detecting cycles, list of elements we're considering, or descendents of one.
    currently_visiting; set_[vertex_]

    sorted; stack_[vertex_]
    while never_visited pop_() is visiting. vertex_
        dag_visit_
        (   .visiting
            edges_
            ;currently_visiting
            ;never_visited
            ;sorted
        )
    sorted

@private
dag_visit_
(   visiting. ~vertex_
    edges_(vertex): (stack_[vertex_]:)
    currently_visiting; set_[vertex_]
    never_visited; set_[vertex_]
    sorted; stack_[vertex_]
): hm_[null_]
    debug assert_(never_visited::[visiting])
    if currently_visiting::[visiting]
        return er_ cyclic_dependency_(.visiting)
    currently_visiting;;[visiting clone_()]

    edges_(vertices,  
    type_datas::[visiting] dependencies each type_dependency:
        # use depth first search (DFS) on non-visited vertices
        if never_visited::[type_dependency name]
            dag_visit_
            (   visiting. type_dependency name clone_()
                :vertices
                ;currently_visiting
                ;never_visited
                ;sorted
            )
    to_visit_stack append_(visiting clone_())

    currently_visiting pop_(:visiting)
    sorted append_(.visiting)
```

```
# this has a cycle that's not obvious.
# NEEDS TO BE A COMPILE ERROR:
type1_: [a; int_, type2;]
type2_: [b; dbl_, type3;]
type3_: [c; str_, type1;]
```


