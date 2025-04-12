# why

oh-lang tries to achieve **consistency** above all else.  
**Convenience**, **clarity**, **concision**, **coolness**, and **simplicity** come next.

## consistency

In most languages, primitive types have different casing than class types that are
created by end-user developers.  This is usually an inconsistency by convention,
e.g., `PascalCase` for class names and `snake_case` for primitive types in C++,
but Rust requires this inconsistency by fiat.
In oh-lang, all types are `lower_snake_case`, like functions.  Variables and identifiers
like `_true`, `_false`, or `_my_var_x` are `_initial_underscore_lower_snake_case`.
oh-lang doesn't recommend distinguishing
between constant identifiers and non-constant identifiers with casing,
e.g., like `UPPER_SNAKE_CASE` in other languages; you can rely on the compiler
to stop you if you try to change a constant variable, and you'll save wear
and tear on your caps-lock key.
TODO: we might want to switch to `_type_case` and `variable_case` because `variable_case`
is probably more common.  if we want to dredge up OLD hm-lang logic, we could use
`__type_case`, `_func_case`, and `var_case`.  that would allow us to do things like this
`my_class: [Count; __count] {__count: count_arch, ::_count(): __count}`, and import
all functions via `[_my_func]: \/import/from/here`.
TODO: update the remainder of the file to use the new cases, although we should decide on
the previous TODO first.

Why snake case: long names are hard to parse with Pascal case, and we want to support descriptive
names.  Code is read more than it is written, so we don't need to optimize underscores out
(e.g., like in `PascalCase`) at the cost of making the code harder to read.
We'll automatically convert `CamelCase` or `dromedaryCase` to their respective
oh-lang types (e.g., `_camel_case` and `dromedary_case`, respectively).  The reason
why we use `_initial_underscore_lower_snake_case` for identifiers is because it's too
easy to refactor a type (or function) like `the_array` into something like `special_array`
and miss the corresponding update for variables like `The_array` into `Special_array`.
It also makes internationalization not dependent on unicode parsing; we can immediately
determine whether something is a variable based on the initial `_`.
For the remainder of this document, we'll use `_variable_case`,
`type_case`, and `function_case`, although the latter two are indistinguishable without context.
In context, functions and types are followed by optional generics (in `[]` brackets),
while functions alone have parentheses `()` with optional arguments inside.

Another change is that oh-lang uses `:` (or `;`) for declarations and `=` for reassignment,
so that declaring a variable and specifying a variable will work the same inside and outside
function arguments.  For example, declaring a function that takes an integer named `X`,
`my_function(X: int): null`, and declaring an integer variable named `X` uses the same syntax:
`X: int`.  Similarly, calling a function with arguments specified as `my_function(X: 5)` and
defining a variable works the same outside of a function: `X: 5`.  There is a slight difference
because we can declare variables with `.` in function arguments, which indicates a temporary.  E.g.,
in `(X: int, Y; str, Z. dbl)`, we declare `X` as a readonly reference, `Y` a writable reference,
and `Z` a temporary, whereas outside of function arguments, `[X: int, Y; str]` indicates
that `X` is readonly (though it can be written in constructors or first assignment) and `Y`
is writable.  TODO: we could make `Z. int` (outside of function arguments) indicate a volatile.

In some languages, e.g., JavaScript, objects are passed by reference and primitives
are passed by value when calling a function with these arguments.  In oh-lang,
arguments are passed by reference by default, for consistency.  I.e., on the left
hand side of an expression like `X = 5`, we know that we're referring to `X` as a reference,
and we extend that to function calls like `do_something(X)`.  Note that it's possible
to pass by value as well; see [passing by reference or by value](#pass-by-reference-or-pass-by-value). 
See [passing by reference gotchas](#passing-by-reference-gotchas) for the edge cases.
For a default (i.e., not using a declaration operator `;`, `:`, or `.`), if the variable
is readonly, we'll pass as a readonly reference, and if the variable is writable
(i.e., defined with `;` or `.`), we'll pass as a writable reference.  If there is no
overload defined with a writable reference, the compiler will retry with a readonly reference.

In oh-lang, determining the number of elements in a container uses the same
method name for all container types; `count(Container)` or `Container count()`,
which works for `Array`, `Lot` (map/dict), `Set`, etc.  In some languages, e.g., JavaScript,
arrays use a property (`Array.length`) and maps use a different property (`Map.size`).

## convenience

oh-lang also prioritizes convenience; class methods can be called like a function with
the instance as an argument or as a method on the instance, e.g., `the_method(My_class)`
or `Class the_method()`.  This extends to functions with definitions like
`my_two_instance_function(My_class_a, My_class_b, Width: int, Height: int)` which
can be called as `(My_class_a, My_class_b) my_two_instance_function(Width: 5, Height: 10)`,
or by calling it as a method on either one of the instances, e.g.,
`My_class_a my_two_instance_function(My_class_b, Width: 5, Height: 10)`, without needing to
define, e.g., `My_class_b my_two_instance_function(My_class_a, Width: 5, Height: 10)` as well.

For convenience, `Array[3] = 5` will work even if `Array` is not already at least size 4;
oh-lang will resize the array if necessary, populating it with default values,
until finally setting the fourth element to 5.  This is also to be consistent with
other container types, e.g., lots (oh-lang's version of a map/dictionary), since `Lot["At"] = 50`
works in a similar fashion, e.g., resizing the `lot` as necessary to add an element.
In some standard libraries (e.g., C++), `Array[3] = 5` is undefined behavior
if the array is not already at least size 4.

Similarly, when referencing `Array[10]` or `Lot["At"]`, a default will be provided
if necessary, so that e.g. `++Array[10]` and `++Lot["At"]` don't need to be guarded
as `Array[10] = if count(Array) > 10 {Array[10] + 1} else {Array count(11), 1}` or
`Lot["At"] = if Lot["At"] != Null {Lot["At"] + 1} else {1}`.

## clarity

Functions are called with named arguments always, although names can be omitted in
certain circumstances [when calling a function](#calling-a-function).

Nullable variables should always be annotated by `?`, both in a declaration
`Y?: some_possibly_null_returning_function()` and also when used as a function
argument, e.g., `call_with_nullable(Some_value?: Y)`.  This is to avoid surprises
with null, since `call_with_nullable(Some_value?: Null)` is equivalent to
`call_with_nullable()`, which can be a different overload.

## concision

When defining a function, variable arguments that use the default name for a type can
elide the type name; e.g., `my_function(Int): str` will declare a function that takes 
an instance of `int`.  See [default-named arguments](#default-name-arguments-in-functions).
This is also true if namespaces are used, e.g., `my_function(@My_namespace Int): str`.
If a declaration operator (like `;`, `:`, or `.`) is not used, we'll default to creating
a readonly reference `:` overload but guess at a few weak overloads for `;` and `.` that
make sense.  These weak overloads can be overridden with explicit `;` and `.` overloads.
TODO: if we want to go `fn(X int)` route, could we also mostly define functions as
readonly via `fn(X int) int` (i.e., for `fn(X int): int`)?  i like how `:` guides the eye, tho.
If not using a default name, you can get the same weak overloads using e.g., `fn(X int)`,
i.e., not using an explicit `:;.` operator between an argument and the type.
We can use `:` to explicitly only create the readonly reference overload, e.g.,
`my_function(Int:): str` to create a function which takes a readonly integer reference, or
you can use `my_function(Int;): str` for a function which can mutate the passed-in integer
reference or `my_function(Int.): str` for a function which takes a temporary integer.
This also works for generic classes like `my_generic[of]` where `of` is a template type;
`my_function(My_generic[int];)` is short for `my_function(My_generic; my_generic[int])`.

When calling a function, we don't need to use `my_function(X: X)` if we have a local
variable named `X` that shadows the function's argument named `X`.  We can just
call `my_function(:X)` to specify the readonly reference overload (`X: X`),
`other_function(;Y)` to specify the writable reference overload (`Y; Y`), or
`tmp_function(.Z)` to specify a temporary overload `Z. @hide(Z)!`, which also
stops you from using `Z` again in the rest of the block.  Using `tmp_function(Z!)`
calls the temporary overload and would allow `Z` to still be used afterwards,
but `Z` will be [reset to the default](#prefix-and-postfix-exclamation-points).
If no declaration operator is used when calling a function, e.g., `my_function(X)`,
then we'll infer `:` if `X` is readonly (i.e., defined with `:`)
and `;` if `X` is writable (i.e., defined with `.` or `;`).
If no writable overload is present, the compiler will retry for a readonly overload.
Note the declaration operator (`.;:`) goes on the left
when calling a function with an existing variable for an argument, and on the right
when defining a function and declaring an argument.  I.e., `X;` expands to `X; x`, while
`;X` expands to `X; X`.

Class methods technically take an argument for `M` everywhere, which is somewhat
equivalent to `this` in C++ or JavaScript or `self` in python, but instead of
writing `the_method(M:, X: int): str`, we can write `::the_method(X: int): str`.
This parallels `my_class::the_method` in C++, but in oh-lang we can analogously use
`;;a_mutating_method` for a method that can mutate `M`, i.e.,
`a_mutating_method(M;, X: int): str` becomes `;;a_mutating_method(X: int): str`,
or `..one_temporary_method()` for a method on a temporary `M`, i.e.,
`one_temporary_method(M.)`.  You can use `M` in an instance method to refer
to the class instance.

Inside of class methods, any variables or methods that are defined inline in the class
are available without using `M` as a prefix, for concision.  However, we do not support
shadowing any global variables or overloads inside a class, so import renaming may be
required.

```
vector3: [X: dbl, Y: dbl, Z: dbl]
{   ::length(): dbl
        sqrt(X * X + Y * Y + Z * Z) # no need for `M X`, etc.
}
```

The primary reason for this is so that we can support defining nested types easily
and use them concisely.  For example, with a generic type, it would be convenient
to refer to another generic subtype if we already have the class.

```
my_generic[at, of]: [m: [Lot;]]
{   lot: @only insertion_ordered_lot[at, of]
    ...
}

# ERROR: `lot` is shadowed inside of `my_generic`, use import renaming to avoid this.
#       e.g., `[core_lot: lot]: \\core/lot`
[lot]: \\core/lot
```

In the example above, outside the class we can use `some_type: my_generic[at, of] lot`
to refer to the nested type, but can we also use `some_type: lot[m: my_generic[at, of]]`.
We don't override `lot[my_generic[at, of]]` because a single type might be an override of `lot[of]`;
not necessarily for `lot` but for `array` it's definitely the case.
See [type manipulation](#type-manipulation) for more details.

Note this is actually ok, because we can distinguish overloads based on arguments.

```
vector2: [X: dbl, Y: dbl]
{   ::atan(): dbl
        atan(X, Y)      # also ok: `\\math atan(X, Y)`
}
[atan(X: dbl, Y: dbl): dbl]: \\math
```

We're just not allowed to import any overloads that would be shadow a method or function
on the class body, where ambiguity could arise with or without the class instance being
supplied.  E.g., if `do_something(X: int)` is defined outside the class, then inside a
class body `::do_something(X: int)` as an instance method (or even `do_something(X: int)`
as a static class function) would throw a compile error.
TODO: we're getting a bit into the weeds, this should be somewhere besides the intro.

Also in the spirit of conciseness, `O` can be used for an *O*ther instance of the same type,
and `g` can be used for the current generic class (without the specification) while
`m` always refers to the current class type (including the specification if generic).

```
vector3[of: number]: [X; of, Y; of, Z; of]
{   # `g` is used for this generic class without the current specification,
    # in this case, `vector3`.
    g(@First Value: ~value, @Second Value, @Third Value): g[value]
        [X: @First Value, Y: @Second Value, Z: @Third Value]

    ::dot(O): of
        # `M X`, etc. is optional, since `[X, Y, Z]` are in scope for this instance method.
        X * O X + Y * O Y + Z * O Z
}

dot(vector3(1, 2, 3), vector3(-6, 5, 4)) == -6 + 10 + 12
```

Class getters/setters *do not use* `::get_x(): dbl` or `;;set_x(Dbl.): null`, but rather
just `::x(): dbl` and `;;x(Dbl.): null` for a private variable `X; dbl`.  This is one
of the benefits of using `function_case` for functions/methods and `Variable_case`
for variables; we can easily distinguish intent without additional verbs.
Of course, overloads are also required here to make this possible.

Because we use `::` for readonly methods and `;;` for writable methods, we can
easily define "const template" methods via `:;` which work in either case `:` or `;`.
This is mostly useful when you can call a few other methods internally that have specific
`::` and `;;` overloads, since there's usually some distinct logic for readonly vs. writable.
E.g., `;:the_method(X;: str): M check(;:X)` where `check` has distinct overloads for `::` and `;;`.
See [const templates](#const-templates) for more details.

oh-lang uses result-passing instead of exception-throwing in order to make it clear
when errors can occur.  The `hm[ok, er]` class handles this, with `ok` being the
type of a valid result, and `er` being the type of an error result.  You can specify
the types via `hm[ok: int, er: str]` for `ok` being `int` and `er` being a `str`.
If the `ok` and `er` types are distinct, you don't need to wrap a return value in
`ok(Valid_result)` and `er(Error_result)`; you can just return `Valid_result` or `Error_result`.
See [the `hm` section](#hm) for more details.  It is a compile error not to handle
errors when they are returned (e.g., something like a `no-unused-result`).

## coolness

**Coolness** is a fairly subjective measure, but we do use it to break ties.
While there are a lot of good formatting options out there, 
[Horstmann brace style](https://en.wikipedia.org/wiki/Indentation_style#Horstmann) is
hands-down the raddest indentation style.  Similarly, `lower_snake_case`
and `Initial_upper_snake_case` make for more readable long names, but they also
look cooler than their `dromedaryCase` and `PascalCase` counterparts.
(`_initial_underscore_snake_case` would be even cooler to replace `Initial_upper_snake_case`
but the ergonomics of `_true` and `_false` aren't great.)

## simplicity

We don't require a different function name for each method to convert a result class
into a new one, e.g., to transform the `ok` result or the `er` error.  In oh-lang, we
allow overloading, so converting a result from one type to another, or extracting a
default value for an error, all use an overloaded `map` method, so there's no mental
overhead here.  Since overloads are not possible in Rust, there is an abundance of methods, e.g.,
[`Result::map_*` documentation](https://doc.rust-lang.org/std/result/enum.Result.html#method.map),
which can be challenging to remember.  

We also don't use a different concept for interfaces and inheritance.
The equivalent of an interface in oh-lang is simply an abstract class.  This way
we don't need two different keywords to `extend` or `implement` a class or interface.
In fact, we don't use keywords at all; to just add methods (or class functions/variables),
we use this syntax, `child_class: parent_class { ::extra_methods(): int, ... }`,
and to add instance variables to the child class we use this notation:
`child_class: all_of[parentClass, m: [Child_x: int, Child_y: str]] { ... methods }`.

oh-lang handles generics/templates in a way more similar to zig or python rather than C++ or Rust.
When compiled on their own, templates are only tested for syntax/grammar correctness.
When templates are *used* in another piece of code, that's when the specification kicks in
and all expressions within the generic are compiled to see if they are allowed with the
specified types.  Any errors are still compile-time errors, but you get to have the simplicity
of duck typing without needing to specify your type constraints fully.

```
my_generic[of](A: ~of, B: of): of
    # this clearly requires `of` to implement `*`
    # but we didn't need to specify `[of: number]` or similar in the generic template.
    A * B

print(my_generic(A: 3, B: 4))   # OK
print(my_generic(A: [1, 2, 3], B: [4, 5]))  # COMPILE ERROR: no definition for `array[int] * array[int]`
```

Similarly, duck typing means that if you define an appropriate `::hash` function on your class,
you don't need to mention that your class is `hashable`.  A check for `some_class is some_other_class`
will not require strict descent from `some_other_class` but only that the same methods and fields
are defined.

## safety

oh-lang supports "safe" versions of functions where it's possible that we'd run out of
memory or otherwise throw.  By default, `Array[100] = 123` will increase the size
of the array if necessary, and this could potentially throw in a memory-constrained
environment (or if the index was large).  If you need to check for these situations,
there is a safe API, e.g., `Hm: (Array[100] = 123)` and the result `Hm` can then
be checked for `is_er()`, etc.  In order to avoid another syntax for safe assignment,
we use operator overloading (via return name).  Most of the time you don't want to
hide errors from other developers, however, but if you do, you should make the
program panic/terminate rather than continue.  Example code:

```
custom_container[of]: [Vector[10, of];]
{   # make an overload for `M[Ordinal]` where `Ordinal` is a 1-based indexing scheme.
    :;[Ordinal]: hm[ok: (Of:;), er: str]
        if Ordinal > 10
            er("index too high")
        else
            ok((Of:; Vector[Ordinal]))

    @can_panic
    :;[Ordinal]: (Of:;)
        M[Ordinal] Hm assert()

    # for short, you can use this `@hm_or_panic` macro, which will essentially
    # inline the logic into both methods but panic on errors.
    :;[Ordinal]: @hm_or_panic[ok: (Of:;), er: str]
        if Ordinal > 10
            er("index too high")
        else
            ok((Of:; Vector[Ordinal]))
}
```

Almost all operations similarly have a result-like syntax, because they can fail.
E.g., `A * B` can overflow for fixed-width integers (or run out of memory for `int`).
Similarly for `A + B` and `A - B`.  (`A // B` is safe for fixed-width, but could
potentially OOM for `int`.)  If overflow/underflow is desired, use the overload
which returns a variable named `Wrap`, e.g., `X: (A + B) Wrap` or `Wrap: A + B`.
Otherwise `A + B` will panic on overflow and terminate the program.  The alternative
is to handle the error explicitly: `Hm: A + B` then something like this:
`what Hm {Ok: {print(Ok)}, Er: {print("got error: ${Er})}}`.

TODO: do we want primitive types to NOT panic on overflow, but types like `count`
to panic?

# general syntax

* `print(...)` to echo some values (in ...) to stdout, `error(...)` to echo to stderr
    * use string interpolation for printing dynamic values: `print("hello, ${Variable_1}")`
    * use `print(No_newline: "keep going ")` to print without a newline
    * default overload is to print to null, but you can request the string that was printed
        if you use the `print(Str.): str` or `error(Str.): str` overloads.
        e.g., `another_fn(Value: int): print("Value is ${Value}")` will return `Null`,
        whereas `another_fn(Value: int): str {print("Value is ${Value}")}` will
        return "Value is 12" (and print that) if you call `another_fn(Value: 12)`.
* `type_case`/`function_case` identifiers like `x` are function/type-like, see [identifiers](#identifiers)
* `Variable_case` identifiers like `X` are instance-like, see [identifiers](#identifiers)
* use `#` for [comments](#comments)
* outside of arguments, use `:` for readonly declarations and `;` for writable declarations
* for an argument, `:` is a readonly reference, `;` is a writable reference, and `.` is a temporary
    (i.e., passed by value), see [pass-by-reference or pass-by-value](#pass-by-reference-or-pass-by-value)
* use `:` to declare readonly things, `;` to declare writable things.
    * use `A: x` to declare `A` as an instance of type `x`, see [variables](#variables),
        with `A` any `Initial_upper_snake_case` identifier.
    * use `fn(): x` to declare `fn` as a function returning an instance of type `x`, see [functions](#functions),
        with any arguments inside `()`.  `fn` can be renamed to anything `lower_snake_case`, but `fn` is the default.
    * use `a: y` to declare `a` as a constructor that builds instances of type `y`
        with `a` any `lower_snake_case` identifier.
    * use `new[]: y` to declare `new` as a function returning a type `y`, with any arguments inside `[]`.
        `new` can be renamed to anything `lower_snake_case`, but `new` is the default.
    * while declaring *and defining* something, you can avoid the type if you want the compiler to infer it,
        e.g., `A: some_expression()`
* when not declaring things, `:` is not used; e.g., `if` statements do not require a trailing `:` like python
* commas `,` are equivalent to a line break at the current tab and vice versa
    * `do_something(), do_something_else()` executes both functions sequentially 
    * see [line continuations](#line-continuations) for how commas can be elided across newlines for e.g., array elements
* `()` for reference objects, organization, and function calls/declarations
    * `(W: str = "hello", X: dbl, Y; dbl, Z. dbl)` to declare a reference object type, `W` is an optional field
        passed by readonly reference, `X` is a readonly reference, `Y` is a writable reference,
        and `Z` is passed by value.  See [reference objects](#reference-objects) for more details.
    * `My_str: "hi", (X: str) = My_str` to create a [reference](#references) to `My_str` in the variable `X`.
    * `(Some_instance x(), Some_instance Y;, W: "hi", Z. 1.23)` to instantiate a reference object instance
        with `X` and `W` as readonly references, `Y` as mutable reference, and `Z` as a temporary.
    * `"My String Interpolation is $(X, Y: Z)"` to add `(X: *value-of-X*, Y: *value-of-Z*)` to the string.
    * `f(A: 3, B: "hi")` to call a function, and `f(A: int, B: str): null` to declare a function.
    * `A@ (x(), Y)` to call `A x()` then `A Y` with [sequence building](#sequence-building)
        and return them in a reference object with fields `X` and `Y`, i.e., `(X: A x(), Y: A Y)`.
        This allows `X` and `Y` to be references.  This can be useful e.g., when `A` is an expression
        that you don't want to add a local variable for, e.g., `my_long_computation()@ (x(), Y)`.
* `[]` are for types, containers (including objects, arrays, and lots), and generics
    * `[X: dbl, Y: dbl]` to declare a plain-old-data class with two double-precision fields, `X` and `Y`
    * `[X: 1.2, Y: 3.4]` to instantiate a plain-old-data class with two double-precision fields, `X` and `Y`
    * `"My String interpolation is $[X, Y]"` to add `[*value-of-X*, *value-of-Y*]` to the string.
    * `some_class[A: int, of]: some_other_class[Count: A, at: int, of]` to define a class type
        `some_class` being related to `some_other_class`.
    * For generic/template classes, e.g., classes like `array[Count, of]` for a fixed array of size
        `Count` with elements of type `of`, or `lot[int, at: str]` to create a map/dictionary
        of strings mapped to integers.  See [generic/template classes](#generictemplate-classes).
    * For generic/template functions with type constraints, e.g., `my_function[of: non_null](X: of, Y: int): of`
        where `of` is the generic type.  See [generic/template functions](#generictemplate-functions) for more.
    * `[Greeting: str, Times: int] = destructure_me()` to do destructuring of a return value
        see [destructuring](#destructuring).
    * `A@ [x(), Y]` to call `A x()` then `A Y` with [sequence building](#sequence-building)
        and return them in an object with fields `X` and `Y`, i.e., `[X: A x(), Y: A Y]`.
        You can also consider them as ordered, e.g.,
        `Results: A@ [x(), Y], print("${Results[0]}, ${Results[1]})`.
* `{}` for blocks and sequence building
    * `{...}` to effectively indent `...`, e.g., `if Condition {do_thing()} else {do_other_thing(), 5}`
        * Used for defining a multi-statement function inline, e.g., `fn(): {do_this(), do_that()}`.
            (Note that you can avoid `{}` if the block is one statement, like `fn(): do_this()`.)
        * Note that braces `{}` are optional if you actually go to the next line and indent,
            but they are recommended for long blocks.
    * `A@ {x(), Y}` with [sequence building](#sequence-building), 
        calling `A x()` and `A Y`, returning `A` if it's a temporary otherwise `A Y`
    * `"My String Interpolation is ${missing(), X}"` to add `X` to the string.
        Note that only the last element in the `${}` is added, but `missing()` will still be evaluated.
* `~` to infer or generalize a type
    * `my_generic_function(Value: ~u): u` to declare a function that takes a generic type `u`
        and returns it.  For more details, see [generic/template functions](#generictemplate-functions).
    * `My_result; array[~] = do_stuff()` is essentially equivalent to `My_result; do_stuff() Array`, i.e.,
        asking for the first array return-type overload.  This infers an inner type but doesn't name it.
    * `Named_inner; array[~infer_this] = do_stuff()` asks for the first array return-type overload,
        but defines the inner type so it can be used later in the same block, e.g.,
        `First_value; infer_this = Named_inner[0]`.
        Any `lower_snake_case` identifier can be used for `infer_this`.
* `$` for inline block and lambda arguments
    * [inline blocks](#block-parentheses-and-commas) include:
        * `$[...]` as shorthand for a new block defining `[...]`, e.g., for a return value:
            `Array: if Some_condition $[1, 2, 3] else $[4, 5]`
        * `$(...)` as shorthand for a new block defining `(...)`, e.g., a reference object:
            `Result: if X > Y $(Max: X, Min: Y) else $(Min: X, Max: Y)`
        * `${...}` is almost always equivalent to `{...}`, except inside of string interpolation,
            so we'll likely alias `${...}` to `{...}` outside of strings.
    * `$Arg` as shorthand for defining an argument in a [lambda function](#lambda-functions)
        * `My_array map({$Int * 2 + 1})` will iterate over e.g., `My_array: [1, 2, 3, 4]`
            as `[3, 5, 7, 9]`.  The `$` variables attach to the nearest brace/indent as
            function arguments.
* all arguments are specified by name so order doesn't matter, although you can have default-named arguments
  for the given type which will grab an argument with that type (e.g., `Int` for an `int` type).
    * `(X: dbl, Int)` can be called with `(1234, X: 5.67)` or even `(Y, X: 5.67)` if `Y` is an `int`
* variables that are already named after the correct argument can be used without `:`
    * `(X: dbl, Y: int)` can be called with `(X, Y)` if `X` and `Y` are already defined in the scope,
        i.e., eliding duplicate entries like `(X: X, Y: Y)`.
* [Horstmann indentation](https://en.wikipedia.org/wiki/Indentation_style#Horstmann) to guide
    the eye when navigating multiline braces/brackets/parentheses
* operators that diverge from some common languages:
    * `**` and `^` for exponentiation
    * `&|` at the start of each text slice to create a multiline string.
    * `<>` for bit flips on integer types (instead of `~`)
    * `><` for bitwise xor on integer types (instead of `^`)
* see [operator overloading](#operator-overloading) for how to overload operators.

## defining variables

See [variables](#variables) for a deeper dive.

```
# declaring a variable:
Readonly_var: int
Mutable_var; int

# declaring + defining a variable:
Mutable_var; 321

# you can also give it an explicit type:
Readonly_var: int(123)

# you can also define a variable using an indented block;
# the last line will be used to initialize the variable.
# here we use an implicit type (whatever `Some_helper_value + 4` is).
My_var:
    # this helper variable will be descoped after calculating `My_var`.
    Some_helper_value: some_computation(3)
    Some_helper_value + 4

# you can also give it an explicit type:
Other_var; explicit_type
    "asdf" + "jkl;"
```

## defining strings

```
# declaring a string:
Name: "Barnabus"

# using interpolation in a string:
Greeting: "hello, ${Name}!"

# declaring a multiline string
Important_items:
        &|Fridge
        &|Pancakes and syrup
        &|Cheese
# this is the same as `Important_items: "Fridge\nPancakes and syrup\nCheese\n"`

# a single-line multiline string still includes a newline at the end.
Just_one_line: &|This is a 'line' "you know"
# this is equivalent to `Just_one_line: "This is a 'line' \"you know\"\n"

# declaring a multiline string with interpolation
Multiline_interpolation:
        &|Special delivery for ${Name}:
        &|You will receive ${Important_items} and more.
# becomes "Special delivery for Barnabus\nYou will receive Fridge\nPancakes and syrup\nCheese\n and more."

# interpolation over multiple file lines.
# WARNING: this does not comply with Horstmann indenting,
# and it's hard to know what the indent should be on the second line.
Evil_long_line: "this is going to be a long discussion ${
        Name}, can you confirm your availability?"
# INSTEAD, use string concatenation:
Good_long_line: "this is going to be a long discussion"
    &   "${Name}, can you confirm your availability?"

# you can also nest interpolation logic, although this isn't recommended:
Nested_interpolation: "hello, ${if Condition {Name} else {'World${"!" * 5}'}}!"
```

Notice that the `&` operator works on strings to add a space (if necessary)
between the two operands.  E.g., `'123' & '456'` becomes `'123 456'`.  It also
strips any trailing whitespace on the left operand and any leading whitespace
on the right operand to ensure things like `'123\n \n' & '\n456'` are still just `'123 456'`.
This makes it the perfect operator for string concatenation across lines where we want
to ensure a space between words on one line and the next.
TODO: it could also be used as a postfix or prefix operator, e.g., `&'   hi'` is 'hi'
and `'hey\n  '&` is 'hey'.  not sure this is better than `'   hi' strip()` though.

## defining arrays

See [arrays](#arrays) for more information.

```
# declaring a readonly array
My_array: array[element_type]

# defining a writable array:
Array_var; array[int](1, 2, 3, 4)
# We can also infer types implicitly via one of the following:
#   * `Array_var; array([1, 2, 3, 4])`
#   * `Array_var; [1, 2, 3, 4]`
Array_var[5] = 5    # Array_var == [1, 2, 3, 4, 0, 5]
++Array_var[6]      # Array_var == [1, 2, 3, 4, 0, 5, 1]
Array_var[0] += 100 # Array_var == [101, 2, 3, 4, 0, 5, 1]
Array_var[1]!       # returns 2, zeroes out Array_var[1]:
                    # Array_var == [101, 0, 3, 4, 0, 5, 1]

# declaring a long array (note the Horstmann indent):
Long_implicitly_typed:
[   4   # commas aren't needed here.
    5
    6
]

# declaring a long array with an explicit type:
Long_explicitly_typed: array[i32]
(   5   # commas aren't needed here.
    6
    7
)
```

Note there are some special rules that allow line continuations for parentheses
as shown above.  See [line continuations](#line-continuations) for more details.

## defining lots

See [lots](#lots) for more information.

```
# declaring a readonly lot
My_lot: lot[at: id_type, value_type]

# defining a writable lot:
Votes_lot; lot[at: str, int]("Cake": 5, "Donuts": 10, "Cupcakes": 3)
# We can also infer types implicitly via one of the following:
#   * `Votes_lot; lot(["Cake": 5, ...])`
#   * `Votes_lot; ["Cake": 5, ...]`
Votes_lot["Cake"]           # 5
++Votes_lot["Donuts"]       # 11
++Votes_lot["Ice Cream"]    # inserts "Ice Cream" with default value, then increments
Votes_lot["Cupcakes"]!      # deletes from the Lot (but returns `3`)
Votes_lot::["Cupcakes"]     # Null
# now Votes_lot == ["Cake": 5, "Donuts": 11, "Ice Cream": 1]
```

## defining sets

See [sets](#sets) for more details.

```
# declaring a readonly set
My_set: set[element_type]

# defining a writable set:
Some_set; set[str]("friends", "family", "fatigue")
# We can also infer types implicitly via the following:
#   * `Some_set; set("friends", ...)`
Some_set::["friends"]   # `True`
Some_set::["enemies"]   # Null (falsey)
Some_set["fatigue"]!    # removes "fatigue", returns `True` since it was present.
                        # Some_set == set("friends", "family")
Some_set["spools"]      # adds "spools", returns Null (wasn't in the set)
                        # Some_set == set("friends", "family", "spools")
```

## defining functions

See [functions](#functions) for a deeper dive.

```
# declaring a "void" function:
do_something(With: int, X; int, Y; int): null

# defining a void function
# braces {} are optional (as long as you go to the next line and indent)
# but recommended for long functions.
do_something(With: int, X; int, Y; int): null
    # because `X` and `Y` are defined with `;`, they are writable
    # in this scope and their changes will persist back into the
    # caller's scope.
    X = With + 4
    Y = With - 4

# calling a function with temporaries:
do_something(With: 5, X; 12, Y; 340)

# calling a function with variables matching the argument names:
With: 1000
X; 1
Y; 2
# Note that readonly arguments (`:`) are the default,
# so you need to specify `;` for writable arguments.
do_something(With, X;, Y;)

# calling a function with argument renaming:
Mean: 1000
Mutated_x; 1
Mutated_y; 2
do_something(With: Mean, X; Mutated_x, Y; Mutated_y)
```

```
# declaring a function that returns other values:
do_something(X: int, Y: int): [W: int, Z: int]

# defining a function that returns other values.
# braces are optional as long as you go to the next line and indent.
do_something(X: int, Y: int): [W: int, Z: int]
{   # NOTE! return fields `W` and `Z` are in scope and can be assigned
    # directly in option A:
    Z = \\math atan(X, Y)
    W = 123
    # option B: can just return `W` and `Z` in an object:
    [Z: \\math atan(X, Y), W: 123]
}
```

We don't require `{}` in function definitions because we can distinguish between
(A) creating a function from the return value of another function, (B) passing a function
as an argument, and (C) defining a function inline in the following ways.  (A) uses
`my_fn(Args): return_type = fn_returning_a_fn()` in order to get the correct type on `my_fn`,
(B) uses `outer_fn(rename_to_this(Args): return_type = use_this_fn)` and requires a single
`function_case` identifier on the RHS, while (C) uses `defining_fn(Args): do_this(Args)`
and uses inference to get the return type (for the default `do_this(Args)` function).

```
# case (A): defining a function that returns a lambda function
make_counter(Counter; int): do(): int
    do(): ++Counter
# TODO: equivalent? `make_counter(Counter; int): do(): ++Counter`
Counter; 123
counter(): int = make_counter(Counter;)
print(counter())    # 124
# `Counter` is also 124 now.
```

Note that because we support [function overloading](#function-overloads), we need
to specify the *whole* function [when passing it in as an argument](#functions-as-arguments).

```
# case (B): defining a function with some lambda functions as arguments
do_something(you(): str, greet(Name: str): str): str
    greet(Name: you())

# calling a function with some functions as arguments:
my_name(): "World"
do_something
(   you(): str = my_name
    greet(Name: str): str
        "Hello, ${Name}"
)

# case (C): defining a few functions inline without `{}`
hello_world(): print(do_something(you(): "world", greet(Name: str): "Hello, ${Name}"))
```

### defining generic functions

There are two ways to define a generic function: (1) via type inference `~x`
and (2) with an explicit generic specification `[types...]` after the function name.
You can combine the two methods if you want to infer a type and specify a
condition that the type should satisfy, e.g., `fn[x: number](X: ~x): x`
to require that `x` is a number type.  Any types that are not inferred
but are explicitly given in brackets must be added at the callsite, e.g.,
`fn[x: number, y](~X, After: y): y` should be called as `fn[y: int](123.4, After: 5)`.
At the callsite, the specified generics in brackets `[]` must not be abstract.

Note that default names apply to either case; `~X` is shorthand for `X: ~x`
which would not need an argument name, and `fn[value](Value): null` would
require `value` specified in the brackets but not in the argument list,
e.g., `fn[value: int](123)`.  In brackets, the "default name" for a type is
`of`, so you can call a function like `fn[of](Of): null` as `fn[int](123)`.

Some examples:

```
# this argument type is inferred, with a default name
fn(~X): x
# call it like this:
fn(512)

# this argument is inferred but need to name it as `X: ...`
fn(@Named ~X): x
# call it like this:
fn(X: 512)

# another way to infer an argument but require naming it as `X: ...`
fn(X: ~t): t
# we call it like this:
fn(X: 512)

# explicit generic with condition, not inferred:
fn[x: condition](X): x
# call it like this, where `int` should satisfy `condition`
fn[x: int](5)

# explicit generic with condition, inferred
fn[x: condition](X: ~x): x
# call it like this, where `dbl` should satisfy `condition`
fn[x: dbl](3.14)

# explicit generic without a default name:
fn[x](Value: x): null
# call it like this:
fn[x: str](Value: "asdf")
```

See [generic/template functions](#generictemplate-functions) for more details.


## defining classes

See [classes](#classes) for more information on syntax.

```
# declaring a simple class
vector3: [X: dbl, Y: dbl, Z: dbl]

# declaring a "complicated" class.  the braces `{}` are optional
# but recommended due to the length of the class body.
my_class: [X; int]
{   # here's a class function that's a constructor
    m(X. int): m
        ++Count
        [X]

    ;;descope(): null
        --Count

    # here's a class variable (not defined per instance)
    @private
    Count; count_arch = 0

    # here's a class function (not defined per instance)
    # which can be called via `my_class count()` outside this class
    # or `count()` inside it.
    count(): count_arch
        Count
    # for short, `count(): Count`

    # methods which keep the class readonly use a `::` prefix
    ::do_something(Y: int): int
        X * Y

    # methods which mutate the class use a `;;` prefix
    ;;update(Y: int): null
        # because there's an implicit `M;` here, it'll look for
        # ;;do_something(Y) first, but resolve to `::do_something(Y)`:
        X = do_something(Y)
}
```

Inside a class body, we don't need to use `M` to scope instance variables/functions
or `m` to scope class variables/functions, because we always produce a
compile error if we notice any variables/functions that would shadow
global variables/functions.  Import renaming is recommended to solve
this issue.
TODO: is everything ok for keywords like `each` and `is` which can also be methods?

Inheritance of a concrete parent class and implementing an abstract class
work the same way, by specifying the parent class/interface in an `all_of`
expression alongside any child instance variables, which should be tucked
inside an `m` field.  Despite requiring the `m` field in the `all_of`,
we don't need to specifically look up fields in the child via `M Field_name`;
we can still just use `Field_name` since `m` fields are automatically
brought into scope for any methods.
TODO: make sure that's desired; it kinda makes sense to only enscope
it as `M`.

```
parent1: [P1: str]
{   ::do_p1(): null
        print("doing p1 ${P1}")
}

parent2: [P2: str]
{   ::do_p2(): null
        print("doing p2 ${P2}")
}

child3: all_of[parent1, parent2, m: [C3: int]]
{   # this passes P1 to parent1 and C3 to child3 implicitly,
    # and P2 to parent2 explicitly.
    ;;renew(Parent1 P1. str, P2. str, M C3. int): null
        # same as `parent2 renew(M;, P2)` or `parent2;;renew(P2)`.
        Parent2 renew(P2)

    ::do_p1(): null
        # this logic repeats `Parent1 do_p1())` `M C3` times.
        M C3 each Int_:
            # same as `parent1 do_p1(M)` or `parent1::do_p1()`.
            Parent1 do_p1()
    
    # do_p2 will be used from parent2 since it is not overridden here.
}
```

For those aware of storage layout, order matters when using `all_of`;
the struct will be started with fields in `a` for `all_of[a, b, c]`
and finish with fields in `c`; the child fields do not need to be first
(or last); they can be added as `a`, `b`, or `c`, of course as `m: [...]`.

### defining generic classes

With classes, generic types must be explicitly declared in brackets.
Any conditions on the types can be specified via `[the_type: the_condition, ...]`.

```
# default-named generic
generic[of]: [@private Of]
{   # you can use inference in functions, so you can use `generic(12)`
    # to create an instance of `generic` with `of: int` inferred.
    # You don't need this definition if `[Of]` is public.
    # NOTE: `g` is like `m` for generic classes but without the specification.
    g(~T): g[t]
        [Of. T] 
}

Generic[int](1)             # shorthand for `Generic: generic[int](1)`.
My_generic: generic(1.23)   # infers `generic[dbl]` for this type.

# not default named:
entry[at: hashable, of: number]: [At, Value; of]
{   ::add(Of): null
        Value += Of
}

Entry[at: str, int](At: "cookies", Value: 123)  # shorthand for `Entry: entry[at: str, int](...)`
My_entry: entry(At: 123, Value: 4.56)           # infers `at: int` and `of: dbl`.
My_entry add(1.23)
My_entry Value == 5.79
```

See [generic/template classes](#generictemplate-classes) for more information.

## identifiers

Identifiers in oh-lang are very important.  The capitalization (or lack thereof)
of the first letter indicates whether the identifier is a variable or a function.
Since we think of functions as verb-like, they are `function_case` identifiers, e.g.,
`make_toast` or `run_marathon`.  On the other hand, variables are names, and we think
of them as proper nouns (like names), e.g., `Sam` or `Max_array_count`, so they are
`Variable_case` identifiers.  Class names are `type_case`, since they
act more functions than variables; e.g., you can convert one class instance into
another class's instance, like `int(My_number_string)` which converts `My_number_string`
(presumably a `string` type) into a big integer.

There are a few reserved keywords, like `if`, `elif`, `else`, `with`, `return`,
`what`, `in`, `each`, `for`, `while`, `pass`, `where`,
which are function-like but may consume the rest of the statement.
E.g., `return X + 5` will return the value `(X + 5)` from the enclosing function.
There are some reserved namespaces with side effects like `@First`, `@Second`,
`@Named`, `@As`,
which should be used for their side effects.  For example, `@First` and `@Second`
are reserved for binary operations like `&&` and `*`.  See [namespaces](#namespaces)
for more details.  Other reserved keywords:

There are some reserved variable names, like `M`, which can only
be used as a reference to the current class instance, and `O` which
can only be used as a reference to an *O*ther instance of the same type;
`O` must be explicitly added as an argument, though, in contrast to `M` which can be implicit.
(The corresponding types `m`, and `o` are reserved for the same reasons.)

Most ASCII symbols are not allowed inside identifiers, e.g., `*`, `/`, `&`, etc., but
underscores (`_`) have some special handling.  They are ignored in numbers,
e.g., `1_000_000` is the same as `1000000`, and highly recommended for large numbers.
Underscores in identifiers will automatically "capitalize" the next letter, so
`my_function` is the same as `myFunction`, and `_count` is the same as `Count`.
Numbers are ignored, so `x_1` is the same as `x1`.  To indicate a variable (or function)
is unused in a block, use a trailing underscore.  If used when defining a function
argument, it will not affect how callers call the function; they'll use the
non-trailing-underscored name.

```
# when defining, we use a trailing underscore to indicate the variable is unused.
my_function(Argument_which_we_will_need_later_: int): null
    print("TODO")

# when calling:
my_function(Argument_which_we_will_need_later: 3)
```

## blocks

### tabs vs. spaces

Blocks of code are made out of lines at the same indent level; an indent is four spaces.
No more than 7 levels of indent are allowed, e.g., lines at 0, 4, 8, 12, 16, 20, 24 spaces.
If you need more indents, refactor your code.

### line continuations

Lines can be continued at a +2 indent from the originating line, and all
subsequent lines can stay there at that indent (without indenting more).
Note that operators *are ignored* for determining the indent, so typical
practice is to tab to the infix operator then tab to the number/symbol
you need for continuing a line. 

There are some special rules with parentheses;
if an opening brace/bracket/paren starts a line, and its insides are indented,
we try to pair it with the previous line, so it's not necessary to indent +2.
Also note, if there is no operator between two lines at the same indent,
we'll assume we should add a comma.

```
# the following are equivalent to `My_array: [5, 6, 7]`.

# this is the block-definition style for a variable
My_array:
    [5, 6, 7]

# this is similar to the block definition.
My_array:   # OK, but...
    [   5
        6
        7
    ]

# note it's unnecessary because we also allow opening brackets
# to get attached to the previous line if the internals are indented.
My_array:   # better!
[   5
    6
    7
]

# if you want to one-line it on a second line it's also possible with a +2 indent.
My_array:
        [5, 6, 7]

# the parentheses trick only works if the inside is indented.
Not_defined_correctly:
[5, 6, 7]       # not attached to the previous line.
```

Because of this, some care must be taken when returning a bracket
from a function, since we may try to pair it with the previous line.
If you *don't* want to pair a block with the previous line, use `pass` or `return`.  

```
# example of returning `[X, Y]` values from a function.
# there's no issue here because we're not indenting in `[X, Y]`:
my_function(Int): [X: int, Y: int]
    do_something(Int)
    [X: 5 - Int, Y: 5 + Int]

# this indents `[X, Y]` (i.e., to split into a multi-line array),
# but note that we need `return` to avoid parsing as `do_something(Int)[X: ...]`.
my_function(Int): [X: int, Y: int]
    do_something(Int)
    return
    [   X: 5 - Int
        Y: 5 + Int
    ]

# alternatively, you could add a comma between the two statements
# to ensure it doesn't parse as `do_something(Int)[X: ...]`:
my_function(Int): [X: int, Y: int]
    do_something(Int),
    [   X: 5 - Int
        Y: 5 + Int
    ]
```

Because parentheses indicate [reference objects](#reference-objects),
which can be returned like brackets, similar care must be taken with
indents in `()`.

```
my_function(Str): int
    Results: if Str == "hello"
        do_something(Str),
        [   X: Str + ", world!"
            Y: Str count()
        ]
    else $[X: "oh no", Y: 3]
    print(Results X)
    return Results Y
```

When it comes to parentheses, you are welcome to use
[one-true-brace style](https://en.wikipedia.org/wiki/Indentation_style#:~:text=One%20True%20Brace),
which will be converted into Horstmann style.

```
Some_variable: some_very_long_function_name_because_it_is_good_to_be_specific(10)
    +   3             # indent at +2 ensures that 3 is added into Some_variable.
    -   Other_variable # don't keep adding more indents, keep at +2 from original.

Array_variable:
[   1   # we insert commas
    2   # between each newline
    3   # as long as the indent is the same.
    Other_array # here we don't insert a comma after `Other_array`
    [   3       # because the indent changes
    ]           # so we parse this as `Other_array[3],`
    5           # and this gets a comma before it.
]

# this is inferred to be a `lot` with a string ID and a `one_of[int, str]` value.
Lot_variable;
[   "Some_value": 100
    "Other_value": "hi"
]
Lot_variable["Some_other_value"] = if Condition {543} else {"hello"}

# This is different than the `Lot_variable` because it is an instance
# of a `[Some_value: int, Other_value: str]` plain-old-data type,
# which cannot have new fields added, even if it was mutable.
Object_variable:
[   Some_value: 100
    Other_value: "hi"
]
```

Note that the close parenthesis must be at the same indent as the line of the open parenthesis.
The starting indent of the line is what matters, so a close parenthesis can be on the same
line as an open parenthesis.

```
Some_value:
(       (20 + 45)
    *   Continuing + The + Line + At_plus_2_indent -
        (       Nested * Parentheses / Are + Ok
            -   Too
        )
)

Another_line_continuation_variable: Can_optionally_start_up_here
    +   Ok_to_not_have_a_previous_line_starting_at_plus_two_indent * 
        (       Keep_going_if_you_like
            -   However_long
        ) + (70 - 30) * 3

# note that the formatter will take care of converting indents like this:
Non_horstmann_indent: (
    20 + some_function(45)
)
# into this:
Non_horstmann_indent:   # FIXME: update name :)
(   20 + some_function(45)
)
```

Note that line continuations must be at least +2 indent, but can be more if desired.
Unless there are parentheses involved, all indents for subsequent line continuations
should be the same.

```
Example_plus_three_indent; some_type
...
Example_plus_three_indent
    =       Hello
        +   World
        -   Continuing
```

Arguments supplied to functions are similar to arrays/lots and only require +1 indent
if they are multiline.

```
if some_function_call
(   X
    Y: 3 + sin(X)   # default given for Y, can be given in terms of other arguments.
    Available_digits:
    [   1
        3
        5
        7
        9
    ]
)
    do_something()

defining_a_function_with_multiline_arguments
(   Times: int
    Greeting: string
    Name: string("World")   # argument with a default
):  string                  # indent here is optional/aesthetic
    # "return" is optional for the last line of the block,
    # unless you're returning a multiline array/object.
    "${Greeting}, ${Name}! " * Times

defining_a_function_with_multiline_return_values
(   Argument0: int
):
[   Value0: int     # you may need to add comments because
    Value1: str     # the formatter may 1-line these otherwise
]
    do_something(Argument0)
    # here we can avoid the `return` since the internal
    # part of this object is not indented.
    [Value0: Argument0 + 3, Value1: str(Argument0)]

# ALTERNATIVE: multiline return statement
defining_a_function_with_multiline_return_values
(   Argument0: int
    Argument1: str
):  [Value0: int, Value1: str]
    do_something(Argument0)
    # this needs to `return` or `pass` since it looks like an indented block
    # otherwise, which would attach to the previous line like
    # `do_something(Argument0)[Value0: ...]`
    return
    [   Value0: Argument0 + 3
        Value1: Argument1 + str(Argument0)
    ]
    # if you are in a situation where you can't return -- e.g., inside
    # an if-block where you want to pass a value back without returning --
    # use `pass`.

defining_another_function_that_returns_a_generic
(   Argument0: str
    Argument1: int
):  some_generic_type
[   type0: int
    type1: str
]
    do_something(Argument0)
    print("got arguments ${Argument0}, ${Argument1}")
    return ...
```

Putting it all together in one big example:

```
Some_line_continuation_example_variable:
        Optional_expression_explicitly_at_plus_two_indent
    +   5 - some_function
        (       Another_optional_expression
            +   Next_variable
            -   Can_keep_going
            /   Indefinitely
                R: 123.4
        )
```

### block parentheses and commas

You can use `{` ... `}` to define a block inline.  The braces block is grammatically
the same as a standard block, i.e., going to a new line and indenting to +1.
This is useful for short `if` statements, e.g., `if Some_condition {do_something()}`.
Similarly, you can return normal objects or reference objects in blocks via
`$[...]` or `$(...)`, respectively.

Similarly, note that commas are essentially equivalent to a new line and tabbing to the
same indent (indent +0).  This allows you to have multiple statements on one line,
in any block, by using commas.  E.g.,

```
# standard version:
if Some_condition
    print("toggling shutoff")
    shutdown()

# comma version:
if Some_condition
    # WARNING: NOT RECOMMENDED, since it's easy to accidentally skip reading
    # the statements that aren't first:
    print("toggling shutoff"), shutdown()

# block parentheses version
if Some_condition { print("toggling shutoff"), shutdown() }
```

If the block parentheses encapsulate content over multiple lines, note that
the additional lines need to be tabbed to +1 indent to match the +1 indent given by `{`.
Multiline block parentheses are useful if you want to clearly delineate where your blocks
begin and end, which helps some editors navigate more quickly to the beginning/end of the block.

```
# multiline block parentheses via an optional `{`
if Some_condition
{   print("toggling shutdown")
    print("waiting one more tick")
    print("almost..."), print("it's a bit weird to use comma statements")
    shutdown()
}
```

## comments

Comments come in three types: (1) end of line (EOL) comments, (2) mid-line comments,
and (3) multiline comments.  End of line comments are the hash `#` followed by any
character that is not `(`.  All characters after an EOL comment are ignored; the
compiler will start parsing on the next line.  A mid-line comment begins with `#(`
followed by any character that is not a `#`, and ends with `)#` *on the same line*.
All characters within the parentheses are ignored by the compiler.  Multiline comments
begin with `#(#` and end with the corresponding `#)#` *at the same tab indent*.
This means you can have nested multiline comments, as long as
the nested symbols are at a new tab stop, and they can even be broken (e.g., an unmatched
closing operator `#)#` as long as it is indented from the symbols which started the
multiline comment), although this is not recommended.  To qualify as a multiline comment
(either to open or close the comment), nothing else is allowed on the line before or after
(besides spaces), otherwise a compiler error is thrown.  All characters on all lines in between
the multiline comment symbols (e.g., `#(#` to `#)#`) are ignored.

Note that the prefix `#@` signifies an end-of-line comment from the compiler, so if you use
them they may be deleted/updated in unexpected ways.

With function documentation comments, it's recommended to declare the asymptotic run time.

# overview of types

Standard types whose instances can take up an arbitrary amount of memory:

* `int`: signed big-integer
* `rtl`: rational number (e.g. an `int` divided by a positive, non-zero `int`)
* `str`: array/sequence of utf8 bytes, but note that `string` is preferred for
    function arguments since it includes other containers which deterministically
    provide utf8 bytes.

Other types which have a fixed amount of memory:

* `null`: should take up no memory, but can require an extra bit for an optional type.
* `flt`: single-precision floating-point number, AKA `f32`
* `dbl`: double-precision floating-point number, AKA `f64`
* `bool`: can hold a True or False value
* `rune`: a utf8 character, presumably held within an `i32`
* `u8`: unsigned byte (can hold values from 0 to 255, inclusive)
* `u16` : unsigned integer which can hold values from 0 to 65535, inclusive
* `u32` : unsigned integer which can hold values from 0 to `2^32 - 1`, inclusive
* `u64` : unsigned integer which can hold values from 0 to `2^64 - 1`, inclusive
* `uXYZ` : unsigned integer which can hold values from 0 to `2^XYZ - 1`, inclusive,
    where `XYZ` is 128 to 512 in steps of 64, and generically we can use
    `unsigned[Bits: count]: what Bits {8 {u8}, 16 {u16}, 32 {u32}, ..., Count: {disallowed}}`
* `count` : `s64` under the hood, intended to be >= 0 to indicate the amount of something.
* `index` : signed integer, `s64` under the hood.  for indexing arrays starting at 0.
* `ordinal` : signed integer, `s64` under the hood.  for indexing arrays starting at 1.

and similarly for `i8` to `i512`, using two's complement.  For example,
`i8` runs from -128 to 127, inclusive, and `u8(i8(-1))` equals `255`.
The corresponding generic is `signed[Bits: count]`.  We also define the
symmetric integers `s8` to `s512` using two's complement, but disallowing
the lowest negative value of the corresponding `i8` to `i512`, e.g.,
-128 for `s8`.  This allows you to fit in a null type with no extra storage,
e.g., `one_of[s8, null]` is exactly 8 bits, since it uses -128 for null.
(See [nullable classes](#nullable-classes) for more information.)
Symmetric integers are useful when you want to ensure that `-Symmetric`
is actually the opposite sign of `Symmetric`; `-i8(-128)` is still `i8(-128)`.
The corresponding generic for symmetric integers is `symmetric[Bits: count]`.

Note that the `ordinal` type behaves exactly like a number but can be used
to index arrays starting at 1.  E.g., `Array[ordinal(1)]` corresponds to `Array[index(0)]`
(which is equivalent to other numeric but non-index types, e.g., `Array[0]`).
There is an automatic delta by +-1 when converting from `index` to `ordinal`
or vice versa, e.g., `ordinal(index(1)) == 2` and `index(ordinal(1)) == 0`.
Note however, that there's a bit of asymmetry here; non-index, numeric types
like `u64`, `count`, or `i32` will convert to `index` or `ordinal` without any delta.
It's only when converting between `index` and `ordinal` that a delta occurs.

## casting between types

oh-lang attempts to make casting between types convenient and simple.
However, there are times where it's better to be explicit about the program's intention.
For example, if you are converting between two number types, but the number is *not*
representable in the other type, the run-time will return an error.  Therefore you'll
need to be explicit about rounding when casting floating point numbers to integers,
unless you are sure that the floating point number is an integer.  Even if the float
is an integer, the maximum floating point integer is larger than most fixed-width integer
types (e.g., `u32` or `i64`), so errors can be returned in that case.  The big-integer type
`int` will not have this latter issue, but may return errors depending on memory constraints.
Notice we use `assert` to shortcircuit function evaluation and return an error result
(like throwing).  See [errors and asserts](#errors-and-asserts) for more details.

```
# Going from a floating point number to an integer should be done carefully...
X: dbl(5.43)
Safe_cast: X int()                  # Safe_cast is a result type (`hm[ok: int, Number_conversion er]`)
# also OK: `Safe_cast: int(X)`.
Q: X int() assert()                 # returns an error since `X` is not representable as an integer
Y: X round(Down) int() assert()     # Y = 5.  equivalent to `X floor()`
Z: X round(Up) int() assert()       # Z = 6.  equivalent to `X ceil()`.
R: X round() int() assert()         # R = 5.  rounds to closest integer, breaking ties at half
                                    #         to the integer larger in magnitude.

# Note, representable issues arise for conversions even between different integer types.
A: u32(1234)
Q: A u8() assert()                  # RUN-TIME ERROR, `A` is not representable as a `u8`.
B: u8(A & 255) assert()             # OK, communicates intent and puts `A` into the correct range.
```

Casting to a complex type, e.g., `one_of[int, str](Some_value)` will pass through `Some_value`
if it is an `int` or a `str`, otherwise try `int(Some_value)` if that is allowed, and finally
`str(Some_value)` if that is allowed.  If none of the above are allowed, the compiler will
throw an error.  Note that nullable types absorb errors in this way (and become null), so
`one_of[int, null](Some_safe_cast)` will be null if the cast was invalid, or an `int` if the
cast was successful.

To define a conversion from one class to another, you can define a global function
or a class method, like this:

```
scaled8:
[   # the actual value held by a `scaled8` is `Scaled_value / Scale`.
    @private
    Scaled_value: u8
]
{   # static/class-level variable:
    @private
    Scale: 32_u8

    m(Flt): hm[ok: m, er: one_of[Negative, Too_big]]
        Scaled_value: round(Flt * Scale)
        if Scaled_value < 0
            return Negative
        if Scaled_value > u8 max() flt()
            return Too_big
        scaled8(Scaled_value u8() ?? panic())

    # if there are no representability issues, you can create
    # a direct method to convert to `flt`;
    # this can be called like `flt(Scaled8)` or `Scaled8 flt()`.
    ::flt(): flt
        # `u8` types have a `flt` method.
        Scaled_value flt() / Scale flt()

    # if you have representability issues, you can return a result instead.
    ::int(): hm[ok: int, Number_conversion er]
        if Scaled_value % Scale != 0
            Number_conversion Not_an_integer
        else
            Scaled_value // Scale
}

# global function; can also be called like `Scaled8 dbl()`.
dbl(Scaled8): dbl
    # note that we can access private variables of the class *in this file/module*
    # but if we weren't in this file we wouldn't have this access.
    Scaled8 Scaled_value dbl() / scaled8 Scale dbl()

# global function which returns a result, can be called like `Scaled8 u16()`
u[Bits: count](Scaled8): hm[ok: u[Bits], Number_conversion er]
    if Scaled8 Scaled_value % scaled8 Scale != 0
        Number_conversion Not_an_integer
    else
        Scaled8 Scaled_value // scaled8 Scale
```

## types of types

Every variable has a reflexive type which describes the object/primitive that is held
in the variable, which can be accessed via the `type_case` version of the
`Variable_case` variable name.  

```
# implementation note: `int` should come first so it gets tried first;
# `dbl` will eat up many values that are integers, including `4`.
X; one_of[int, dbl] = 4
Y; x = 4.56     # use the type of `X` to define a variable `Y`.
```

Note that the `type_case` version of the `Variable_case` name does not have
any information about the instance, so `x` is `one_of[int, dbl]` in the above
example and `Y` is an instance of the same `one_of[int, dbl]` type.  For 
ways to handle different types differently within a `one_of`, see 
[this](#one_of-types).

Some more examples:

```
vector3: [X; dbl, Y; dbl, Z; dbl]

My_vector3: vector3(X: 1.2, Y: -1.4, Z: 1.6)

print(my_vector3)                # prints `vector3`
print(vector3 == my_vector3)     # this prints true
```

Variables that refer to types cannot be mutable, so something
like `some_type; vector3` is not allowed.  This is to make it
easier to reason about types.

TODO: types of functions, shouldn't really have `new`.
TODO: we should discuss the things functions do have, like `my_function Inputs`
and `my_function Outputs`.
TODO: if we define `my_fn(~X): x` and then use `Y: x(3)` internally, that's fine,
we're creating an `x` instance with initialization of `3`.
but if we do `Z: X x()` then we have a bit of a shadowing problem; are we referring
to `x(X)`, e.g., `X clone()`, or are we referring to a method `x:;x()`?
presumably we may want to refer to either, so we need ways to disambiguate.
in this case, we probably want `x(X)` to do the clone operation and `X x()` to request
the method `:;x()`.  e.g., if `x == vector3`, then `x(X)` is `vector3(X x(), X y(), X z())`
and `X x()` is component x of the vector.
TODO: don't use `X clone()`, use `x(X)` because we can do copy-constructors
like `m(O): hm[ok: m, er: Out_of_memory]` or whatever.

## type overloads

Similar to defining a function overload, we can define type overloads for generic types.
For example, the generic result class in oh-lang is `hm[ok, er]`, which
encapsulates an ok value (`ok`) or a non-nullable error (`er`).  For your custom class you
may not want to specify `hm[ok: my_ok_type, er: my_class_er]` all the time for your custom
error type `my_class_er`, so you can define `hm[of]: hm[ok: of, er: my_class_er]` and
use e.g. `hm[int]` to return an integer or an error of type `my_class_er`.  Shadowing variables is
invalid in oh-lang, but overloads are valid.  Note however that we disallow redefining
an overload, as that would be the equivalent of shadowing.

## type manipulation

Plain-old-data objects can be thought of as merging all fields
in this way:
```
object == merge[object fields(), {[$Field Name: $Field value]}]
```

TODO: good ways to do keys and values for an object type (e.g., like TypeScript).
see if there's a better way to do it, e.g., `object valued[{um[$value]}]`, so
it's easy to see that all field names are the same, just values that change.
TODO: do we even need `{um[$of]}`?  isn't `um` already functable as an `um[of]`?

Here are some examples of changing the nested fields on an object
or a container, e.g., to convert an array or object to one containing futures.

```
# base case, needs specialization.
nest[m, new[of]: ~n]: disallowed

# container specialization.
# TODO: can we do `nest[$um]` instead of `nest[{um[$of]}]`?
# e.g., `array[int] nest[{um[$of]}] == array[um[int]]`,
# or you can do `nest[m: array[int], {um[$of]}]` for the same effect.
nest[c: container, m: ~c[of: ~nested, ~at], new[of]: ~n]: c[of: new[nested], at]

# object specialization.
# e.g., `[X: int, Y: str] nest[{hm[ok: $of, er: some_er]}]`
# or you can do `nest[{hm[ok: $of, er: some_er]}, m: [X: int, Y: str]]` for the same effect.
# to make `[X: hm[ok: int, er: some_er], Y: hm[ok: str, er: some_er]]`,
nest[m: object, new[of]: ~n]: merge
[   m fields()
    {[$Field Name: new[$Field value]]}
]
```

Here are some examples of unnesting fields on an object/future/result.

```
# base case, needs specialization
unnest[of]: disallowed

# container specialization
# e.g., `unnest[array[int]] == int` and `unnest[set[dbl]] == dbl`.
unnest[container[of: ~nested, ~at]]: nested

# future specialization
# e.g., `unnest[um[str]] == str`.
unnest[um[~nested]]: nested

# result specialization
# e.g., `unnest[hm[str, er: int]] == str`.
unnest[hm[ok: ~nested, ~er]]: nested

# null specialization
# e.g., `unnest[int?] == int`.
unnest[one_of[...~nested, null]]: one_of[...nested]
```

Note that if we have a function that returns a type, we must use brackets, e.g.,
`the_function[...]: the_return_type`, but we can use instances like booleans
or numbers inside of the brackets (e.g., `array[3, int]` for a fixed size array type).
Conversely, if we have a function that returns an instance, we must use parentheses,
e.g., `the_function(...): instance_type`.  In either case, we can use a type as
an argument, e.g., `nullable(of): bool` or `array3[of]: array[3, of]`.
Type functions can be specialized in the manner shown above, but instance functions
cannot be.  TODO: would we want to support that at some point??

Here is some nullable type manipulation:

```
# the `null` type should not be considered nullable because there's
# nothing that can be unnulled, so ensure there's something not-null in a nullable.
#   nullable(one_of[dbl, int, str]) == False
#   nullable(one_of[dbl, int, null]) == True
#   nullable(one_of[int, null]) == True
#   nullable(one_of[null]) == False
#   nullable(null) == False
nullable(of): of contains(not[null], null)

# examples
#   unnull[int] == int
#   unnull[int?] == int
#   unnull[one_of[array[int], set[dbl], null]] == one_of[array[int], set[dbl]]
unnull[of]: if nullable(of) {unnest[of]} else {of}

# a definition without nullable, using template specialization:
unnull[of]: of
unnull[one_of[...~nested, null]]: nested
```

# operators and precedence

TODO: add : , ; ?? postfix/prefix ?
TODO: add ... for dereferencing.  maybe we also allow it for spreading out an object into function arguments,
e.g., `my_function(A: 3, B: 2, ...My_object)` will call `my_function(A: 3, B: 4, C: 5)` if `My_object == [B: 4, C: 5]`.

| Precedence| Operator  | Name                      | Type/Usage        | Associativity |
|:---------:|:---------:|:--------------------------|:-----------------:|:-------------:|
|   1       |   `()`    | parentheses               | grouping: `(A)`   | ??            |
|           |   `[]`    | parentheses               | grouping: `[A]`   |               |
|           |   `{}`    | parentheses               | grouping: `{A}`   |               |
|           | `\\x/y/z` | library module import     | special: `\\a/b`  |               |
|           | `\/x/y/z` | relative module import    | special: `\/a/b`  |               |
|   2       |  ` ()`    | function call             | on fn: `a(B)`     | LTR           |
|           |   `::`    | impure read scope         | binary: `A::B`    | LTR           |
|           |   `;;`    | impure read/write scope   | binary: `A;;B`    |               |
|           |   ` `     | implicit member access    | binary: `A B`     |               |
|           |   ` []`   | subscript                 | binary: `A[B]`    |               |
|           |   `!`     | postfix moot = move+renew | unary:  `A!`      |               |
|           |   `?`     | postfix nullable          | unary:  `A?`/`a?` |               |
|           |   `??`    | nullish OR                | binary: `A??B`    |               |
|   3       |   `^`     | superscript/power         | binary: `A^B`     | RTL           |
|           |   `**`    | also superscript/power    | binary: `A**B`    |               |
|           |   `--`    | unary decrement           | unary:  `--A`     |               |
|           |   `++`    | unary increment           | unary:  `++A`     |               |
|           |   `~`     | template/generic scope    | unary:  `~b`      |               |
|   4       |   `<>`    | bitwise flip              | unary:  `<>A`     | RTL           |
|           |   `-`     | unary minus               | unary:  `-A`      |               |
|           |   `+`     | unary plus                | unary:  `+A`      |               |
|           |   `!`     | prefix boolean not        | unary:  `!A`      |               |
|   5       |   `>>`    | bitwise right shift       | binary: `A>>B`    | LTR           |
|           |   `<<`    | bitwise left shift        | binary: `A<<B`    |               |
|   6       |   `*`     | multiply                  | binary: `A*B`     | LTR           |
|           |   `/`     | divide                    | binary: `A/B`     |               |
|           |   `%`     | modulus                   | binary: `A%B`     |               |
|           |   `//`    | integer divide            | binary: `A//B`    |               |
|           |   `%%`    | remainder after //        | binary: `A%%B`    |               |
|   7       |   `+`     | add                       | binary: `A+B`     | LTR           |
|           |   `-`     | subtract                  | binary: `A-B`     |               |
|   8       |   `&`     | bitwise AND + string cat  | binary: `A&B`     |               |
|           |   `\|`    | bitwise OR                | binary: `A\|B`    |               |
|           |   `><`    | bitwise XOR               | binary: `A><B`    |               |
|   9       |   `==`    | equality                  | binary: `A==B`    | LTR           |
|           |   `!=`    | inequality                | binary: `A!=B`    |               |
|   10      |   `&&`    | logical AND               | binary: `A && B`  | LTR           |
|           |  `\|\|`   | logical OR                | binary: `A \|\| B`|               |
|           |  `!\|`    | logical XOR               | binary: `A !\| B` |               |
|   11      |   `=`     | assignment                | binary: `A = B`   | LTR           |
|           |  `???=`   | compound assignment       | binary: `A += B`  |               |
|           |   `<->`   | swap                      | binary: `A <-> B` |               |
|   12      |   `->`    | ergo                      | binary: `A -> B`  | LTR           |
|   13      |   `,`     | comma                     | binary/postfix    | LTR           |


TODO: discussion on `~`

## function calls

Function calls are assumed whenever a function identifier (i.e., `function_case`)
occurs before a parenthetical expression.  E.g., `print(X)` where `X` is a variable name or other
primitive constant (like `5`), or `any_function_name(Any + Expression / Here)`.
In case a function returns another function, you can also chain like this:
`get_function(X)(Y, Z)` to call the returned function with `(Y, Z)`.

It is recommended to use parentheses where possible, to help people see the flow more easily.
E.g., `some_function(Some_instance Some_field some_method()) Final_field` looks pretty complicated.
This would compile as `(some_function(Some_instance)::Some_field::some_method())::Final_field`,
and including these parentheses would help others follow the flow.  Even better would be to
add descriptive variables as intermediate steps.

We don't allow for implicitly currying functions in oh-lang,
but you can explicitly curry like this:

```
some_function(X: int, Y; dbl, Z. str):
    print("something cool with ${X}, ${Y}, and ${Z}")

curried_function(Z. str): some_function(X: 5, Y; 2.4, Z.)

# or you can make it almost implicit like this:
$curried_function{some_function(X: 5, Y; 2.4, $Z.)}:
```

## macros

TODO: discussion on how to code up macros.

All standard flow control words also get a reserved macro, e.g., `@if`, `@else`, `@while`,
etc., so that users can tell the compiler to check these values at compile time rather than
at runtime.  Obviously inputs to these need to be resolvable at compile time.

List of existing macros.
* `@if`, `@elif`, `@else`
* `@what`
* `@while`, `@each`
* `@return` - probably isn't necessary but reserved anyway.


## namespaces

Namespaces are used to avoid conflicts between two variable names that should be called
the same, i.e., for function convenience.  oh-lang doesn't support shadowing, so something
like this would break:

```
my_function(X: int): int
    # define a nested function:
    # COMPILE ERROR
    do_stuff(X: int): null
        # is this the `X` that's passed in from `my_function`? or from `do_stuff`?
        # most languages will shadow so that `X` is now `do_stuff`'s argument,
        # but oh-lang does not allow shadowing.
        print(X)
    do_stuff(X)
    do_stuff(X: X // 2)
    do_stuff(X: X // 4)
    X // 8
```

There are two ways to get around this; one is [hiding variables](#hiding-variables).
In the above example, the best way is to use a namespace for one or both conflicts.
Namespaces look like `Variable_case` annotations with a field name, e.g.,
`@My_namespace My_variable_name`, where `My_namespace` is some unrestricted
name.  You normally use these in function arguments, but they can
annotate any variable that you're declaring.  Any future references to the
variable just use the namespace `My_namespace`.  It is recommended to namespace
the "outer" variable so you don't accidentally use it in the inner scope.

```
my_function(@Outer X: int): int
    # nested function is OK due to namespace:
    do_stuff(X: int): null
        # inner scope, any usage of `@Outer X` would be clearly intentional.
        print(X)
    do_stuff(@Outer X)
    do_stuff(X: @Outer X // 2)
    do_stuff(X: @Outer X // 4)
    X // 8
```

If it's difficult to namespace the outer variable (e.g., because you don't
want to delta many lines), you can use `@hide` to ensure you don't use the
other value accidentally.

```
my_function(X: int): int
    # nested function is OK due to namespace:
    do_stuff(@Other X: int): null
        # inner scope, usage of `X` might be accidental, so let's hide:
        @hide X
        ...
        print(@Other X) # OK
        print(X)        # COMPILE ERROR, `X` was hidden from this scope.
        ...
    do_stuff(X)
    do_stuff(X: X // 2)
    do_stuff(X: X // 4)
    X // 8
```

Similarly, you can define new variables with namespaces, in case you need a new variable
in the current space.  This might be useful in a class method like this:

```
my_class: [X; dbl]
{   # this is a situation where you might like to use namespaces.
    ;;do_something(@New X. dbl): dbl
        # this is what `;;x(X. dbl): dbl` might be internally.
        # defines a variable `X` in the namespace `@Old`:
        @Old X: X!
        X = @New X
        @Old X
}
```

One of the most convenient uses for namespaces is the ability to use elide argument
names when calling functions.  E.g., if you have a function which takes a variable named `X`,
but you already have a different one in scope, you can create a new variable with a namespace
`@Example_namespace X: My_new_x_value` and then pass it into the function as
`my_function(@Example_namespace X)` instead of `my_function(X: @Example_namespace_x)`.
This also works with default-named variables.

```
some_function(@Input Index): null
    # `@Input Index` is a default-named variable of type `index`, but we refer to it
    # within this scope using `@Input Index`.
    even(Index): bool
        Index % 2 == 0
    # you can define other namespaces inline as well, like `@Another` here:
    range(@Input Index) each @Another Index:
        if even(@Another Index)
            print(@Another Index)
        
X: index = 100
some_function(X)     # note that we don't need to call as `some_function(Index: X)` or `some_function(@Input Index: X)`.
```

You can use the same namespace for multiple variables, e.g., `@Input Rune` and `@Input String`,
as long as the variable names don't overlap.  Like the member access operators below, the
namespace operator binds left to right.

One final note, we don't consider namespacing as renaming, e.g., `@Outer Int: int`, and then referring
to it as `Outer` internally -- we need the full `@Outer Int` to refer to it.  This is because we also
want namespacing to work with functions, e.g., `@Outer fn`.  We also don't want to introduce another
way to rename things, e.g., in destructuring.
TODO: we could use `@Outer fn` and then `outer(...)` to call the function.

### full list of reserved namespaces

* `@First` - for the first operand in a binary operation (where order matters)
* `@Second` - for the second operand in a binary operation (where order matters)
* `@Named` - for arguments that should be explicitly named in [functions](#defining-generic-functions)

TODO: maybe change `@Named` to `@As`.

## member access operators `::`, `;;`, ` `, and subscripts `[]`

We use `::`, `;;`, and ` ` (member access) for accessing variables or functions that belong to
another object.  The `::` operator ensures that the RHS operand is read only, not write,
so that both LHS and RHS variables remain constant.  Oppositely, the `;;` scope operator passes
the RHS operand as writable, and therefore cannot be used if the LHS variable is readonly.
The implicit member access operator ` ` is equivalent to `::` when the LHS is a readonly variable
and `;;` when the LHS is a writable variable.  When declaring class methods, `::` and `;;` can be
unary prefixes to indicate readonly/writable-instance class methods.  They are shorthand for adding a
readonly/writable `M` (self/this) as an argument.

```
example_class: [X: int, Y: dbl]
{   # this `;;` prefix is shorthand for `renew(M;, ...): null`.
    # in a `renew` method, adding `M` to the arguments like `M X` means `X` will
    # be initialized (or re-initialized) with the value that is passed in for `X`.
    ;;renew(M X: int, M Y: dbl): null
        print("X ${X} Y ${Y}")

    # this `::` prefix is shorthand for `multiply(M: m, ...): dbl`:
    ::multiply(Z: dbl): dbl
        X * Y * Z
}
```


```
some_class: [X: dbl, Y: dbl, A; array[str]]
Some_class; some_class(X: 1, Y: 2.3, A: ["hello", "world"])
print(Some_class::A)     # equivalent to `print(Some_class A)`.  prints ["hello", "world"]
print(Some_class::A[1])  # prints "world"
print(Some_class A[1])   # also prints "world", using ` ` (member access)
Some_class;;A[4] = "love"    # the fifth element is love.
Some_class::A[7] = "oops"    # COMPILE ERROR, `::` means the array should be readonly.
Some_class;;A[7] = "no problem"

Nested_class; array[some_class]
Nested_class[1] X = 1.234        # creates a default [0] and [1], sets [1]'s X to 1.234
Nested_class[3] A[4] = "oops"    # creates a default [2] and [3], sets [3]'s A to ["", "", "", "", "oops"]
```

For class methods, `;;` (`::`) selects the overload with a writable (readonly) class
instance, respectively.  For example, the `array` class has overloads for sorting, (1) which
does not change the instance but returns a sorted copy of the array (`::sort(): m`), and
(2) one which sorts in place (`;;sort(): null`).  The ` ` (member access) operator will use
`A:` if the LHS is a readonly variable or `A;` if the LHS is writable.  Some examples in code:

```
# there are better ways to get a median, but just to showcase member access:
get_median_slow(Array[int]): hm[ok: int, er: string]
    if Array count() == 0
        return er("no elements in array, can't get median.")
    # make a copy of the array, but no longer allow access to it (via `@hide`):
    @Sorted Array: @hide Array sort()   # same as `Array::sort()` since `Array` is readonly.
    ok(@Sorted Array[@Sorted Array count() // 2])

# sorts the array and returns the median.
get_median_slow(Array[int];): hm[ok: int, er: string]
    if Array count() == 0
        return er("no elements in array, can't get median.")
    Array sort()    # same as `Array;;sort()` since `Array` is writable.
    ok(Array[Array count() // 2])
```

Note that if the LHS is readonly, you will not be able to use a `;;` method.
To sum up, if the LHS is writable, you can use `;;` or `::`, and ` ` (member access) will
effectively be `;;`.  If the LHS is readonly, you can only use `::` and ` `, which are equivalent.

Subscripts `[]` have the same binding strength as member access operators since they are conceptually
similar operations.  This allows for operations like `++A[3]` meaning `++(A[3])` and
`--A B C[3]` equivalent to `--(((A;;B);;C)[3])`.  Member access binds stronger than exponentation
so that operations like `A B[C]^3` mean `((A::B)[C])^3`.

Note that `something() Nested_field` becomes `(something())::Nested_field` due to
the function call having higher precedence.  (You can also use destructuring if you want
to keep a variable for multiple uses: `[Nested_field]: something()`.)

## prefix and postfix question marks `?`

The `?` operator binds strongly, but less so than member access, so `x a?` is equivalent
to `one_of[x a, null]` and not `x one_of[a, null]`.  This is for nested classes, e.g.,
`x: [...] {a: int}`, so that we don't need to use `(x a)?` to represent `x one_of[a, null]`.
Generally speaking, if you want your entire variable to be nullable,
it should be defined as `X?: int`.

Prefix `?` can be used to short-circuit function evaluation if an argument is null.
for a function like `do_something(X?: int)`, we can use `do_something(?X: My_value_for_x)`
to indicate that we don't want to call `do_something` if `My_value_for_x` is null;
we'll simply return `Null`.  E.g., `do_something(?X: My_value_for_x)` is equivalent
to `if My_value_for_x == Null {Null} else {do_something(X: My_value_for_x)}`.
In this case `X` is already in scope, it becomes `do_something(?X)` to elide the
variable name.

There's also an infix `??` type which is a nullish or.
`X Y ?? Z` will choose `X Y` if it is non-null, otherwise `Z`.

## prefix and postfix exclamation points

The operator `!` is always unary (except when combined with equals for not equals,
e.g., `!=`).  It can act as a prefix operator "not", e.g., `!A`, pronounced "not A",
or a postfix operator on a variable, e.g., `Z!`, pronounced "Z mooted" (or "moot Z").  In the first
example, prefix `!` calls the `!(M:): bool` (or `::!(): bool`) method defined on `A`, which creates a
temporary value of the boolean opposite of `A` without modifying `A`.  In the second
case, it calls a built-in method on `Z`, which moves the current data out of `Z` into
a temporary instance of whatever type `Z` is, and resets `Z` to a blank/default state.
The method would look like `::()!: m` or `(M:)!: m`, but again this is defined for you.
This is a "move and reset" operation, or "moot" for short.  Overloads for prefix `!`
should follow the rule that, after e.g., `Z!`, checking whether `Z` evaluates to false,
i.e., by `!Z`, should return true.

Note, it's easier to think about positive boolean actions sometimes than negatives,
so we allow defining either `!!(M:): bool` or `::!!(): bool` on a class, the former
allowing you to cast a value, e.g., `A`, to its positive boolean form `!!A`, pronounced
"not not A."  Note, you cannot define both `!` and `!!` overloads for a class, since
that would make things like `!!!` ambiguous.

## superscripts/exponentiation

Note that exponentiation -- `^` and `**` which are equivalent --
binds less strongly than function calls and member access.  So something like `A[B]^2` will be
equivalent to `(A[B])^2` and `A B^3` is equivalent to `(A::B)^3`.

## bitshifts `<<` and `>>`

The notation `A << B`, called "bitshift left", means to multiply `A` by `2^B`.  For example, 
`A << 1 == A * 2`, `A << 2 == A * 4`, and `A << 3 == A * 8`.  Conversely, "bitshift right"
`A >> B` means to divide `A` by `2^B`.  Typically, we use bitshifts `<<` and `>>`
only for fixed-width integers, so that `A >> 5 == A // 32`, but there are overloads
for other types that will do the expected full division.  For floats, e.g., 16.0 >> 5 == 0.5.
Note that `A << 0 == A >> 0 == A`, and that negating the second operand is the same
as switching the operation, i.e., `A << B == A >> -B`.

In contrast to C/C++, bitshifts have a higher precedence than multiplication and division because they are
"stronger" operations: `100 << 3 == 800` whereas `100 * 3 == 300`, and the difference widens
as the second operand increases; similarly for division, bitshift right `>>` is "stronger"
than division at reducing the first operand via the second operand.
Thus `7 * 31 >> 3` groups as `7 * (31 >> 3) == 21` (and not as `(7 * 31) >> 3 == 27`),
and `105 // 5 << 2` groups as `105 // (5 << 2) == 5` and not `(105 // 5) << 2 == 84`.

Looking the other direction, bitshifts have lower precedence than exponentiation because
exponents are generally stronger -- as long as the first operand, the base of the exponentiation, is
larger than two.  E.g., `3^4 == 81` is greater than `3 << 4 == 48`.
Thus `2 << 3^2 == 2 << (3^2) == 1024` and not `(2 << 3)^2 == 256`, etc.

## division and remainder operators: `/` `//` `%` `%%`

The standard division operator, `/`, will promote integer operands to a rational return value.
E.g., `dbl(3/4) == 0.75` or `6/4 == rtl(3)/rtl(2)`.

The integer division operator, `//`, will return an integer, rounded towards zero, e.g.,`3//4 == 0`
and `-3//4 == 0`.  Also, `5//4 == 1` and `-5//4 == -1`, and `12 // 3 == 4` as expected.
If any operand is a double, the resulting value will be an integer double, e.g.,
`5.1 // 2 == 2.0`.

The modulus operator, `%`, will put the first operand into the range given by the second operand.
E.g., `5 % 4 == 1`, `123.45 % 1 == 0.45`.  Mathematically, we use the relation
`A % B == A - B * floor(A/B)`.

The remainder operator, `%%`, has the property that `A %% B == A - B * (A // B)`;
i.e., it is the remainder after integer division.  The remainder operator, `%%`,
differs from the modulus, `%`, when the operands have opposing signs.

|  `A`  |  `B`  | `floor(A/B)`  |  `A % B`  | `A // B`  | `A %% B`  |
|:-----:|:-----:|:-------------:|:---------:|:---------:|:---------:|
|   1   |   5   |      0        |     1     |     0     |     1     |
|  -1   |   5   |     -1        |     4     |     0     |    -1     |
|   1   |  -5   |     -1        |    -4     |     0     |     1     |
|  -1   |  -5   |      0        |    -1     |     0     |    -1     |
|  13   |   5   |      2        |     3     |     2     |     3     |
| -13   |   5   |     -3        |     2     |    -2     |    -3     |
|  13   |  -5   |     -3        |    -2     |    -2     |     3     |
| -13   |  -5   |      2        |    -3     |     2     |    -3     |
|  56   |   7   |      8        |     0     |     8     |     0     |
|  56   |  -7   |     -8        |     0     |    -8     |     0     |
|  6.78 |   1   |      6.0      |    0.78   |     6.0   |    0.78   |
| -6.78 |   1   |     -7.0      |    0.22   |    -6.0   |   -0.78   |

## less-than/greater-than operators

The less-than, less-than-or-equal-to, greater-than, and greater-than-or-equal-to
binary operators `<`, `<=`, `>`, and `>=` (respectively) have special return types.
This allows chaining like `W >= X < Y <= Z`, which will evaluate as truthy iff
`W >= X`, `X < Y`, and `Y <= Z`.  Note that these expressions are evaluated
left-to-right and the first inequality to fail will stop any further evaluations
or expressions from executing.

Internally, `X < Y` becomes a class which holds onto a value or reference of `Y`,
so that it can be chained.  Any future right operands take over the spot of `Y`.
Note, oh-lang doesn't expose this internal class,
so `Q: X < Y > Z` instantiates `Q` as a boolean.

## and/or/xor operators

If you are looking for bitwise `AND`, `OR`, and `XOR`, they are `&`, `|`, and `><`, respectively.

The operators `and` and `or` act the same as JavaScript `&&` and `||`, as long as the
left hand side is not nullable.  `xor` is an "exclusive or" operator.

The `or` operation `X or Y` has type `one_of[x, y]` (for `X: x` and `Y: y`).
If `X` evaluates to truthy (i.e., `!!X == True`), then the return value of `X or Y` will be `X`.
Otherwise, the return value will be `Y`.  Note in a conditional, e.g., `if X or Y`, we'll always
cast to boolean implicitly (i.e., `if bool(X or Y)` explicitly).

Similarly, the `and` operation `X and Y` also has type `one_of[x, y]`.  If `X` is falsey,
then the return value will be `X`.  If `X` is truthy, the return value will be `Y`.
Again, in a conditional, we'll cast `X and Y` to a boolean.

If the LHS of the expression can take a nullable, then there is a slight modification.
`X or Y` will be `one_of[x, y, null]` and `X and Y` will be `one_of[y, null]`.
The result will be `Null` if both (either) operands are falsey for `or` (`and`).

```
Non_null_or: X or Y         # Non_null_or: if X {X} else {Y}
Non_null_and: X and Y       # Non_null_and: if !X {X} else {Y}
Nullable_or?: X or Y        # Nullable_or?: if X {X} elif Y {Y} else {Null}
Nullable_and?: X and Y      # Nullable_and?: if !!X and !!Y {Null} else {Y}
```

This makes things similar to the `xor` operator, but `xor` always requires a nullable LHS.
The exclusive-or operation `X xor Y` has type `one_of[x, y, null]`, and will return `Null`
if both `X` and `Y` are truthy or if they are both falsey.  If just one of the operands
is truthy, the result will be the truthy operand.  An example implementation:

```
# you can define it as nullable via `xor(~X, ~Y): one_of[x, y, null]` or like this:
xor(~X, ~Y)?: one_of[x, y]
    X_is_true: bool(X)          # `X_is_true: !!X` is also ok.
    Y_is_true: bool(Y)
    if X_is_true
        if Y_is_true {Null} else {X}
    elif Y_is_true
        Y
    else
        Null
```

Thus `xor` will thus return a nullable value, unless you do an assert.

```
Nullable_xor?: X xor Y
Non_null_xor: X xor Y assert()  # will shortcircuit this block if `X xor Y` is null
```

## reassignment operators

Note that `:`, `;`, and `.` can assign values if they're also being declared.
Thus, `=` is only used for reassignment.  Many binary (two operand) operators
such as `*`, `/`, `+`, `-`, etc., also support being paired with reassignment.
As long as `X @op Y` has the same type as `X`, then we can do `X @op = Y` for
shorthand of `X = X @op Y` for any eligible binary operator `@op`.  Examples
include `X -= 5`, `Y &= 0x12`, etc.

Swapping two variables is accomplished by something like `A <-> B`.
Swap uses `<->` since `<=>` is reserved for a future spaceship operator
(encompassing `<`, `<=`, `==`, `=>` and `>` in one).  As a function, swap
would require mutable variables, e.g., `@order_independent ;;x(@New X.): [@Old X]`.
If you define `swap` in this way for your custom class, it will be available
for the shorthand notation `Some_class X <-> 1234`.
TODO: make all "swappers" have the same function signature, not `swap` but
`;;x(X.): x`.  could also use `;;x(X;): null` as the function signature.

## ergo operator

`->` is called the "ergo operator" and is used in conditional logic for
more fine-grained flow control to define a `then` instance which can
break out of loops in more interesting (possibly *less readable*) ways.
Use sparingly.

```
X: if Some_condition -> Then:
    if Other_condition
        Then exit(5)
    Then exit(7)
else -> Then:
    Then exit(10)

# the above is equivalent to the following:
X: if Some_condition { if Other_condition {5} else {7} } else {10}
```

See [then statements](#then-statements) for more examples and details.

# variables

Variables are named using `Variable_case` identifiers.  The `:` symbol is used
to declare deeply constant, non-reassignable variables, and `;` is used to declare
writable, reassignable variables.  Note when passed in as arguments to a function,
`:` has a slightly different meaning; a variable with `:` is readonly and not
necessarily deeply constant.  That will be discussed more later.

```
# declaring and setting a non-reassignable variable that holds a big integer
Y: int = 5
# also equivalent: `Y: 5` or `Y: int(5)`.

# using the variable:
print(Y * 30)

Y = 123     # COMPILER ERROR, Y is readonly and thus non-reassignable.
Y += 3      # COMPILER ERROR, Y is readonly and here deeply constant.
```

Mutable/reassignable/non-constant variables can use `Variable_name = Expression`
after their first initialization, but they must be declared with a `;` symbol.

```
# declaring a reassignable variable that holds a big integer
X; int

# X is default-initialized to 0 if not specified.
X += 5      # now X == 5 is True.

# you can also define the value inline as well:
W; 7
# also equivalent, if you want to be explicit about the type.
W; int = 7
```

Note that we use `;` and `:` as if it were an annotation on the variable name (rather
than the type) so that we don't have to worry about needlessly complex types like a writable
array of a constant integer.  Constant variables are deeply constant, and writable
variables are modifiable/reassignable, and we only have to think about this
(as programmers using the language) at the level of the variable itself,
not based on the type of the variable.  The underlying type is the same for both
readonly and writable variables (i.e., a writable type), but the variable is only
allowed to mutate the memory if it is declared as a writable variable with `;`.

## nullable variable types

To make it easy to indicate when a variable can be nullable, we reserve the question mark
symbol, `?`, placed after the variable name like `X?: int` or after a simple type like
`X: int?`.  Either example declares a variable `X` that
can be an integer or null.  The default value for an optional type is `Null`.

For an optional type with more than one non-null type, we use `Y?: one_of[some_type, another_type]`
or equivalently, `Y: one_of[some_type, another_type, null]` (where `null` comes last).
Note that `null` should come last for casts to work correctly (e.g., `one_of[null, int](1234)`
would cast to null rather than `int(1234)`).  Normally the first value in a `one_of` is the
default, but if `null` or `Null` is an option, then null is the default.  

In either case, you can use `;` instead of `:` to indicate that the variable is writable.
Note that if you are defining a nullable variable inline, you should
prefix the operator with a `?`, e.g., `X?: nullable_result(...)`.  It is a compiler error
if a declared variable is nullable but `?` is not used, since we want the programmer to be
aware of the fact that the variable could be null, even though the program will take care
of null checks automatically and safely.  The `?` operator is required for any `one_of` that
could take on a `Null` value, e.g., `one_of[Null, Bread, Tomato, Mozzarella]`.

One of the cool features of oh-lang is that we don't require the programmer
to check for null on a nullable type before using it.  The executable will automatically
check for null on variables that can be null.  If a function can be null, the executable
will return null if the function is null, or otherwise execute the function.
This is also helpful for method chaining on classes (see more on those below).
If your code calls a method on an instance that is null, a null will be
returned instead (and the method will not be called).

```
# define a class with a method called `some_method`:
some_class: []{ ::some_method(): int }

Nullable?; some_class = Null

Value?: Nullable some_method()  # `Value` has type `one_of[int, null]` now,
                                # so it needs to be defined with `?`

# eventually we want to support things like this, where the compiler
# can tell if the type is nullable or not:
if Nullable != Null
    Non_null_value: Nullable some_method()   # `Non_null_value` here must be `int`.
```

It is not allowed to implicitly cast from a nullable type to a non-nullable type,
e.g., `Value: Nullable some_method()`.  The compiler will require that we define
`Value` with `?:`, or that we explicitly cast via whatever ending type we desire,
e.g., `Value: int(Nullable some_method())`.  Note that `whatever_type(Null)` is
the same as `whatever_type()`, and number types (e.g., `int()` or `flt()`)  default
to 0.

Optional functions are defined in a similar way (cf. section on nullable functions),
with the `?` just after the function name, e.g., `some_function?(...Args): return_type`.

## nullable classes

We will allow defining a nullable type by taking a type and specifying what value
is null on it.  For example, the symmetric type `s8` defines null as `-128` like this:

```
s8: i8 {@Null: -128}

# roughly equivalent to `s8?: s8 { Null: -128_i8, ::is(null): M == -128_i8 }`
```

Similarly, `f32?` and `f64?` indicate that `NaN` is null via `{@Null: NaN, ::is(null): is_nan(M)}`,
so that you can define e.g. a nullable `f32` in exactly 32 bits.  To get this functionality,
you must declare your variable as type `s8?` or `f32?`, so that the nullable checks
kick in.

If you are defining a class and want to also declare the nullable at the same time, you
can do one of the following:

```
my_class: [@private Some_state: int]
{   ;;renew(M Some_state: int): {}

    ::normal_method(): int
        Some_state + 3

    # the nullable definition, inside a class:
    ?: m
    {   Null: [Some_state: -1]
        ::is_null(): Some_state < 0

        ::null_method(): int
            assert(Some_state >= 0)
            Some_state * 5
    }
}

# nullable definition, outside a class (but same file).
# both internal/external definitions aren't required of course.
my_class?: my_class
{   Null: [Some_state: -1]
    ::is_null(): Some_state < 0
    ::null_method(): int
        assert(Some_state >= 0)
        Some_state * 5
}
```

Note that any `one_of` that can be null gets nullable methods.  They are defined globally
since we don't want to make users extend from a base nullable class.

```
# nullish or.
# `Nullable ?? X` to return `X` if `Nullable` is null,
# otherwise the non-null value in `Nullable`.
non_null_or(@First ~A?., @Second A.): a
    what @First A
        Non_null: {Non_null}
        Null {@Second A}

# boolean or.
# `Nullable || X` to return `X` if `Nullable` is null or falsey,
# otherwise the non-null truthy value in `Nullable`.
truthy_or(@First ~A?., @Second A.): a
    what @First A
        Non_null:
            if Non_null
                Non_null
            else
                @Second A
        Null {@Second A}
```

We'll support more complicated pattern matching (like in Rust) using
the `where` operator.  The shorter version of the above `what` statement is:

```
truthy_or(@First ~A?., @Second A.): a
    what @First A
        @Non_null A: where !!@Non_null A
            @Non_null A
        Null
            @Second A
```

In this case, you can think of the `what` cases as being evaluated in order,
and the first one to match will be executed.  Internally there are more optimizations.

## nested/object types

You can declare an object type inline with nested fields.  The nested fields defined
with `:` are readonly, and `;` are writable.

```
Vector; [X: dbl, Y: dbl, Z: dbl] = [X: 4, Y: 3, Z: 1.5]
Vector X += 4   # COMPILER ERROR, field `X` of object is readonly 

# note however, as defined, Vector is reassignable since it was defined with `;`:
Vector = [X: 1, Y: 7.2]
# note, missing fields will be default-initialized.
Vector Z == 0   # should be True.

# to make an object variable readonly, use : when defining:
Vector2: [X: 3.75, Y: 3.25]
# or you can use `:` with an explicit type specifier and then `=`:
Vector2: [X: dbl, Y: dbl] = [X: 3.75, Y: 3.25]
# then these operations are invalid:
Vector2 X += 3          # COMPILER ERROR, variable is readonly, field cannot be modified
Vector2 = [X: 1, Y: 2]  # COMPILER ERROR, variable is readonly, cannot be reassigned
```

You can define a type/interface for objects you use multiple times.

```
# a plain-old-data class with 3 non-reassignable fields, X, Y, Z:
vector3: [X: dbl, Y: dbl, Z: dbl]

# you can use `vector3` now like any other type, e.g.:
Vector3: vector3(X: 5, Y: 10)
```

We also allow type definitions with writable fields, e.g. `[X; int, Y; dbl]`.
Depending on how the variable is defined, however, you may not be able to change
the fields once they are set.  If you define the variable with `;`, then you
can reassign the variable and thus modify the writable fields.  But if you define the
variable with `:`, the object fields are readonly, regardless of the field definitions.
Readonly fields on an object are normally deeply constant, unless the instance is
writable and is reset (either via `renew` or reassignment).  This allows you to
effectively change any internal readonly fields, but only in the constructor.

```
# mix_match has one writable field and one readonly field:
mix_match: [Wr; dbl, Ro: dbl]

# when defined with `;`, the object `Mutable_mix` is writable: mutable and reassignable.
Mutable_mix; mix_match = [Wr: 3, Ro: 4]
Mutable_mix = mix_match(Wr: 6, Ro: 3)   # OK, Mutable_mix is writable and thus reassignable
Mutable_mix renew(Wr: 100, Ro: 300) # OK, will update `Ro` to 300 and `Wr` to 100
Mutable_mix Wr += 4                 # OK, Mutable_mix is writable and this field is writable
Mutable_mix Ro -= 1                 # COMPILE ERROR, Mutable_mix is writable but this field is readonly
                                    # if you want to modify the `Ro` field, you need to reassign
                                    # the variable completely or call `renew`.

# when defined with `:`, the object is readonly, so its fields cannot be changed:
Readonly_mix: mix_match = [Wr: 5, Ro: 3]
Readonly_mix = mix_match(Wr: 6, Ro: 4)  # COMPILE ERROR, Readonly_mix is readonly, thus non-reassignable
Readonly_mix renew(Wr: 7, Ro: 5)        # COMPILE ERROR, Readonly_mix is readonly, thus non-renewable
Readonly_mix Wr += 4                    # COMPILE ERROR, Readonly_mix is readonly
Readonly_mix Ro -= 1                    # COMPILE ERROR, Readonly_mix is readonly

# NOTE that in general, calling a function with variables defined by `;` is a different
# overload than calling with `:`.  Mutable argument variables imply that the arguments will
# be mutated inside the function, and because they are passed by reference, escape the function
# block with changes.  Data classes have overloads with writable arguments, which indicate
# the data class will swap out the argument (e.g., giving you the old version while taking
# what's passed in).  In case of a constructor, the old value is the default value.
Wr; 123
Ro; 567
My_mix_match: mix_match(;Wr, ;Ro)   # `;` is useful for taking arguments via a swap.
print("got updated to ${Wr}, ${Ro}") # "got updated to 0, 0"
# see section on writable/readonly arguments for more information.
```

Note that oh-lang takes a different approach than C++ when it comes to constant/readonly fields
inside of classes.  In C++, using `const` on a field type bars reassignment of the class instance.
(`non-static const member ‘const t T’, cannot use default assignment operator`.)
In oh-lang, readonly variables are not always deeply constant.  And in the case of readonly class
instance fields, readonly variables are set based on the constructor and shouldn't be modified
afterwards by other methods... except for the constructor if it's called again (i.e., via
`renew`ing the instance or reassignment).

### automatic deep nesting

We can create deeply nested objects by adding valid identifiers with consecutive `:`.  E.g.,
`[X: Y: 3]` is the same as `[X: [Y: 3]]`.  Similarly for `()` and `{}`.

## temporarily locking writable variables

You can also make a variable readonly
for the remainder of the current block by using `@lock` before the variable name.
Note that you can modify it one last time with the `@lock` annotation, if desired.
Also note that the variable may not be deeply constant, e.g., if lambdas are called
which modify it, but you will not be able to explicitly modify it.

```
X; int = 4  # defined as writable and reassignable

if Some_condition
    @lock X = 7 # locks X after assigning it to the value of 7.
                # For the remainder of this indented block, you can use X but not reassign it.
                # You also can't use writable, i.e., non-const, methods on X.
else
    @lock X # lock X to whatever value it was for this block.
            # You can still use X but not reassign/mutate it.

print(X)    # will either be 7 (if Some_condition was true) or 4 (if !Some_condition)
X += 5      # can modify X back in this block; there are no constraints here.
```

## hiding variables

We can hide a variable from the current block by using `@hide` before the variable name.
This doesn't descope the variable, but it does prevent the variable from being used by
new statements/functions.  `@hide` has similar behavior to the `@lock` annotation, in that
you can use the variable one last time with the annotation, if desired.

```
Date_string; str("2023-01-01")

# after this line, `Date_string` can't be accessed anymore.
Date: date(@hide Date_string)

# note in some circumstances you may also want to include `!` to avoid copying the variable,
# if the underlying class makes use of that same type variable internally, e.g.:
Date: date(@hide Date_string!)
# see discussion on `moot` for more information.
```

In fact, hiding variables make it possible to shadow identifiers; i.e.,
for variable renaming.  See the following example:

```
do_something(Date: str("2023-01-01")):
    Date: date(@hide Date!)
```

# functions

Functions are named using `function_case` identifiers.  The syntax to declare
a function is `function_case_name(Function_arguments...): return_type`, but if
you are also defining the function the `return_type` is optional (but generally
recommended for multiline definitions).  Defining the function can occur inline
with `:` or over multiple lines using an indented block.

```
# declaring a function with no arguments that returns a big integer
v(): int

# setting/defining/initializing the function:
v(): int
    # `return` is optional for the last line in a block.
    # e.g., the following could have been `return 600`.
    600

# inline definition
v(): 600

# inline, but with explicit type
v(): int(600)

# function with X,Y double-precision float arguments that returns nothing
v(X: dbl, Y: dbl): null
    print("X = ${X}, Y = ${Y}, atan(Y, X) = ${\\math atan(X, Y)}")
    # Note this could also be defined more concisely using $(),
    # which also prints the expression inside the parentheses with an equal sign and its value,
    # although this will print `X: ..., Y: ..., Atan: ...`, e.g.:
    # print("$(X, Y, \\math atan(X, Y))")

# Note that it is also ok to use parentheses around a function definition,
# but you should use braces `{}`.
excite(Times: int): str
{   "hi!" * Times
}

# You can define a "multiline" function in one line like this:
oh(Really; dbl): dbl { Really *= 2.5, return 50 + Really }
```

Note that we disallow the inverted syntax of `function_name: return_type(...Args)`
because this looks like declaring a type (e.g., no parentheses on the left hand side)
and the right hand side looks like how we call a function and get an instance (not a type).
See [returning a type](#returning-a-type) for how we'd return a type.

## calling a function

You can call functions with arguments in any order.  Arguments must be specified
with the named identifiers in the function definition.  The only exception is
if the argument is default-named (i.e., it has the same name as the type), then you
don't need to specify its name.  We'll discuss that more in the
[default-name arguments](#default-name-arguments-in-functions) section.

```
# definition:
v(X: dbl, Y: dbl): null

# example calls:
v(X: 5.4, Y: 3)
v(Y: 3, Y: 5.4)

# if you already have variables X and Y, you don't need to re-specify their names:
X: 5.4
Y: 3
v(X, Y)     # equivalent to `v(X: X, Y: Y)` but the redundancy is not idiomatic.
v(Y, X)     # equivalent
```

### references

We can create references using [reference objects](#reference-objects) in the following way.
Note that you can use all the same methods on a reference as the original type.

```
My_value; int(1234567890)
(My_ref; int) = My_value
# equivalent: `My_ref; (Int;) = My_value`
(My_readonly_ref: int) = My_value
# equivalent: `My_readonly_ref: (Int:) = My_value`

# NOTE: `My_value` and the `My_ref` reference need to be writable for this to work.
My_ref = 12345
# My_readonly_ref = 123 # COMPILE ERROR!

# This is true; `My_value` was updated via the reference `My_ref`
My_value == 12345
My_readonly_ref == 12345 # also true.

# There is no need to "dereference" the pointer
print(My_ref * 77)
print(My_readonly_ref * 23)
```

Unlike in C++, there's also an easy way to change the reference to point to
another instance.  This does require a bit more syntax if you are pointing
to a readonly value like `(Referent_type:)`, since you'll need to refer to it
in a way that lets you modify the reference itself.

```
My_value1: int(1234)
My_value2: int(765)
My_ref; (Int:) = My_value1
# This works only for `My_ref;` declarations.  The actual data can be readonly,
# as it is in this case (`My_value1` and `2` are both readonly), or writable.
(My_ref) = My_value2
```

Note that by default, references like `(My_ref; int) = some_reference()`
will be reassignable, i.e., defined like `My_ref; (Int;) = some_reference()`,
and references like `(My_ref: int) = some_reference()` will not be reassignable,
i.e., defined like `My_ref: (Int:) = some_reference()`.  If you want a readonly-
referent reference to be reassignable, use `My_ref; (Int:) = ...`.

You can grab a few references at a time using [destructuring](#destructuring)
notation like this:

```
Ref3; (Str:) = some_ref()
# this declares+defines `Ref1` and `Ref2`, and reassigns `Ref3`:
(Ref1;, Ref2:, Ref3) = some_function_that_returns_refs()

# e.g., with function signature:
some_function_that_returns_refs(): (Ref2; int, Ref2: dbl, Ref3; str)
```

#### reference objects

TODO: we probably need a borrow checker (like Rust):

```
Result?; some_nullable_result()
if Result is Non_null:
    print(Non_null)
    Result = some_other_function_possibly_null()
    # this could be undefined behavior if `Non_null` is a reference to the
    # nonnull part of `Result` but `Result` became null with `some_other_function_possibly_null()`
    print(Non_null)
```

Alternatively, we pass around "full references" whenever we can't determine that
borrowing can be done with just a pointer.  Full references include a path from
a safely-borrowed pointer, with checks at each nested value for any additions
that need to be made.  In the above example, we need `Non_null` to be a pointer
from `Result` that checks if `Result` is non-null before any dereferencing.  The
above example can be checked by the compiler, but if `Result` was itself a reference
path then we'd need to recheck any dereferences of `Non_null`.

In oh-lang, parentheses can be used to define reference objects, both as types
and instances.  As a type, `(X: dbl, Y; int, Z. str)` differs from the object
type `[X: dbl, Y; int, Z; str]`, for more than just the reason that `.` is invalid
in an object type.  When instantiated, reference objects with `;` and `:` fields
contain references to variables; objects get their own copies.

Because they contain references, reference object instances cannot outlive the lifetime
of the variables they contain.

```
a: (X: dbl, Y; int, Z. str)

# This is OK:
X: 3.0
Y; 123
A: (X, Y, Z. "hello")    # `Z` is passed by value, so it's not a reference.
A Y *= 37    # OK

# This is not OK:
return_a(Q: int): a
    # X and Y are defined locally here, and will be descoped at the
    # end of this function call.
    X: Q dbl() ok_or(NaN) * 4.567
    Y; Q * 3
    # So we can't pass X, Y as references here.  Z is fine.
    (X, Y, Z. "world")
```

Note that we can return reference object instances from functions, but they must be
defined with variables whose lifetimes outlive the input reference object instance.
For example:

```
X: 4.56
return_a(Q; int): (X: dbl, Y; int, Z. str)       # inline reference object type
    Q *= 37
    # X has a lifetime that outlives this function.
    # Y has the lifetime of the passed-in variable, which exceeds the return type.
    # Z is passed by value, so no lifetime concerns.
    (X, Y; Q, Z. "sky")
```

Argument objects are helpful if you want to have arguments that should be
references, but need nesting to be the most clear.  For example:

```
# function declaration
copy(From: (Pixels, Rectangle.), To: (Pixels;, Rectangle.): null

# function usage
@Source Pixels: pixels() { #( build image )# }
@Destination Pixels; pixels()
Size: rectangle(Width: 10, Height: 7)

copy
(   From: 
    (   @Source Pixels
        Size + Vector2(X: 3, Y: 4)
    )
    To:
    (   @Destination Pixels;
        Size + Vector2(X: 9, Y: 8)
    )
)
```

We can create deeply nested reference objects by adding valid identifiers with consecutive `:`/`;`/`.`.
E.g., `(X: Y: 3)` is the same as `(X: (Y: 3))`.  This can be useful for a function signature
like `run(After: duration, fn(): ~t): t`.  `duration` is a built-in type that can be built
out of units of time like `Seconds`, `Minutes`, `Hours`, etc., so we can do something like
`run(After: Seconds: 3, (): print("hello world!"))`, which will automatically pass
`(Seconds: 3)` into the `duration` constructor.  Of course, if you need multiple units of time,
you'd use `run(After: (Seconds: 6, Minutes: 1), (): print("hello world!"))` or to be explicit
you'd use `run(After: duration(Seconds: 6, Minutes: 1), (): print("hello world!"))`.


#### reference lifetimes

References are not allowed to escape the block in which their referent is defined.
For example, this is illegal:

```
Original_referent: int = 3
My_reference: (Int) = Original_referent
if Some_condition
{   Nested_referent: int
    # COMPILE ERROR: `Nested_referent` doesn't live as long as `My_reference`
    (My_reference) = Nested_referent
}
```

However, since function arguments can be references (e.g., if they are defined with
`:` or `;`), references that use these function arguments can escape the function block.

```
fifth_element(Array[int];): (Int;)
    # this is OK because `Array` is a mutable reference
    # to an array that already exists outside of this scope.
    (;Array[4])

My_array; array[int](1, 2, 3, 4, 5, 6)
(Fifth;) = fifth_element(;My_array)
Fifth += 100
My_array == [1, 2, 3, 4, 105, 6]    # should be true
```

#### refer function

If you need some special logic before returning a reference, e.g., to create a default,
you can use the `refer` function with the following signature: `refer(~R;, fn(R;): (~T;)`
and similarly for a constant reference (swap `;` with `:` everywhere).  There's also a
key-like interface (e.g., for arrays or lots):

```
# if `At` is passed as a temporary, it should be easily copyable.
refer(~R;:, At` ~k, fn(R;:, K:): (~T;:)): (T;:)`
```

You can also create a reference via getters and setters using the `refer` function, which
has the following signature: `refer(~R;, @Getter fn(R): ~t, @Setter fn(R;, T.): null): (T;)`.
It extends a base reference to `R` to provide a reference to a `t` instance.
There's also a key-like interface (e.g., for arrays or lots):

```
# if `At` is passed as a temporary, it should be easily copyable.
refer(~R;:, At` ~k, @Getter fn(R:, K:): ~t, @Setter fn(R;:, K:, T.): null): (T;)`
```

When calling `refer`, we want the getters and setters to be known at compile time,
so that we can elide the reference object creation when possible.

```
My_array; [1, 2, 3, 4]

# here we can elide `refer` here that is inside the method
# `array[int];;[Index]: (Int;)`, and call the setter immediately:
My_array[0] = 0     # My_array == [0, 2, 3, 4]

# here we cannot elide `refer`
(My_reference;) = My_array[2]
My_reference += 3   # My_array == [0, 2, 6, 4]
print(My_reference) # prints `6`
```

### default-name arguments in functions

For functions with one argument (per type) where the variable name doesn't matter,
you can use default-named variables.  For standard ASCII identifiers, the default-name identifier
is just the `Variable_case` version of the `type_case` type.

```
# this function declaration is equivalent to `f(Int: int): int`:
f(Int:): int
    Int + 5

Z: 3
f(Z)                    # ok
f(4.3 floor() int())    # ok
f(5)                    # ok
f(Int: 7)               # ok but overly verbose
```

If passing functions as an argument where the function name doesn't matter,
there are actually a few options: `a`, `an`, `f`, `fn`, and `do`.
We recommend `a` and `an` for `map`-like operations with a single argument,
choosing `an` if the argument name starts with a vowel (and `a` otherwise),
and `do` for multi-argument functions.  We keep `f` and `fn` around
mostly to make it easy for developers new to the language.  Note that if
any functions are defined, including default named functions, no variables
can shadow their `Initial_upper_snake_case` form.  And vice versa.

```
# declaring a function that takes a lambda, note the default name.
q(fn(): bool): null

# defining a function that takes a lambda.
q(fn(): bool): null
    if fn()
        print("function returned true!")
    else
        print("function returned false!")

q(name_it_what_you_want(): bool
    return True
)   # should print "function returned true!"

# or you can create a default-named function yourself:
q
(   fn(): bool
        random() > 0.5
)   # will print one of the above due to randomness.
# equivalent to `q(fn(): random() > 0.5)` or `q({random() > 0.5})`

# defining a lambda usually requires a name, feel free to use the default:
q(fn(): True)
# or you can use this notation, without the name:
q({True})

# or you can do multiline:
X; bool
q
(   fn():
        X
)
# equivalent to `q(fn(): {X})`
# also equivalent to `q({X})`
```

You need to use braces so that we can distinguish function definitions
from function assignments, i.e., assigning the value of one function to another,
like `my_mutable_fn(); int {original_definition()}`, followed by
`my_mutable_fn(); int = some_other_fn`.  See [mutable functions](#mutable-functions).

### the name of a called function in a reference object

Calling a function with one argument being defined by a nested function will use
the nested function's name as the variable name.  E.g., if a function is called
`value`, then executing `what_is_this(value())` will try to call the `what_is_this(Value)`
overload.  If there is no such overload, it will fall back on `what_is_this(Type)` where
`Type` is the return value of the `value()` function.

```
value(): int
    return 1234 + 5

what_is_this(Value: int): null
    print(Value)

what_is_this(Value: 10)   # prints 10
what_is_this(value())     # prints 1239
```

You can still use `value()` as an argument for a default-named `Int` argument,
or some other named argument by renaming.

```
takes_default(Int): string
    string(Int)

takes_default(value())   # OK.  we try `Value: value()`
                        # and then the type of `value()` next

other_function(Not_value: int): string
    return "!" * Not_value

other_function(value())              # ERROR! no overload for `Value` or for `Int`.
other_function(Not_value: value())    # OK
```

This works the same for plain-old-data objects, e.g., `[value()]` corresponds to
`[Value: value()]`.  In case class methods are being called, the class name
and the class instance variable name are ignored, e.g., `[My_class_instance my_function()]`
is short-hand for `[My_function: My_class_instance my_function()]`.

### functions as arguments

A function can have a function as an argument, and there are a few different ways to call
it in that case.  This is usually a good use-case for lambda functions, which define
an inline function to pass into the other function.  Because we support
[function overloading](#function-overloads), any externally defined functions need to be
fully specified.  (E.g., this is not allowed: `greet(Int): "hello" + "!" * Int, do_greet(greet)`.)
This is also because we allow passing in types as function arguments, so anything that is
`function_case` without a subsequent parenthesized argument list `(Args...)` will be considered
`type_case` instead.

```
# finds the integer input that produces "hello, world!" from the passed-in function, or -1
# if it can't find it.
detect(greet(Int): string): int
    100 each @Check Int:
        if greet(@Check Int) == "hello, world!"
            return @Check Int
    return -1

# if your function is named the same as the function argument...
greet(Int): string
    return "hay"
# you can use it directly, although you still need to specify which overload you're using,
detect(greet(Int): string)  # returns -1
# also ok, but a bit verbose:
detect(greet(Int): greet(Int) String)

# if your function is not named the same, you can do argument renaming;
# internally this does not create a new function:
say_hi(Int): string
    return "hello, world" + "!" * Int
detect(greet(Int): string = say_hi) # returns 1

# you can also create a function named correctly inline -- the function
# will not be available outside, after this call (it's scoped to the function arguments).
detect
(   greet(Int): string
        "hello, world!!!!" substring(Length: Int)
)   # returns 13

detect(greet(Int): {["hi", "hey", hello"][Int % 3] + ", world!"}) # returns 2
```

### lambda functions

Lambda functions are good candidates for [functions as arguments](#functions-as-arguments),
since they are very concise ways to define a function.  They utilize an indented block
or set of braces  like `{...function-body...}` with function arguments defined inside using
`$The_argument_name`.  There is no way to specify the type of a lambda function argument,
so the compiler must be able to infer it (e.g., via using the lambda function as an argument,
or by using a default name like `$Int` to define an integer).  Some examples:

```
run_asdf(do(J: int, K: str, L: dbl): null): null
    print(do(J: 5, K: "hay", L: 3.14))

# Note that `$K`, `$J`, and `$L` attach to the same lambda based on looking
# for the first matching `{}`.
run_asdf({$K * $J + str($L)})   # prints "hayhayhayhayhay3.14"

# One example with brackets:
My_array: [0.06, 0.5, 4.0, 30.0, 200.0, 1000.0]
# Again, `$K`, `$J`, and `$L` attach to the same lambda.
run_asdf({$K + str(My_array[$J] * $L)})  # prints "hay3140
# The same example with an indent:
run_asdf
(   $K + str(My_array[$J] * $L)
)
# this is wrong, this looks like line continuation.
run_asdf
(       $K + str(My_array[$J] * $L)
)
```

If you need a lambda function inside a lambda function, use another `$` to escape
one variable into the parent scope, e.g.,

```
# with function signatures
# `run(fn(X: any): any): any` and
# `run_nested(fn(Y: any): any): any`
run({$X + run_nested({$Y + $$X})})

# or with indents
run
(   $X + run_nested
    (   $Y + $$X
    )
)
```

But it would probably be more readable to just define the functions normally in this instance.

There is currently no good way to define the name of a lambda function; we may use
`@named(whatever_name) {$X + $Y}`, but it's probably more readable to just define
the function inline as `whatever_name(X, Y): X + Y`.
TODO: would `$named{$X + $Y}` work??

### types as arguments

Generally speaking you can use generic/template programming for this case,
which infers the types based on instances of the type.

```
# generic function taking an instance of `x` and returning one.
do_something(~X): x
    return X * 2

do_something(123)    # returns 246
do_something(0.75)   # returns 1.5
```
See [generic/template functions](#generictemplate-functions) for more details
on the syntax.

However, there are use cases where we might actually want to pass in
the type of something.  This makes the most sense as a generic type,
and can be done like this:

```
do_something(~x): x
    return x(123)

print(do_something(dbl)) # returns 123.0
print(do_something(u8))  # returns u8(123)
```

### returning a type

We use a different syntax for functions that return types; namely `()` becomes `[]`,
e.g., `type_fn[Args...]: the_type`.  This is because we do not need
to support functions that return instances *or* constructors, and it becomes clearer
that we're dealing with a type if we use `[]`.  The alternative would be to use
`fn(Int): Int` to return an `int` instance and `fn(Int): int` to return the
`int` constructor, but again we never need to mix and match.  The bracket syntax is
related to [template classes](#generictemplate-classes) and
[overloading generic types](#overloading-generic-types).

```
# it's preferable to return a more specific value here, like
# `one_of[int, dbl, string]`, but `any` works as well.
random_class[]: any
    if random(dbl) < 0.5
        int
    elif random(dbl) < 0.5
        dbl
    else
        string

X: random_class[] = 123
match X
    Int:
        print("X is an int")
    Dbl:
        print("X is a dbl")
    String:
        print("X is a string")
```

We can also pass in named types as arguments.  Here is an example
where we also return a type constructor.  Named types are just
`type_case` on both left and right sides (e.g., `class_name: t`).

```
random_class[~x, named_new: ~y]: one_of[x, y]
    return if random(dbl) < 0.5 {x} else {named_new}

print(random_class[int, named_new: dbl])  # will print `int` or `dbl` with 50-50 probability
```

To return multiple types, you can use the [type tuple syntax](#type-tuples).

### unique argument names

Arguments must have unique names; e.g., you must not declare a function with two arguments
that have the same name.  This is obvious because we wouldn't be able to distinguish between
the two arguments inside the function body.

```
my_fun(X: int, X: dbl): one_of[int, dbl] = X    # COMPILER ERROR.  duplicate identifiers
```

However, there are times where it is useful for a function to have two arguments with the same
name, and that's for default-named arguments in a function where (1) *order doesn't matter*,
or (2) order does matter but in an established convention, like two sides of a binary operand.
An example of (1) is in a function like `max`:

```
@order_independent
max(Int, @Other Int): int
    return if Int > @Other Int
        Int
    else
        @Other Int

max(5, 3) == max(3, 5)
```

The compiler is not smart enough to know whether order matters or not, so we need to annotate
the function with `@order_independent` -- otherwise it's a compiler error -- and we need to use
namespaces (e.g., `@Other` with `@Other Int`) in order to distinguish between the two variables
inside the function block.  When calling `max`, we don't need to use those namespaces, and
can't (since they're invisible to the outside world).

There is one place where it is not obvious that two arguments might have the same name, and
that is in method definitions.  Take for example the vector dot product:

```
vector2: [X; dbl, Y; dbl]
{   ;;renew(M X. dbl, M Y. dbl): {}

    @order_independent
    ::dot(Vector2): dbl
        return M X * Vector2 X + M Y * Vector2 Y
}
Vector2: vector2(1, 2)
Other_vector2: vector2(3, -4)
print(Vector2 dot(Other_vector2))    # prints -5
print(dot(Vector2, Other_vector2))   # equivalent, prints -5
```

The method `::dot(Vector2): dbl` has a function signature `dot(M, Vector2): dbl`,
where `M` is an instance of `vector2`, so ultimately this function creates a global
function with the function signature `dot(Vector2, Vector2): dbl`.  Therefore this function
*must* be order independent and should be annotated as such.  Otherwise it is
a compiler error.

As mentioned earlier, we can have order dependence in certain established cases, but these
should be avoided in oh-lang as much as possible, where we prefer unique names.
One example is the cross product of two vectors, where order matters but the
names of the vectors don't.  (The relationship between the two orders is also
somewhat trivial, `A cross(B) == -B cross(A)`, and this simplicity should be aspired to.)
The way to accomplish this in oh-lang is to use `@First` and `@Second` namespaces for
each variable.  If defined in a method, `M` will be assumed to be namespaced as `@First`,
so you can use `@Second` for the other variable being passed in.  Using `@First` and `@Second`
allows you to avoid the compiler errors like `@order_independent` does.  You can also
use `O` as the variable name which in the class body is the same as `@Second M`.

```
vector3: [X; dbl, Y; dbl, Z; dbl]
{   ;;renew(M X. dbl, M Y. dbl, M Z. dbl): {}

    # defined in the class body, we do it like this:
    ::cross(O): vector3
    {   # we could drop `M X` for just `X` here but i like the symmetry with `O`.
        vector3
        (   X. M Y * O Z - M Z * O Y
            Y. M Z * O X - M X * O Z
            Z. M X * O Y - M Y * O X
        )
    }
}

# defined outside the class body, we do it like this:
# NOTE: both definitions are *not* required, only one.
cross(@First Vector3, @Second Vector3): vector3
(   X: @First Vector3 Y * @Second Vector3 Z - @First Vector3 Z * @Second Vector3 Y
    Y: @First Vector3 Z * @Second Vector3 X - @First Vector3 X * @Second Vector3 Z
    Z: @First Vector3 X * @Second Vector3 Y - @First Vector3 Y * @Second Vector3 X
)
```

One final note is that operations like `+` should be order independent, whereas `+=` should be
order dependent, since the method would look like this: `;;+=(Vector2)`, which is
equivalent to `+=(M;, Vector2)`, where the first argument is writable.  These
two arguments can be distinguished because of the writeability.

## function overloads

Functions can do different things based on which arguments are passed in.
For one function to be an overload of another function, it must be defined in the same file,
and it must have different argument names or return values.  You can also have different
argument modifiers (i.e., `;` and `:` are different overloads, as are nullable types, `?`).

```
greet(String): null
    print("Hello, ${String}!")

greet(Say: string, To: string): null
    print("${Say}, ${To}!")

greet(Say: string, To: string, Times: int): null
    range(Times) each Int_:
        greet(Say, To)

# so you call this in different ways:
greet("World")
greet(To: "you", Say: "Hi")
greet(Times: 5, Say: "Hey", To: "Sam")

# note this is a different overload, since it must be called with `Say;`
greet(Say; string): null
    Say += " wow"
    print("${Say}, world...")

My_say; "hello"
greet(Say; My_say)   # prints "hello wow, world..."
print(My_say)            # prints "hello wow" since My_say was modified
```

Note also, overloads must be distinguishable based on argument **names**, not types.
Name modifiers (i.e., `;`, `:`, and `?`) also count as different overloads.

```
fibonacci(Times: int): int
    Previous; 1
    Current; 0
    range(Times) each Int:
        Next_previous: Current
        Current += Previous
        Previous = Next_previous
    return Current

fibonacci(Times: dbl): int
    Golden_ratio: dbl = (1.0 + \\math sqrt(5)) * 0.5
    Other_ratio: dbl = (1.0 - \\math sqrt(5)) * 0.5
    return round((Golden_ratio^Times - Other_ratio^Times) / \\math sqrt(5))
# COMPILE ERROR: function overloads of `fibonacci` must have unique argument names,
#                not argument types.

# NOTE: if the second function returned a `dbl`, then we actually could distinguish between
# the two overloads.  This is because default names for each return would be `Int` and `Dbl`,
# respectively, and that would be enough to distinguish the two functions.  The first overload
# would still be default in the case of a non-matching name (e.g., `Result: fibonnaci(Times: 3)`),
# but we could determine `Int: fibonacci(Times: 3)` to definitely be the first overload and
# `Dbl: fibonacci(Times: 7.3)` to be the second overload.
```

There is the matter of how to determine which overload to call.  We consider
only overloads that possess all the specified input argument names.  If there are some
unknown names in the call, we'll check for matching types.  E.g., an unnamed `4.5`, 
as in `fn(4.5)` or `fn(X, 4.5)`, will be checked for a default-named `Dbl` since `4.5`
is of type `dbl`.  Similarly, if a named variable, e.g., `Q` doesn't match in `fn(Q)`
or `fn(X, Q)`, we'll check if there is an overload of `fn` with a default-named type
of `Q`; e.g., if `Q` is of type `animal`, then we'll look for the `fn(Animal):` 
or `fn(X, Animal)` overload.

NOTE: we cannot match a function overload that has more arguments than we supplied in
a function call.  If we want to allow missing arguments in the function call, the declaration
should be explicit about that; e.g., `fn(X?: int): ...` or `fn(X: 0): ...`.
Similarly, we cannot match an overload that has fewer arguments than we supplied in the call.

Output arguments are similar, and are also matched by name.  This is pretty obvious with
something like `X: calling(Input_args...)`, which will first look for an `X` output name
to match, such as `calling(Input_args...): [X: whatever_type]`.  If there is no `X` output
name, then the first non-null, default-named output overload will be used.  E.g., if
`calling(Input_args...): dbl` was defined before `calling(Input_args...): str`, then `dbl`
will win.  For an output variable with an explicit field request, e.g., `X: calling(Input_args...) Dbl`,
this will look for an overload with output name `Dbl` first.  If there is no output named `X`,
then `X: dbl = calling(Input_args...)` will also work to get the `Dbl` field.  For function calls like
`X: dbl(calling(Input_args...))`, we will lose all `X` output name information because
`dbl(...)` will hide the `X` name.  In this case, we'll use the default overload and attempt
to convert it to a `dbl`.

One downside of overloads is that we must specify the whole function
[when passing it in as an argument](#functions-as-arguments).  But we also need to specify
the whole function because we want to be able to distinguish `type_case` from `function_case`
(where some parenthesized arguments `(Args...)` follow).

### nullable input arguments

When you call a function with an argument that is null, conceptually we choose the
overload that doesn't include that argument.  In other words, a null argument is
the same as a missing argument when it comes to function overloads.  Thus, we are
not free to create overloads that attempt to distinguish between all of these cases:
(1) missing argument, (2) present argument, (3) nullable argument, and (4) default argument.
Only functions for Cases (1) and (2) can be simultaneously defined; any other combination
results in a compile-time error.  Cases (3) and (4) can each be thought of as defining two function
overloads, one for the case of the missing argument and one for the case of the present argument.

Defining conflicting overloads, of course, is impossible.  Here are some example overloads;
again, only Cases (1) and (2) are compatible and can be defined together.

```
# missing argument (case 1):
some_function(): dbl
    return 987.6

# present argument (case 2):
some_function(Y: int): dbl
    return 2.3 * dbl(Y)

# nullable argument (case 3):
some_function(Y?: int): dbl
    if Y != Null {1.77} else {Y + 2.71}

# default argument (case 4):
some_function(Y: 3): dbl
    return dbl(Y)
```

Note that writable arguments `;` are distinct overloads, which indicate either mutating
the external variable, taking it, or swapping it with some other value, depending on
the behavior of the function.  Temporaries are also allowed, so defaults can be defined
for writable arguments.

What are some of the practical outcomes of these overloads?  Suppose
we define present and missing argument overloads in the following way:

```
overloaded(): dbl
    return 123.4
overloaded(Y: int): string
    return "hi ${Y}"
```

The behavior that we get when we call `overloaded` will depend on whether we
pass in a `Y` or not.  But if we pass in a null `Y`, then we also will end up
calling the overload that defined the missing argument case.  I.e.:

```
Y?; int = ... # Y is maybe null, maybe non-null

# the following calls `overloaded()` if Y is Null, otherwise `overloaded(Y)`:
Z: overloaded(Y?) # also OK, but not idiomatic: `Z: overloaded(Y?: Y)`
# Z has type `one_of[dbl, string]` due to the different return types of the overloads.
```

The reason behind this behavior is that in oh-lang, an argument list is conceptually an object
with various fields, since an argument has a name (the field name) as well as a value (the field value).
An argument list with a field that is `Null` should not be distinguishable from an argument list that
does not have the field, since `Null` is the absence of a value.

Note that when calling a function with a nullable variable/expression, we need to
indicate that the field is nullable if the expression itself is null (or nullable). 
Just like when we define nullable variables, we use `?:` or `?;`, we need to use
`?:` or `?;` (or some equivalent) when passing a nullable field.  For example:

```
some_function(X?: int): int
    return X ?? 1000

# when argument is not null:
some_function(X: 100)    # OK, expression for X is definitely not null
some_function(X?: 100)   # ERROR! expression for X is definitely not null

# when argument is an existing variable:
X?; Null
print(some_function(X?))  # can do `X?: X`, but that's not idiomatic.

# when argument is a new nullable expression:
some_function(X?: some_nullish_function())  # REQUIRED since some_nullish_function can return a Null
some_function(X: some_nullish_function())   # ERROR! some_nullish_function is nullable, need `X?:`.

# where some_nullish_function might look like this:
some_nullish_function()?: int
    return if Some_condition { Null } else { 100 }
```

Note however that if a value is definitely null, then we don't allow passing
it in as a nullable argument.

```
some_null_function(): null
    print("go team")
 
# COMPILE ERROR, `X` is always null.
# cleaner: `some_null_function(), some_function()`:
some_function(X?: some_null_function())
```

We also want to make it easy to chain function calls with variables that might be null,
where we actually don't want to call an overload of the function if the argument is null.

```
# in other languages, you might check for null before calling a function on a value.
# this is also valid oh-lang but it's not idiomatic:
X?: if Y != Null { overloaded(Y) } else { Null }

# instead, you should use the more idiomatic oh-lang version.
# putting a ? *before* the argument name will check that argument;
# if it is Null, the function will not be called and Null will be returned instead.
X?: overloaded(?Y)

# either way, X has type `one_of[string, null]`.
```

You can use prefix `?` with multiple arguments; if any argument with prefix `?` is null,
then the function will not be called.

This can also be used with the `return` function to only return if the value is not null.

```
do_something(X?: int): int
    Y?: ?X * 3    # Y is Null or X*3 if X is not Null.
    return ?Y       # only returns if Y is not Null
    #( do some other stuff )#
    ...
    return 3
```

### nullable output arguments

We also support function overloads for outputs that are nullable.  Just like with overloads
for nullable input arguments, there are some restrictions on defining overloads with (1) a
missing output, (2) a present output, and (3) a nullable output.  The restriction is a bit
different here, in that we cannot define (1) and (3) simultaneously for nullable outputs.
This enables us to distinguish between, e.g., `X?: my_overload(Y)` and `X: my_overload(Y)`,
which defines a nullable `X` or a non-null `X`.

```
# case 1, missing output (not compatible with case 3):
my_overload(Y: str): null
    print(Y)

# case 2, present output:
my_overload(Y: str): [X: int]
    [X: int(Y) ?? panic("should be an integer")]

# case 3, nullable output (not compatible with case 1):
my_overload(Y: str): [X?: int]
    # this is essentially an implementation of `X?: int(Y), return [X]`
    what int(Y)
        Ok: $[X: Ok]
        Er: $[]

[X]: my_overload(Y: "1234")  # calls (2) if it's defined, otherwise it's a compiler error.
[X?]: my_overload(Y: "abc")  # calls (1) or (3) if one is defined, otherwise it's a compiler error.
```

Note that if only Case 3 is defined, we can use `assert`s to ensure that the return
value is not null, e.g., `[X]: my_overload() assert()`.  This will throw a run-time error if the return
value for `X` is null.  Note that this syntax is invalid if Case 2 is defined, since there is
no need to assert a non-null return value in that case.  This will also work for an
overload which returns a result `hm`.

```
# normal call for case 3, defines an X which may be null:
[X?]: my_overload(Y: "123")

# special call for case 3; if X is null, this will throw a run-time error,
# otherwise will define a non-null X:
[X]: my_overload(Y: "123") assert()

# make a default for case 3, in case X comes back as null from the function
[X: -1] = my_overload(Y: "123")
```

If there are multiple return arguments, i.e., via an output type data class,
e.g., `[X: dbl, Y: str]`, then we support [destructuring](#destructuring)
to figure out which overload should be used.  E.g., `[X, Y]: my_overload()` will
look for an overload with outputs named `X` and `Y`.  Due to assumptions with
[single field objects](#single-field-objects) (SFO), `X: my_overload()` is
equivalent to `[X]: my_overload()`.  You can also explicitly type the return value,
e.g., `@Some Int: my_overload()` or `R: my_overload() Dbl`,
which will look for an overload with an `int` or `dbl` return type, respectively.

When matching outputs, the fields count as additional arguments, which must
be matched.  If you want to call an overload with multiple output arguments,
but you don't need one of the outputs, you can use the `@hide` annotation to
ensure it's not used afterwards.  E.g., `[@hide X, Y]: my_overload()`.
You can also just not include it, e.g., `Y: my_overload()`, which is
preferred, in case the function has an optimization which doesn't need to
calculate `X`.

We also allow
[calling functions with any dynamically generated arguments](#dynamically-determining-arguments-for-a-function),
so that means being able to resolve the overload at run-time.

### pass-by-reference or pass-by-value

Functions can be defined with arguments that are passed-by-value using `.`, e.g., via
`Temporary_value. type_of_the_temporary`.  This argument type can be called with
temporaries, e.g., `fn(Arg_name. "my temp string")`, or with easily-copyable types
like `dbl` or `i32` like `My_i32: i32 = 5, fn(My_arg. My_i32)`, or with larger-allocation
types like `int` or `str` with an explicit copy or move: `My_str: "asdf..."` with
`fn(Tmp_arg. str(My_str))` or `fn(Tmp_arg. My_str!)`.  In any case, the passed-by-value
argument, if changed inside the function block, will have no effect on the
things outside the function block.  Inside the function block, pass-by-value
arguments are mutable, and can be reassigned or modified as desired.
Similar to Rust, variables that can be easily copied implement a `::copy(): me`
method, while variables that may require large allocations should only implement
`;;renew(@Other M): null` (essentially a C++ copy constructor).  This is done
by default for most oh-lang classes.

Functions can also be defined with writable or readonly reference arguments, e.g., via
`Mutable_argument; type_of_the_writeable` and `Readonly_argument: type_of_the_readonly` in the
arguments list, which are passed by reference.  This choice has three important
effects: (1) readonly variables may not be deeply constant (see section on
[passing by reference gotchas](#passing-by-reference-gotchas)), (2) you can modify
writable argument variables inside the function definition, and (3) any
modifications to a writable argument variable inside the function block persist
in the outer scope.  Note that pass-by-constant-reference arguments are the default,
so `fn(Int): null` is the same as `fn(Int: int): null`.

When passing by reference for `:` and `;` variables, we cannot automatically 
cast to the correct type.  Two exceptions: (1) if the variable is a temporary
we can cast like this, e.g., `My_value: 123` can be used for a `My_value: u8` argument,
and (2) child types are allowed to be passed by reference when the function asks
for a parent type.

Return types are never inferred as references, so one secondary difference between
`fn(Int.): ++Int` and `fn(Int;): ++Int` is that a copy/temporary is required
before calling the former and a copy is made for the return type in the latter.
The primary difference is that the latter will modify the passed-in variable in
the outer scope.  To avoid dangling references, any calls of `fn(Int;)` with a
temporary will actually create a hidden `int` before the function call.  E.g.,
`fn(Int; 12345)` will essentially become `Uniquely_named_int; 12345` then
`fn(Int; @hide Uniquely_named_int)`, so that `Uniquely_named_int` is hidden from the
rest of the block.  See also [lifetimes and closures](#lifetimes-and-closures).
To avoid a copy entirely, you'd need to explicitly annotate the return type;
e.g., you can use `fn(Int;): (Int;) {++Int}` for the above example, i.e.,
using [reference objects](#reference-objects) for the return value.

In C++ terms, arguments declared as `.` are passed as temporaries (`t &&`),
arguments declared as `:` are passed as constant reference (`const t &)`,
and arguments declared as `;` are passed as reference (`t &`).
Overloads can be defined for all three, since it is clear which is desired
based on the caller using `.`, `:`, or `;`.  Some examples:

```
# this function passes by value and won't modify the external variable
check(Arg123. string): string
    Arg123 += "-tmp"    # OK since Arg123 is defined as writable, implicit in `.`
    return Arg123

# this function passes by reference and will modify the external variable
check(Arg123; string): string
    Arg123 += "-writable"  # OK since Arg123 is defined as writable via `;`.
    return Arg123

# this function passes by constant reference and won't allow modifications
check(Arg123: string): string
    return Arg123 + "-readonly"

My_value; string = "great"
check(Arg123. My_value copy())  # returns "great-tmp".  needs `copy` since
                                # `.` requires a temporary.
print(My_value)                 # prints "great"
check(Arg123: My_value)         # returns "great-readonly"
print(My_value)                 # prints "great"
check(Arg123; My_value)         # returns "great-writable"
print(My_value)                 # prints "great-writable"
```

Note that if you try to call a function with a readonly reference argument,
but there is no overload defined for it, this will be an error.  Similarly
for writable-reference or temporary variable arguments.

```
only_readonly(A: int): str
    return str(A) * A

My_a; 10
only_readonly(A; My_a)      # COMPILE ERROR, no writable overload for `only_readonly(A;)`
only_readonly(A. int(My_a)) # COMPILE ERROR, no temporary overload for `only_readonly(A.)`

print(only_readonly(A: 3))      # OK, prints "333"
print(only_readonly(A: My_a))   # OK, prints "10101010101010101010"

only_mutable(B; int): str
    Result: str(B) * B
    B /= 2
    return Result

My_b; 10
only_mutable(B: My_b)           # COMPILE ERROR, no readonly overload for `only_mutable(B:)`
only_mutable(B. int(My_b))      # COMPILE ERROR, no temporary overload for `only_mutable(B.)`

print(only_mutable(B; My_b))    # OK, prints "10101010101010101010"
print(only_mutable(B; My_b))    # OK, prints "55555"

only_temporary(C. int): str
    Result; ""
    while C != 0
        Result append(str(C % 3))
        C /= 3
    Result reverse()
    Result

My_c; 5
only_temporary(C: My_c)     # COMPILE ERROR, no readonly overload for `only_temporary(C:)`
only_temporary(C; My_c)     # COMPILE ERROR, no temporary overload for `only_temporary(C;)`

print(only_temporary(C. 3))     # OK, prints "10"
print(only_temporary(C. My_c!)) # OK, prints "12"
```

Note there is an important distinction between variables defined as writable inside a block
versus inside a function argument list.  Mutable block variables are never reference types.
E.g., `B; A` is always a copy of `A`, so `B` is never a reference to the variable at `A`.
For a full example:

```
reference_this(A; int): int
    B; A  # B is a mutable copy of A.  if you want a reference, use `(B;) = A`
    A *= 2
    B *= 3
    return B

A; 10
print(reference_this(A;))   # prints 30, not 60.
print(A)                    # A is now 20, not 60.
```

You are allowed to have default parameters for reference arguments, and a suitable
block-scoped (but hidden) variable will be created for each function call so that
a reference type is allowed.

```
fn(B; int(3)): int
    B += 3
    return B

# This definition would have the same return value as the previous function:
fn(B?: int): int
    if B != Null 
        B += 3
        return B    # note that this will make a copy.
    else
        return 6

# and can be called with or without an argument:
print(fn())         # returns 6
My_b; 10
print(fn(B; My_b))   # returns 13
print(My_b)          # My_b is now 13 as well.
print(fn(B; 17))    # My_b is unchanged, prints 20
```

Note that a mooted variable will automatically be considered a temporary argument
unless otherwise specified.

```
over(Load: int): str
    return str(Load)

over(Load; int): str
    return str(Load++)

over(Load. int): str
    return str(++Load)

Load; 100
print(over(Load!))  # calls `over(Load.)` with a temporary, prints 101
print(Load)         # Load = 0 because it was mooted, and was not modified inside the function

Load = 100
print(over(Load: Load!))    # calls `over(Load)` with a const temporary, prints 100
print(Load)                 # Load = 0 because it was mooted

Load = 100
print(over(Load; Load!))    # calls `over(Load;)` with a temporary, prints 100
print(Load)                 # Load = 0 because it was mooted

# for reference, without mooting:
Load = 100
print(over(Load;))          # calls `over(Load;)` with the reference, prints 100
print(Load)                 # Load = 101 because it was passed by reference
```

The implementation in C++ might look something like this:

```
// these function overloads are defined for `fn(Str;)` and `fn(Str)`:
void fn(string &String);        // reference overload for `fn(Str;)`
void fn(string &&String);       // temporary overload for `fn(Str.)`
void fn(const string &String);  // constant reference just for `fn(Str:)`
```

Implementation detail: while `.` corresponds to finality as a sentence ender, and thus might
appear to relate most closely to a readonly type, we choose to use `:` as readonly and `;` as
writable references due to the similarity between `:` and `;`; therefore `.` can be the
odd-one-out corresponding to a non-reference, pass-by-value type.

### const templates

If you want to define multiple overloads, you can use the const template `;:` (or `:;`)
syntax for writable/readonly references.  There will be some annotation/macros which can be
used while before compiling, e.g., `@writable`/`@readonly` to determine if the variable is
writable or not.  Similarly, we can use const templates like `:;.` for
readonly-reference/writable-reference/temporary.  When we have the const template `:;.`,
use `respectively[a, b, c]` to get type `a`  for `:`, `b` for `;`, and `c` for `.`.
Similarly for the const template `:;`, `respectively[a, b]` will give `a` for `:` and
`b` when `;`.

```
my_class[of]: [X; of]
{   ;;take(Of.):
        X = Of!
    ;;take(Of:):
        X = Of

    # maybe something like this?
    ;;take(Of;:):
        X = @moot_or_clone(Of)
        # `@moot_or_clone(Z)` can expand to `@if @readonly(Z) {Z clone()} @else {Z!}`
}
```

Alternatively, we can rely on some boilerplate that the language will add for us, e.g.,

```
my_class[of]: [X; of]
{   # these are added automatically by the compiler since `X; of` is defined.
    ;;x(Of; of): { X<->Of }
    ;;x(Of: of): { X = Of }
    ;;x(Of. of): { X = Of! }

    # so `take` would become:
    ;;take(Of:;. of):
        x(Of:;!)
}
```

### passing by reference gotchas

For example, this function call passes `Array[3]` by reference, even if `Array[3]` is a primitive.

```
Array[int]; [0, 1, 2, 3, 4]
my_function(Array[3];)   # passed as writable reference
my_function(Array[3]:)   # passed as readonly reference
my_function(Array[3])    # also passed as readonly reference, it's the default.
```

You can switch to passing by value by using `.` or making an explicit copy:

```
Array[int]; [0, 1, 2, 3, 4]
my_function(int(Array[3]))   # passed by value (e.g., `my_function(Int.)`):
```

Normally this distinction of passing by reference or value does not matter except
for optimization considerations.  The abnormal situations where it does matter in
a more strange way is when the function modifies the outside variable
in a self-referential way.  While this is allowed, it's not recommended!  Here is
an example with an array:

```
Array; [0, 1, 2, 3, 4]
saw_off_branch(Int;): null
    Array erase(Int)
    Int *= 10

saw_off_branch(Array[3];)
print(Array)    # prints [0, 1, 2, 40]
# walking through the code:
# saw_off_branch(Array[3];):
#     Array erase(Array[3]) # Array erase(3) --> Array becomes [0, 1, 2, 4]
#     Array[3] *= 10        # reference to 4 *= 10  --> 40
```

Note that references to elements in a container are internally pointers to the container plus the ID/offset,
so that we don't delete the value at `Array[3]` and thus invalidate the reference `Array[3];` above.
Containers of containers (and further nesting) require ID arrays for the pointers.
E.g., `My_lot["Ginger"][1]["Soup"]` would be a struct which contains `&My_lot`, plus the tuple `("Ginger", 1, "Soup")`.

Here is an example with a lot.  Note that the argument is readonly, but that doesn't mean
the argument doesn't change, especially when we're doing self-referential logic like this.

```
Animals; ["hello": cat(), "world": snake(Name: "Woodsy")]

do_something(Animal): string
    Result; Animal Name
    Animals["world"] = cat()        # overwrites snake with cat
    Result += " ${Animal speak()}"
    return Result

print(do_something(Animals["world"]))    # returns "Woodsy hisss!" (snake name + cat speak)
```

Here is an example without a container, which is still surprising, because
the argument appears to be readonly.  Again, it's not recommended to write your code like this;
but these are edge cases that might pop up in a complex code base.

```
My_int; 123
not_actually_constant(Int): null
    print("Int before ${Int}")
    My_int += Int
    print("Int middle ${Int}")
    My_int += Int
    print("Int after ${Int}")
    # Int += 5  # this would be a compiler error since `Int` is readonly from this scope.

not_actually_constant(My_int) # prints "Int before 123", then "Int middle 246", then "Int after 492"
```

Because of this, one should be careful about assuming that a readonly argument is deeply constant;
it may only be not-writable from your scope's reference to the variable.

In cases where we know the function won't do self-referential logic,
we can try to optimize and pass by value automatically.  However, we
do want to support closures like `next_generator(Int; int): do(): ++Int`,
which returns a function which increments the passed-in, referenced integer,
so we can never pass a temporary argument (e.g., `Arg. str`) into `next_generator`.

### destructuring

If the return type from a function has multiple fields, we can grab them
using the notation `[Field1:, Field2;, Field3] = do_stuff()`, where `do_stuff` has
a function signature of `fn(): [Field1: field1, Field2: field2, Field3: field3, ...]`,
and `...` are optional ignored return fields.  In the example, we're declaring
`Field1` as readonly, `Field2` as writable, and `Field3` is an existing variable
that we're updating (which should be writeable), but any combination of `;`, `:`,
and `=` are possible.  The standard case, however, is to declare (or reassign) all
variables at the same time, which can be done with `[Field1, Field2]: do_stuff()`
(`[Field1, Field2]; do_stuff()`) for readonly (writeable) declaration + assignment,
or `[Field1, Field2] = do_stuff()` for reassignment.

If the returned fields are references (and we don't want to copy them into local variables),
we can use parentheses in an analogous way:  `(Ref1:, Ref2;, Not_a_ref.) = do_stuff()`
to define `Ref1` as a readonly reference, `Ref2` as a writeable reference, and
`Not_a_ref` as a unique, writable instance.  Or if we want to do all fields the same,
`(Ref1, Ref2): do_stuff()` works to define `Ref1` and `Ref2` as readonly references, or
`(Ref1, Ref2); do_stuff()` works to define them as writable references, and
`(Ref1, Ref2). do_stuff()` works to define them as unique, writable instances.
Notice that the only distinction between destructuring references and defining a function
is that the function requires a `function_case` identifier before the parentheses
(e.g., `fn(): ...` or `do_stuff(Ref1, Ref2); ...`).

You can also use destructuring to specify return types explicitly.
The notation is `[Field1: type1, Field2; type2] = do_stuff()`.  This can be used
to grab even a single field and explicitly type it, e.g., `[X: str] = whatever()`,
although via [SFO](#single-field-objects) this is the same as `X: str = whatever()`.

This notation is a bit more flexible than JavaScript, since we're
allowed to reassign existing variables while destructuring.  In JavaScript,
`/*js*/ const {Field1, Field2} = do_stuff();` declares and defines the fields `Field1` and `Field2`,
but `/*js??*/ {Field1, Field2} = do_stuff();`, i.e., reassignment in oh-lang, is an error in JS.

Some worked examples follow, including field renaming.

```
fraction(In: string, Io; dbl): [Round_down: int, Round_up: int]
    print(In)
    Round_down: Io round(Down)
    Round_up: Io round(Up)
    Io -= Round_down
    [Round_down, Round_up]

# destructuring
Io; 1.234
[Round_down]: fraction(In: "hello", Io;)

# === calling the function with variable renaming ===
Greeting: "hello!"
Input_output; 1.234      # note `;` so it's writable.
# just like when we define an argument for a function, the newly scoped variable goes on the left,
# so too for destructuring return arguments.  this one uses the default type of `Round_down`:
[Integer_part; round_down] = fraction(In: Greeting, Io; Input_output)

# here's an example without destructuring.
Io; 1.234
Result: fraction(In: "hello", Io;)
# `Result` is an object with these fields:
print(Result Round_down, Result Round_up)
```

TODO:
Note that we're not allowed to cast... or are we?  we want to be able to easily convert
an iterator into a list, for example.

```
countdown(Count): all_of[iterator[count], m: [Count]]
{   ::next()?: count
        if M Count > 0
            --M Count
        else
            Null
}

My_array: array[count] = countdown(5)
```

Note, you can also have nullable output arguments.  These will be discussed
more in the function overload section, but here are some examples.

```
# standard definition:
wow(Lives: int)?: cat
    if Lives == 9
        cat()
    else
        Null
```

For nested object return types, there is some syntactic sugar for dealing with them.
Note, however, that nested fields won't help the compiler determine the function overload.

```
nest(X: int, Y: str): [W: [Z: [A: int], B: str, C: str]]
    [W: [Z: [A: X], B: Y, C: Y * X]]

# defines `A`, `B`, and `C` in the outside scope:
[W: Z: A, W: B, W: C] = nest(X: 5, Y: "hi")
print(A)    # 5
print(B)    # "hi"
```

### single field objects

Single field objects (SFOs) are used to make it more concise to return a
type into a variable with a given name; the output variable name can help
determine which overload will be called.  Consider the following overloads.

```
patterns(): [Chaos: f32, Order: i32]
patterns(): i32
# overload when we don't need to calculate `Order`:
patterns(): Chaos: f32      # equivalent to `patterns(): [Chaos: f32]`

I32: patterns()             # calls `patterns(): i32` overload
My_value: patterns() I32    # same, but defining `My_value` via the `i32` return value.
@Namespace I32: patterns()  # same, defining `@Namespace I32` via the `i32`.
I32 as Q: patterns()        # same, defining `Q` via the `i32`.

F32: patterns()             # COMPILE ERROR: no overload for `patterns(): f32`

[Chaos]: patterns()         # calls `patterns(): [Chaos: f32]` overload via destructuring.
Chaos: patterns()           # same, via SFO concision.
My_value: patterns() Chaos  # same, but with renaming `Chaos` to `My_value`. 
Chaos as Cool: patterns()   # same, but with renaming `Chaos` to `Cool`.
[Wow; chaos] = patterns()   # same, but with renaming `Chaos` to `Wow`.

Result: patterns()          # calls `patterns(): [Chaos: f32, Order: i32]`
                            # because it is the default (first defined).
[Chaos, Order]: patterns()  # same overload, but because of destructuring.
[Order]: patterns()         # same, but will silently drop the `Chaos` return value.
Order: patterns()           # more concise form of `[Order]: patterns()`.
My_value: patterns() Order  # same, but with renaming `Order` to `My_value`.
Order as U: patterns()      # same, with renaming `Order` to `U`.
[T; order] = patterns()     # same, with renaming `Order` to `T`.
```

The effect of SFO is to make it possible to elide `[]` when asking for a single named output.
The danger is that your overload may change based on your return variable
name; but this is usually desired, e.g., `Old Count: Array count(1000) assert()`
if you care to get the old count of an array.

IMPLEMENTATION NOTE: `X: ... assert()` will require inferring through
the `[X: x]` return value through the result `hm[ok: [X: x], ...]`
via `assert()`.  This may be difficult for more complicated expressions.

SFO effectively makes any `x` return type into a `[X: x]` object.  This means
that overloads like `patterns(): i32` and `patterns(): [I32]` would actually
conflict; trying to define both would be a compile error.

TODO: we probably can have `x(@New X: x): null` overloads where we don't need
to always swap out the old value (e.g., `x(@New X: x): x`.
TODO: we should make it clear by requiring setters to return the old value only
if `x(@New X: x): [@Old X]` is used.  or just use `;;x(X; x): null` as the
swapper signature and `;;x(X. x): null` as the setter, and don't specify
what `;;x(X; x): x` would mean.

### `arguments` class

Variadic functions are possible in oh-lang using the `arguments[of]` class.
We recommend only one class type, e.g., `arguments[int]` for a variable number
of integers, but you can allow multiple classes via e.g. `arguments[one_of[x, y, z]]`
for classes `x`, `y`, and `z`.  oh-lang disallows mixing `arguments` with any
other arguments.  The `arguments` class has methods like `::count()`, `;:[Index]`,
and `;;[Index]!`.  Thus, `arguments` is effectively a fixed-length array, but you
can modify the contents if you pass as `Arguments[type];`.  It is guaranteed that
there is at least one argument, so `Arguments[0]` is always defined.

```
max(Arguments[int]): int
    Max; Arguments[0]
    range(1, Arguments count()) each Index:
        if Arguments[Index] > Max
            Max = Arguments[Index]
    Max  
```

### dynamically determining arguments for a function

We allow for dynamically setting arguments to a function by using the `call` type,
which has a few fields: `Input` and `Output` for the arguments and return values
of the function, as well as optional `Info`, `Warning`, and `Error` fields for any issues
encountered when calling the function.  These fields are named to imply that the function
call can do just about anything (including fetching data from a remote server).

```
call:
[   Input; lot[at: str, reference[any]]
    # we need to distinguish between the caller asking for specific fields
    # versus asking for the whole output.
    Output; lot[at: str, any]
    # things printed to stdout via `print`:
    Print; array[string]
    # things printed to stderr via `error`:
    Error; array[string]
]
{   # adds an argument to the function call.
    # e.g., `Call input(Name: "Cave", Value: "Story")`
    ;;input(Name: str, Value: reference[any]): null
        Input[Name] = Value

    # adds an argument to the function call.
    # e.g., `Call input(Cave: "Story")`
    ;;input(~Name: reference[any]): null
        Input[@@Name] = Name

    # adds a single-value return type
    ;;output(Any): null
        assert(Output count() == 0)
        # TODO: a better way to refer to the class name.
        # can we just do `Any Class_name`?
        Output[Any is() Class_name] = Any

    # adds a field to the return type with a default value.
    # e.g., `Call output(Field_name: 123)` will ensure
    # `{Field_name}` is defined in the return value, with a
    # default of 123 if `Field_name` is not set in the function.
    ;;output(~Name: any):
        output(Name: @@Name, Value: Name)

    # adds a field to the return type with a default value.
    # e.g., `Call output(Name: "Field_name", Value: 123)` will ensure
    # `{Field_name}` is defined in the return value, with a
    # default of 123 if `Field_name` is not set in the function.
    ;;output(Name: string, Value: any): null
        Output[Name] = Value 
}
reference[of]: one_of[writable: (Of;), readonly: (Of:)]
```

When passing in a `call` instance to actually call a function, the `Input` field will be treated
as constant/read-only.  The `Output` field will be considered "write-only", except for the fact
that we'll read what fields are defined in `Output` (if any) to determine which overload to use.
This call structure allows you to define "default values" for the output, which won't get
overwritten if the function doesn't write to them.  To make things easier to reason about, you
can't influence the function overload by requesting nested fields in the output (e.g., `[X: [Y: Z]]`);
only fields directly attached to the `Output` (e.g., `[X, T]`) can influence the function overload.

Let's try an example:

```
# define some function to call:
some_function(X: int): string
    return "hi" * X
# second overload:
some_function(X: string): int
    return X count_bytes()

My_string: string = some_function(X: 100)   # uses the first overload
My_int: int = some_function(X: "cow")       # uses the second overload
Check_type1: some_function(X: 123)          # uses the first overload since the type is `int`
Check_type2: some_function(X: "asdf")       # uses the second overload since the type is `string`
Invalid: some_function(X: 123.4)            # COMPILE ERROR: 123.4 is not referenceable as `int` or `string`

# example which will use the default overload:
Call; call
Call input(X: 2)
# use `Call` with `;` so that `Call;;Output` can be updated.
some_function(;Call)
print(Call Output)  # prints "hihi"

# define a value for the object's Output field to get the other overload:
Call; call
Call input(X: "hello")
some_function(;Call)
print(Call Output)  # prints 5

# dynamically determine the function overload:
Call; call
if some_condition()
    Call {input(X: 5), output("?")}
else
    Call {input(X: "hey"), output(-1)}

some_function(;Call)
print(Call Output)  # will print "hihihihihi" or 3 depending on `some_condition()`.
```

Note that `call` is so generic that you can put any fields that won't actually
be used in the function call.  In this, oh-lang will return an error at run-time.

```
Call; call() { input(X: "4"), output(Value1: 123), output(Value2: 456) }
some_function(;Call) assert()    # returns error since there are no overloads with [Value1, Value2]
```

If compile-time checks are desired, one should use the more specific
`my_fn call` type, with `my_fn` the function you want arguments checked against.

```
# throws a compile-time error:
Call; some_function call() { input(X: "4"), output(Value1: 123), output(Value2: 456) }
# the above will throw a compile-time error, since two unexpected fields are defined for Output.

# this is ok (calls first overload):
Call2; some_function call() { input(X: "4"), output(0) }
# also ok (calls second overload):
Call3; some_function call() { input(X: 4), output("") }
```

Note that it's also not allowed to define an overload for the `call` type yourself.
This will give a compile error, e.g.:

```
# COMPILE ERROR!!  you cannot define a function overload with a default-named `call` argument!
some_function(;Call): null
    print(Call Input["X"])
```

This is because all overloads need to be representable by `Call; call`, including any
overloads you would create with `call`.  Instead, you can create an overload with a
`call` argument that is not default-named.

```
some_function(My_call; call): null
    print(Call Input["X"]) # OK
```

### callable

Taking the `call` idea one step further, we can have a pointer to the function
already ready so all we need to do is run `Callable call()`.  To specify the overload,
the input and output variables need to be named inside the `Callable`.


## mutable functions

To declare a reassignable function, use `;` after the arguments.

```
greet(Noun: string); null
    print("Hello, ${Noun}!")

# you can use the function:
greet(Noun: "World")

# or you can redefine it:
greet(Noun: string); null
    print("Overwriting!")
# it's not ok if we use `greet(Noun: string): null` when redefining, since that looks like
# we're switching from writable to readonly.
```

It needs to be clear what function overload is being redefined (i.e., having
the same function signature), otherwise you're just creating a new overload
(and not redefining the function).  You can also assign an existing function
to a mutable function using notation like this:
`my_mutable_fn(); int {original_definition()}`, followed by
`my_mutable_fn(); int = some_other_fn`.  Because the overload on `my_mutable_fn`
is fully specified (e.g., no input arguments, an `int` return value), we know
to select that overload on `some_other_fn`.

## nullable functions

The syntax for declaring a nullable/optional function is to put a `?` after the function name
but before the argument list.  E.g., `optional_function?(...Args): return_type` for a non-reassignable
function and swapping `:` for `;` to create a reassignable function.
When calling a nullable function, unless the function is explicitly checked for non-null,
the return type will be nullable.  E.g., `X?: optional_function(...Args)` will have a
type of `one_of[return_type, null]`.  Nullable functions are checked by the executable, so the
programmer doesn't necessarily have to do it.

A nullable function has `?` before the argument list; a `?` after the argument list
means that the return type is nullable.  The possible combinations are therefore the following:

* `normal_function(...Args): return_type` is a non-null function
  returning a non-null `return_type` instance.

* `nullable_function?(...Args): return_type` is a nullable function,
  which, if non-null, will return a non-null `return_type` instance.
  Conversely, if `nullable_function` is null, trying to call it will return null.

* `nullable_return_function(...Args)?: return_type` is a non-null function
  which can return a nullable instance of `return_type`.

* `super_null_function?(...Args)?: return_type` is a nullable function
  which can return a null `return_type` instance, even if the function is non-null.
  I.e., if `super_null_function` is null, trying to call it will return null,
  but even if it's not null `super_null_function` can still return null.

```
# creating an optional function in a class:
example: [X: dbl, optional_fn?(M, Z: dbl); int]

Example; example(X: 5)

# define your own function for optional_fn:
Example::optional_fn(Z: dbl); int
    # need the namespace `M X` here because `X` is not obviously in scope.
    return floor(Z * M X)

# or set it to null (would set all overloads to null):
Example optional_fn = null

# the entire overload must be specified
# if you want to delete just one overload.
# (i.e., in case there are multiple overloads)
Example optional_fn?(Z: dbl); int = null

# after setting it to null...
Example optional_fn(Z: 3.21)    # returns Null
```

## generic/template functions

We can have arguments with generic types, but we can also have arguments
with generic names.

### argument type generics

For functions that accept multiple types as input/output, we define template types
inline, e.g., `copy(Value: ~t): t`, using `~` for where the compiler should infer
what the type is.  You can use any unused identifier for the new type, e.g.,
`~q` or `~sandwich_type`.

```
copy(Value: ~t): t
    print("got $(Value)")
    return Value

vector3: [X: dbl, Y: dbl, Z: dbl]
Vector3: vector3(Y: 5)
Result: copy(Value: Vector3)    # prints "got vector3(X: 0, Y: 5, Z: 0)".
Vector3 == Result           # equals True
```

You can also add the new types in brackets just after the function name,
e.g., `copy[t: my_type_constraints](Value: ~t): t`, which allows you to specify any
type constraints (`my_type_constraints` being optional).  Note that types defined with
`~` are inferred and therefore can never be explicitly given inside of the brackets,
e.g., `copy[t: int](Value: 3)` is invalid here, but `copy(Value: 3)` is fine.

If you want to require explicitly providing the type in brackets, don't use `~` when
defining the function.

```
# this generic function does not infer any types because it doesn't use `~`.
copy[the_type](Value: the_type): the_type
    ...
    the_type(Value)

# therefore we need to specify the generics in brackets before calling the function.
copy[the_type: int](Value: 1234)    # returns 1234
```

For this example, it would be better to use `of` instead of `the_type`, since `of`
is the "default name" for a generic type.  E.g., you don't need to specify `[of: int]`
to specialize to `int`, you can just use `[int]` for an `[of]`-defined generic.
See also [default named generic types](#default-named-generic-types).  For example:

```
# this generic function does not infer any types because it doesn't use `~`.
copy[of](Value: of): of
    ...
    of(Value)

# because the type is not inferred, you always need to specify it in brackets.
# you can use `of: the_type` but this is not idiomatic:
copy[of: int](Value: 3) # will return the integer `3`

# because it is default named, you can just put in the type without a field name.
copy[dbl](Value: 3)     # will return `3.0`
```

### default-named generic arguments

TODO: restrictions here, do we need to only have a single argument, so that
argument names are unique?  it's probably ok if we have an `@order_independent`
or use `@First ~T` and `@Second ~U` to indicate order is ok.
or need to use `@Named` on some of them.
maybe we see if there's an issue when compiling the generics and then complain at compile time.

Similar to the non-generic case, if the `Variable_case` identifier
matches the `type_case` type of a generic, then it's a default-named argument.
For example, `My_type; ~my_type` or `T: ~t`.  There is a shorthand for this
which is more idiomatic: `~My_type;` or `~T`.  Here is a complete example:

```
logger(~T): t
    print("got ${T}")
    return T

vector3: [X: dbl, Y: dbl, Z: dbl]
Vector3: vector3(Y: 5)
Result: logger(Vector3)     # prints "got vector3(X: 0, Y: 5, Z: 0)".
Vector3 == Result           # equals True

# implicit type request:
Int_result: logger(5)        # prints "got 5" and returns the integer 5.

# explicit type request:
Dbl_result: logger(dbl(4))   # prints "got 4.0" and returns 4.0
```

Note that you can use `my_function(~T;)` for a writable argument.
Default naming also works if we specify the generics ahead of the function arguments
like this:

```
logger[of: some_constraint](Of): of
    print("got ${Of}")
    Of

# need to explicitly add the type since it's never inferred.
logger[int](3)  # returns the integer `3`
logger[dbl](3)  # will return `3.0`
```

If we have a named generic type, just name the `type_case` type the
same as the `Variable_case` variable name (besides initial capitalization)
so default names can apply.

```
logger[value](Value): value
    print("got ${Value}")
    Value

logger[value: dbl](3)  # will return `3.0` and print "got 3.0"
```

If we want to suppress default naming, e.g., require the function argument 
to be `Value: XYZ`, then we need to explicitly tell the compiler that we don't
want default names to apply, which we do using the `@Named` namespace.

```
logger(@Named ~Value): value
    print("got ${@Named Value}")
    @Named Value

# it can be called like this, which implicitly infers the `value` type:
logger(Value: 3)  # returns the integer `3`
```

And as in other contexts, you can avoid inferring the type by avoiding using `~`.

```
# this generic needs to be specified in brackets at the call site: 
logger[value](@Named Value): value
    ...
    value(@Named Value)

# and because it's not default named (i.e., it's named `value` not `of`),
# you need to call it like this:
logger[value: dbl](Value: 3)  # will return `3.0`
```


### argument name generics: with different type

You can also define an argument with a known type, but an unknown name.
This is useful if you want to use the inputted variable name at the call site
for logic inside the function, e.g., `this_function(I_want_to_know_this_variable_name: 5)`.
You can access the variable name via `@@`.

TODO: internally this creates an overload with a "The_name_value" int argument
and "The_name_name" string argument.  do we really want to support this?
functionally this is how we support creating lots like `My_lot: [Whatever: "dude"]`,
so if it's something the compiler is able to do, it's something users should be able to.

```
this_function(~Argument: int): null
    Argument_name: str(@@Argument)
    print("calling this_function with ${Argument_name}: ${Argument}")
```

We cannot define an argument name and an argument type to both be
generic and different.  `my_function(~My_name: ~another_type)` (COMPILE ERROR)
is needlessly verbose; if the type should be generic, just rely on what
is passed in: `my_function(~My_name: my_name)` or `my_function(~My_name) for short.

### require

`Require` is a special generic field that allows you to include a function,
method, or variable only if it meets some compile-time constraints.  It is
effectively a keyword within a generic specification, so it can't be used
for other purposes, and the boolean value it takes must be known at compile-time.

```
my_class[of, N: count]:
[   Value: of
    # this field `Second_value` is only present if `N` is 2 or more.
    # TODO: does this conflict with any other usages of generic classes?
    # if so, let's switch to `@require(N >= 2) Second_value: of`
    Second_value[Require: N >= 2]: of
    # this field `Third_value` is only present if `N` is 3 or more.
    Third_value[Require: N >= 3]: of
    ... # plz help am i coding this right??     (no, prefer `vector[N, of]`)
]
{   # `of is hashable` is true iff `of` extends `hashable` either explicitly
    # or implicitly by implementing a `hash` method like this:
    ::hash[Require: of is hashable](~Builder):
        Builder hash(Value)
        @if N > 1 {Builder hash(Second_value)}
        @if N > 2 {Builder hash(Third_value)}
        # TODO: maybe add something like `Builder hash(@?Second_value)`
        ...
}
```

TODO: should we make this an annotation instead?  `@require(of is orderable)`??
in C++, these sorts of things are templates, but that can be kinda confusing.
but it does allow you to do things like this, where you introduce new types
on the fly and can require them to be a certain way:
`::do[additional_type, Require: additional_type is foo](~Additional_type): int`
so if possible, i think i'd prefer to keep it as a template.

# classes

A class is defined with a `type_case` identifier, an object `[...]`
defining instance variables and instance functions (i.e., variables and
functions that are defined *per-instance* and take up memory), and an
optional indented block (optionally in `{}`) that includes methods and functions that are
shared across all instances: class instance methods (just methods for short)
and class functions (i.e., static methods in C++/Java) that don't require an instance.
Class definitions must be constant/non-reassignable, so they are declared using
the `:` symbol.

When defining methods or functions of all kinds, note that you can use `m`
to refer to the current class instance type.  E.g.,

```
# classes can enclose their body in `{}`, which is recommended for long class definitions.
# for short classes, it's ok to leave braces out.
my_class: [Variable_x: int]
    ::copy(): me    # OK
        print("logging a copy")
        return m(M)   # or fancier copy logic
```

Inside the class body, you can use `M` to refer to any other instance variables or methods,
e.g., `M X` or `M do_stuff_method()`, or you can just use them directly as `X` and `do_stuff_method()`.
Any collisions with global variables/functions will be reported with a compile error.

Note that when returning a newly-declared type from a function (e.g., `my_fn(Int): [X: int, Y: dbl]`),
we do not allow building out the class body; any indented block will be assumed
to be a part of the function body/definition:

```
my_fn(Int): [X: int, Y: dbl]
{   # this is part of the `my_fn` definition,
    # and never a part of the `[X: int, Y: dbl]` class body.
    return [X: 5, Y: 3.0]
}
```

If you want to specify methods on a return type, make sure to build it out as a separate
class first.

```
x_and_y: [X: int, Y: dbl]
{   ::my_method(): X + round(Y) Int
}

my_fn(Int): x_and_y
    [X: Int + 5, Y: 3.0]
```

## example class definition

```
parent_class: [Name: str]

# example class definition
example_class: all_of
[   parent_class
    m:
    [   # given the inheritance with `parent_class`,
        # child instance variables should be defined in this `m: [...]` block.
        # if they are public, a public constructor like `example_class(X;:. int)` will be created.
        X; int

        # instance functions can also be defined here.  they can be set 
        # individually for each class instance, unlike a class function/method
        # which is shared.
        # we define a default for this function but you could change its definition in a constructor.
        # NOTE: instance functions can use `M` as necessary.
        #       even though we could use the notation `::instance_function()` here,
        #       we prefer to keep that for methods, to make it more clear that
        #       this is different in principle.
        instance_function(M): null
            print("hello ${M X}!")

        # this class instance function can be changed after the instance has been created
        # (due to being declared with `;`), as long as the instance is mutable.
        some_mutable_function(); null
            print("hello!")
    ]
]
{   # classes must be resettable to a blank state, or to whatever is specified
    # as the starting value based on a `renew` function.  this is true even
    # if the class instance variables are defined as readonly.
    # NOTE:  defining this method isn't necessary since we already would have had
    # `example_class(X: int)` based on the public variable definition of `X`, but
    # we include it as an example in case you want to do extra work in the constructor
    # (although avoid doing work if possible).
    ;;renew(@New X. int): null
        Parent_class renew(Name: "Example")
        X = @New X!
    # or short-hand: `;;renew(M X. int, Parent_class Name: "Example"): {}`
    # adding `M` to the arg name will automatically set `M X` to the passed in `X`.

    # create a different constructor.  constructors use the class reference `m` and must
    # return either an `m` or a `hm[ok: m, er]` for any error type `er`.
    # this constructor returns `m`:
    # TODO: why did we switch to requiring `{}` in functions everywhere?  it doesn't help
    # distinguish reference-object destructuring from function declarations (need a function name).
    # probably can require it for lambda functions but not necessarily named functions like
    # `my_fn(Int): ++Int`
    m(K: int): m(X: K * 1000)

    # some more examples of class methods:
    # prefix `::` (`;;`) is shorthand for adding `M: m` (`M; m`) as an argument.
    # this one does not change the underlying instance:
    ::do_something(Int): int
        print("My name is ${M Name}")   # `M Name` will check child first, but also look in parents.
        # also ok, if we know it's definitely in `parent_class`:
        print("My name is ${Parent_class Name}")
        X + Int     # equivalent to `M X + Int`

    # this method mutates the class instance, so it uses `M; m` instead of `M:`:
    ;;add_something(Int): null
        X += Int    # equivalent to `M X += Int`

    # COMPILE ERROR: reassignable methods are currently not supported;
    # they may be in the future but would require hotswapping functions.
    # in case someone is running an old function, we need to let them
    # finish before reclaiming the memory of that function.
    ::reassignable_method(Int); string
        string(X + Int)

    # some examples of class functions:
    # this function does not require an instance, and cannot use instance variables:
    some_static_function(Y; int): int
        Y /= 2
        return Y!

    # this function does not require an instance, and cannot use instance variables,
    # but it can read/write global variables (or other files):
    some_static_function(Y: int): null
        write(Y, File: "Y")
}

Example; example_class(X: 5)
print(Example do_something(7))   # should print 12
Example = example_class(X: 7)    # note: variable can be reassigned.
Example X -= 3                  # internal fields can be reassigned as well.

# note that if you define an instance of the class as readonly, you can only operate
# on the class with functions that do not mutate it.
Const_var: example_class(X: 2)
Const_var X += 3                # COMPILER ERROR! `Const_var` is readonly.
Const_var = example_class(X: 4) # COMPILER ERROR! variable is readonly.

# calling class functions doesn't require an instance.
Dont_need_an_instance: example_class some_static_function(Y; 5)
```

## declaring methods and class functions outside of the class

You can also define your own custom methods/functions on a class outside of the class body.
Note that we do not allow adding instance functions or instance variables outside
of the class definition, as that would change the memory footprint of each class instance.
You can also use [sequence building](#sequence-building) outside of the class to define
a few methods, but don't use `:` since we're no longer declaring the class.

```
# static function that constructs a type or errors out
example_class(Z: dbl): hm[ok: example_class, er: str]
    X: Z round() int() assert(Er: "Need `round(Z)` representable as an `int`.")
    example_class(X)

# static function that is not a constructor.
# this function does not require an instance, and cannot use instance variables,
# but it can read (but not write) global variables (or other files):
example_class some_static_function(): int
    Y_string: read(File: "Y")
    return int(?Y_string) ?? 7

# a method which can mutate the class instance:
# this could also be defined as `example_class another_method(M;, Plus_k: int): null`.
example_class;;another_method(Plus_k: int): null
    # outside of a class body, `M` is required to namespace any instance fields,
    # because they are not obviously in scope here like in a class body.
    M X += Plus_k * 1000

# Use sequence building.
example_class@
{   # with sequence building, `example_class my_added_class_function(K: int): example_class`
    # is exactly how you'd define a class function.
    my_added_class_function(K: int): example_class
        example_class(X: K * 1000)

    # a method which keeps the instance readonly:
    ::my_added_method(Y: int): int
        M X * 1000 + Y * 100
}
```

If they are public, you can import these custom methods/functions in other files in two
ways: (1) import the full module via `[*]: \/relative/path/to/file` or `[*]: \\library/module`,
or (2) import the specific method/function via e.g.,
`{example_class my_added_class_function(K: int): example_class} \/relative/path/to/file`
or `{example_class::my_added_method(Y: int): int} \\library/module`.

Note that we recommend using named fields for constructors rather than static
class functions to create new instances of the class.  This is because named fields
are self descriptive and don't require named static functions for readability.
E.g., instead of `My_date: date_class from_iso_string("2020-05-04")`, just use
`My_date: date_class(Iso_string: "2020-05-04")` and define the
`;;renew(Iso_string: string)` method accordingly.

## destructors

The `;;renew(Args...): null` (or `: hm[ok: me, er: ...]`) constructors
are technically resetters.  If you have a custom destructor, i.e., code
that needs to run when your class goes out of scope, you shouldn't define
`;;renew` but instead `m(Args...): m` and `;;descope(): null`.
It will be a compile error if you try to define both `m` and `;;renew`
with the same arguments.

```
destructor_class: [X: int]
{   @protected
    # TODO: the `@Debug` annotation should do something interesting, like
    #       stop the debugger when the value is `set`ted or `get`ted.
    m(@Debug X. int): m
        print("X ${@Debug X}")
        [X. @Debug X]
    # `m(...): m` will also add methods like this:
    #   ;;renew(@Debug X. int): null
    #       # this will call `M descope()` just before reassignment.
    #       M = m(.@Debug X)

    # you should define the destructor:
    ;;descope(): null
        print("going out of scope, had X ${X}")
        # note that destructors of instance variables (e.g., `X`)
        # will automatically be called, in reverse order of definition.
}
```

Destructors are called before instance variables are descoped.
Child class destructors only need to clean up their own instance variables;
they will be called before the parent class destructor (which will automatically
be called).

## instance functions, class functions, and methods

Class methods can access instance variables and call other class methods,
and require a `M: m` argument to indicate that it's an instance method.
Mutating methods -- i.e., that modify the class instance, `M`, i.e., by modifying
its values/variables -- must be defined with `M;` in the arguments.
Non-mutating methods must be defined with `M:` and can access variables but not modify them.
Methods defined with `M.` indicate that the instance is temporary.  We'll use
the shorthand notation `Some_class..temporary_method()` to refer to a temporary instance method,
`Some_class;;some_mutating_method()` to refer to a mutable instance method,
and `Some_class::some_method()` to refer to a readonly instance method, with an implicit `M` due
to the class instance `Some_class` being present.  Calling a class method does not require
the `..`, `;;`, or `::` prefix, but it is allowed, e.g.,

```
Some_class; some_class("hello!")
Some_class some_method()      # ok
Some_class::some_method()     # also ok
Some_class some_mutating_method()  # ok
Some_class;;some_mutating_method() # also ok
# you can get a temporary by using moot (!):
My_result1: Some_class!..temporary_method()
# or you can get a temporary by creating a new class instance:
My_result2: some_class("temporary")..temporary_method()
```

Note that you can overload a class method with readonly instance `::`, writable
instance `;;`, and temporary instance `..` versions.  If it's unclear,
callers are recommended to be explicit and use `::`, `;;`, or `..` instead of ` ` (member access).
See the section on member access operators for how resolution of ` ` works in this case.
You can also call a class method via an inverted syntax, e.g., `some_method(Some_class)`,
`some_mutating_method(Some_class;)`, or `temporary_method(Some_class!)`,
with any other arguments to the method added as well.
This is useful to overload e.g., the printing of your class instance, via defining
`print(M)` as a method, so that `print(Some_class)` will then call `Some_class::print()`.
Similarly, you can do `count(Some_class)` if `Some_class` has a `count(M)` method, which
all container classes have.  This also should work for multiple argument methods, since
`Array swap(Index1, Index2)` can easily become `swap(Array;, Index1, Index2)`.

And of course, class methods can also be overridden by child classes (see section on overrides).

Class functions can't depend on the instance, i.e., `M`.  They can
be called from the class name, e.g., `x my_class_function()`, or
from an instance of the class, e.g., `X my_class_function()`.  Note that because of this,
we're not allowed to define class functions with the same overload as instance methods.
Similar to class functions are class variables, which are defined in an analogous way.

Instance functions are declared like instance variables, inside the `[...]` block.
Instance functions can be different from instance to instance.
They cannot be overridden by child classes but they can be overwritten.  [Oprah meme]
I.e., if a child class defines the instance function of a parent class, it overwrites the parent's
instance function; calling one calls the other.

Class constructors can be defined in two ways, either as a method or as a class function.
Class *method* constructors are defined with the function signature (a) `;;renew(Args...): null`
or (b) `;;renew(Args...): hm[ok: null, er: ...]`, and these methods also allow you to renew an
existing class instance as long as the variable is writable.  Class *function* constructors
are defined like (c) `m(Args...): m` or (d) `m(Args...): hm[ok: m, er: ...]`.  In both
(a) and (c) cases, you can use them like `My_val: my_class(Args...)`, and for (b) and (d)
you use them like `My_var: my_class(Args...) assert()`.

The first constructor defined in the class is also the default constructor,
which will be called with default-constructed arguments (if any) if a default
instance of the class is needed.  It is a compiler error if a constructor with
zero arguments is defined after another constructor with arguments.

## localization support

We intend oh-lang to support all languages, and so the upper/lower-case requirements
may seem a bit strange in other alphabets.  To set a custom `Variable_case` default name
for an instance of the class, use this syntax:

```
örsted: [...]
{   # define a custom Upper_camel_case name.
    M: Örsted 
    # probably could also parse `Örsted: (M)` as well here.

    ... usual class methods ...
}

# Now we can use `Örsted` to mean a default-named variable of the class `örsted`:
do_something(Örsted): bool
    return ...
```

We will throw a compile error when a class begins with a non-ascii letter, unless
the class defines the default-name of a variable of the class.  Also, you'll get
a compile error unless the custom default name is the first statement in the class
definition.

## public/private/protected visibility

We use annotations `@public`, `@private`, and `@protected` to indicate various
levels of access to a class instance's variables.  Public is default, and means
that the variable can be both accessed and modified by anyone.  Protected means
that the variable can be accessed and modified by friends, but for non-friends
the variable can only be accessed.  Friendship is defined by being in the same
directory as the module in question; i.e., being in the same file or being in
neighboring files in the same filesystem folder.  Private means the variable
can be accessed by friends only, and modified only by functions in the same module; i.e.,
the class (or other instances of the class) *or other functions in the same
file as the class definition*.  Non-friends are not able to access or modify private variables.

|  variable access  |  public   | protected |  private  |
|:-----------------:|:---------:|:---------:|:---------:|
|   module access   |   yes     |   yes     |   yes     |
|   module mutate   |   yes     |   yes     |   yes     |
|   friend access   |   yes     |   yes     |   yes     |
|   friend mutate   |   yes     |   yes     |   no      |
| non-friend access |   yes     |   yes     |   no      |
| non-friend mutate |   yes     |   no      |   no      |

The privacy for methods on a class follows the same table.
Using the method depends on visibility as well
as if the method modifies the class or not, i.e., whether the method was
defined as `mutating_method(M;): return_type` (AKA `;;mutating_method(): return_type`)
or `non_mutating_method(M): return_type` (AKA `::non_mutating_method(): return_type`).
Mutating methods follow the "mutate" visibility in the table above, and non-mutating methods follow
the "access" visibility in the table above.

To put into words -- `@public` methods can be called by anyone, regardless
of whether the method modifies the class instance or not.  `@protected`
methods which modify the class instance cannot be called by non-friends,
but constant `@protected` methods can be called by anyone.  `@private` methods which
modify the class instance can only be called by module functions, and
constant `@private` methods can be called by friends.

Note that reassignable methods, e.g., those defined with
`::some_constant_method(...Args); return_type` or `;;some_mutating_method(...Args); return_type`
can only be reassigned based on their visibility as if they were variables.
I.e., public reassignable methods can be reassigned by anyone,
protected reassignable methods can be reassigned by friends or module,
and private reassignable methods can only be reassigned within the module.

One final note, child classes are considered friends of the parent class,
even if they are defined outside of the parent's directory, and even if they
are defined in the same module as the parent (discouraged).  What this means
is they can modify public and protected variables defined on the parent instance,
and read (but not modify) private variables.  Overriding a parent class method
counts as modifying the method, which is therefore possible for public and protected
methods, but not private methods.

## getters and setters on class instance variables

Note that all variables defined on a class are given methods to access/set
them, but this is done with syntactical sugar.  That is,
*all uses of a class instance variable are done through getter/setter methods*,
even when accessed/modified within the class.  The getters/setters are methods
named the `function_case` version of the `Variable_case` variable,
with various arguments to determine the desired action.
TODO: at some point we need to have a "base case" so that we don't infinitely recurse;
should a parent class not call the child class accessors?  or should we only
not recurse when we're in a method like `;;x(@New X): { X = @New X }`?  or should we
avoid recursing if the variable was defined in the class itself?  (probably the
latter, as it's the least surprising.)

```
# for example, this class:
example: [@visibility X; str("hello")]
W = example()
W X += ", world"
print(W X)  # prints "hello, world"

# expands to this:
example:
[   @invisible
    X; str
]
{   # no-copy readonly reference getter.
    @visibility
    ::x(): (Str:)
        (:X)

    # no-copy writable reference getter.
    ;;x(): (Str;)
        (;X)

    # copy getter; has lower priority than no-copy getters.
    ::x(): str
        X

    # setter.
    ;;x(Str.):
        X = Str!

    # swapper: swaps the value of X with whatever is passed in.
    @visibility
    ;;x(Str;):
        X <-> Str

    # no-copy "take" method.  moves X from this temporary.
    @visibility
    ..x(): X!
}
W = example()
W x(W x() + ", world")
print(W x())
```

If you define overloads for any of these methods on child classes,
they will be used as the getters/setters for that variable.  Anyone
trying to access the variable (or set it) will use the overloaded methods.
Note that only one of the copy or getter methods needs to be defined
on a class; the compiler will automatically use the defined method internally
even if the undefined method is requested.  The same is true of swapper
and modifier classes.

```
# a class with a getter and setter gets reference getters automatically:
just_copyable: [@invisible A_var; int]
{   ::some_var(): int
        return A_var - 1000

    ;;some_var(Int.): null
        A_var = Int + 1000

    #(#
    # the following references become automatically defined;
    # they are just thin wrappers around the getters/setters.

    # writable reference
    ;;some_var(): (Int;)
        refer
        (   ;M
            {some_var(:$O)}         # getter: `O` is an instance of `just_copyable`
            {some_var(;$O, .$Int)}  # setter
        )

    # readonly reference
    ::some_var(): (Int)
        refer
        (   :M
            {some_var(:$O)}
        )

    # similarly a no-copy take method becomes defined based on the getter.
    ..some_var(): int
        A_var! - 1000
    #)#
}

# a class with a swapper method gets a setter and taker method automatically:
just_swappable: [@invisible Some_var; int]
{   @visibility
    ;;some_var(Int;): null
        Some_var <-> Int
        # you can do some checks/modifications on Some_var here if you want,
        # though it's best not to surprise developers.  a default-constructed
        # value for `Some_var` (e.g., in this case `Int: 0`) should be allowed
        # since we use it in the modifier to swap out the real value into a temp.
        # if that's not allowed, you would want to define both the swapper
        # and modifier methods yourself.

    #(#
    # the following setter becomes automatically defined:
    ;;some_var(Int.): null
        some_var(;Int)

    # and the following take method becomes automatically defined:
    ..some_var(): t
        Temporary; int
        # swap Some_var into Temporary:
        some_var(;Temporary)
        Temporary!
    #)#
}

# a class with a readonly reference getter method gets a copy getter automatically:
just_gettable: [@invisible Some_var; int]
{   ::some_var(): (Int:)
        (Int: Some_var)

    #(#
    # the following becomes automatically defined:
    ::some_var(): int
        # uses automatic conversion of (Int) -> int:
        some_var()
    #)#
}

# a class with a writable reference method gets a swapper and taker method automatically:
just_referable: [@invisible Some_var; int]
{   ;;some_var(): (Int;)
        (Int; Some_var)

    #(#
    # the following swapper becomes automatically defined:
    ;;some_var(Int;): null
        Int <-> some_var()

    # the following setter becomes automatically defined:
    ;;some_var(Int.): null
        some_var() = Int!

    # and the following taker method becomes automatically defined:
    ..some_var(): t
        Result; int
        Result <-> some_var()
        Result
    #)#
}

# A class with just a take method doesn't get swapper and modifier methods automatically;
# the take method is in some sense a one way modification (pull only) whereas swapper and
# modifier methods are two way (push and pull).
```

TODO: some example of child class overriding parent class getter/setters.
TODO: parent class with getter defined, child class with copy defined.

## parent-child classes and method overrides

You can define parent-child class relationships with the following syntax.
For one parent, `child_class: parent_class_name {#( child methods )#}`.  Multiple
inheritance is allowed as well, via `all_of[parent1, parent2] {#( child methods )#}`.
TODO: do we need `m` as polymorphic here like in Typescript?  we're not doing builder patterns.
We can access the current class instance using `M`,
and `m` will be the current instance's type.  Thus, `m` is
the parent class if the instance is a parent type, or a subclass if the instance
is a child class.  E.g., a parent class method can return a `m` type instance,
and using the method on a subclass instance will return an instance of the subclass.
If your parent class method truncates at all (e.g., removes information from child classes),
make sure to return the same `parent_class_name` that defines the class.

We can access member variables or functions that belong to that the parent type,
i.e., without subclass overloads, using the syntax `parent_class_name some_method(M, ...Args)`
or `parent_class_name::some_method(...Args)`.  Use `M;` to access variables or methods that will
mutate the underlying class instance, e.g., `parent_class_name some_method(M;, ...Args)`
or `parent_class_name;;some_method(...Args)`.  oh-lang doesn't have a `super` keyword
because we want inheritance to be as clear as composition for how method calls work.

Some examples:

```
animal: [Name: string]
{   ;;renew(M Name: string): {}

    # define two methods on `animal`: `speak` and `go`.
    # these are "abstract" methods, i.e., not implemented by this base class.
    ::speak(): null
    ::go(): string

    # this method is defined, so it's implemented by the base class.
    # derived classes can still change it, though.
    ::escape(): null
        print("${Name} ${go()} away!!")

    # a method that returns an instance of whatever the class instance
    # type is known to be.  e.g., an animal returns an animal instance,
    # while a subclass would return a subclass instance:
    ::clone(): m
        return m(Name clone())
}

snake: animal
{   # if no `renew` functions are defined,
    # child classes will inherit their parent `renew()` methods.

    ::speak(): null
        print("hisss!")
    ::go(): string
        return "slithers"

    # no need to override `clone`, since we can create a snake using a name.
}

Snake: snake(Name: "Fred")
Snake escape()  # prints "Fred slithers away!!"
```

To define extra instance variables for a child class, you'll use this notation:

```
cat: all_of[animal, m: [Fur_balls: int]]
{   # here we define a `renew` method, so the parent `renew` methods
    # become hidden to users of this child class:
    ;;renew(): null
        # can refer to parent methods using the `Variable_case`
        # version of the `type_case` class name:
        Animal renew(Name: "Cat-don't-care-what-you-name-it")
        Fur_balls = 0

    ::speak(): null
        print("hisss!")
    ::go(): string
        return "saunters"

    ::escape(): null
        print("CAT ESCAPES DARINGLY!")

    # the parent `clone()` method won't work, so override:
    ::clone(): m
        # cats are essentially singletons, that cannot have their own name;
        m()
}

Cat: cat()
Cat escape()    # prints "CAT ESCAPES DARINGLY!"
```

We have some functionality to make it easy to pass `renew` arguments to
a parent class via the `Parent_class_name` namespace in the constructor arguments.
This way you don't need to add the boiler plate logic inside the
constructor like this `;;renew(Parent_argument): Parent renew(Parent_argument)`,
you can make it simpler like this instead:

```
horse: all_of[animal, m: [Owner: str]]
{   # this passes `Name` to the `animal` constructor and sets `Owner` on self:
    ;;renew(Animal Name: str, M Owner: str, Neigh_times: int = 0)
        range(Neigh_times) each Int_:
            This speak()

    ::speak(): null
        print("Neigh!")

    ::go(): string
        return "gallops"
}

Horse: horse(Name: "James", Owner: "Fred", Neigh_times: 1)
print(Horse Owner)  # Fred
print(Horse Name)   # James
```

All abstract base classes also provide ways to instantiate using lambda functions.
All abstract methods must be defined for the instance to be created, and if a
`reset` method is defined on the parent, any arguments passed into the first reset
(i.e., which is the default constructor) should be defined for the lambda class.
While these don't look like normal lambda functions, they use the notation `::speak(): null`
as a shortcut for `speak(M): null`, which works as a lambda.

```
Weird_animal: animal
(   Name: "Waberoo"
    ::speak(): null
        print("Meorooo")
    ::go(): "meanders"
    ::escape(): null
        # to call the parent method `escape()` in here, we can use this:
        animal::escape()
        # `M` is required since `Name` and `go()` are not obviously in scope.
        print("${M Name} ${M go()} back...")
        # or we can use this:
        animal escape(M)
)

Weird_animal escape()    # prints "Waberoo ... meanders ... meanders back ... meanders away!!"
```

## operator overloading

To overload operators for a class, we use the following syntax.

```
# this class checks for overflow/underflow and switches to a "null" (-128) if so.
flow8: [I8;]
{   ;;renew(M I8. -128): {}

    # cloning works without errors:
    m(O): m
        [I8: O I8]

    ::!(): bool     # overload `!M`
        I8 == -128 || I8 == 0

    ;;+=(O): null
        if I8 == -128
            return
        if O I8 == -128
            I8 = -128
            return
        I16. I8 + O I8
        I8 = i8(.I16) map({$Er_, -128})

    ::+(O): flow8
        Copy; M
        Copy += O
        Copy
}
```

And similarly for all other operators.

## inheritance and dynamic allocation

We will likely use C/C++ to implement oh-lang at first, i.e., so that it transpiles to C/C++.
In C++, a variable that is typed as a parent instance cannot be a child instance in disguise;
if a child instance is assigned to the variable, the extra derived bits get sliced off.  This
helps avoid dynamic allocation, because the memory for the parent class instance can be allocated
on the stack.  On the other hand, if the variable is a pointer to a parent instance, the variable
can actually point to a child instance in disguise.  This is great in practice for object-oriented
programming, since you can use the child instance in place of the parent instance; as long as
the child class fulfills the same contract as the parent class, it shouldn't matter the exact
implementation.  But this generally requires dynamic memory allocation, which has a cost.

In oh-lang, we want to make it easy for variables of a parent class to be secretly instances
of child classes, so by default we are paying the cost and dynamically allocating non-primitive
types.  That way, we can easily do things like this:

```
Some_animal; animal
Some_animal = snake(Name: "Josie")
Some_animal go()   # prints "slithers"
Some_animal = cat()
Some_animal go()   # prints "saunters"
```

TODO: discuss wrapper class which allocates enough data for any child class; if some child
class takes up too much memory it's created as a new unique pointer.

This is less surprising than the C++ behavior.  But in cases where users want to gain back
the no-dynamically-allocated class instances, we have a `@only` annotation that can be used
on the type.  E.g., `Some_variable: @only some_type` will ensure that `Some_variable` is
stack allocated (non-dynamically).  If defined with `;`, the instance can still be modified,
but it will be sliced if some child instance is copied to it.  To prevent confusion, we
enforce that upcasting (going from child to parent) must be done *explicitly*.  For example:

TODO: classes should probably be allowed to be marked final.  e.g., `i64` and similar
fixed-width integers should be `final` so that we don't need to worry about vtables,
or specifying `@only i64`.  classes that are `final` would not need to be marked `@only`.

```
# extra field which will get sliced off when converting from
# mythological_cat to cat:
mythological_cat: all_of[cat, m: [Lives; 9]]

Cat; @only cat
Mythological_cat; mythological_cat()

Cat = Mythological_cat      # COMPILER ERROR, implicit cast to `@only cat` not allowed.
Cat = cat(Mythological_cat) # OK.  explicit upcast is allowed.

Other_cat; @only cat
Cat <-> Mythological_cat    # COMPILER ERROR.  swaps not allowed unless both types are the same.
Other_cat <-> Cat           # OK.  both variables are `@only cat`.
```

We also will likely ignore `@only` annotations for tests, so that we can mock out
classes if desired.

One final note: abstract classes cannot be `@only` types for a variable, since they
are not functional without child classes overriding their abstract methods.

## template methods

You can define methods on your class that work for a variety of types.

```
some_example: [Value: int]
{   ;;renew(Int.): null
        Value = Int!

    # in your own code, prefer adding `t(Some_example): t`
    # outside of this class body as the more idiomatic way
    # to convert `Some_example` to a different type.
    ::to(): ~t
        return t(Value)
}

Some_example: some_example(5)

# you can explicitly ask for a type like this:
To_string: string = Some_example to()

# or like this:
{My_value: dbl} Some_example to()

# but you can't implicitly ask for the type.
Unspecified: Some_example to()      # COMPILER ERROR, specify a type for `Unspecified`
```
it'd be hard to give up `Array[3]` for simplicity/convenience/conciseness, etc,
i don't prefer `Array at(3)` or `Array index(3)`.

## generic/template classes

TODO: discuss how `null` can be used as a type in most places.
But note that if you have a generic function defined like this,
we are already assuming some constraints:
```
my_generic[of](Y: of, Z: of): of
    X: Y * Z
    X
```
If `of` was nullable, then `X` would potentially be nullable, and should
be defined via `X?: Y * Z`.  We can probably avoid this by requiring non-null
in certain template declarations.  i.e., if we see a definition like `X: Y * Z`
we have to assume `Y` and `Z` are non-null.

To create a generic class, you put the expression `[types...]` after the
class identifier, or `[of]` for a single template type, where `of` is the
[default name for a generic type](#default-named-generic-types).  For example, we use
`my_single_generic_class[of]: [...]` or `my_multi_generic_class[type1, type2]: [...]`
for single/multiple generics, respectively, to define the generic class.
When specifying the types of the generic class, we use
`my_single_generic_class[int]` (for an `of`-defined generic class) or
`my_multi_generic_class[type1: int, type2: str]` (for a multi-type generic).
Note that any static/class methods defined on the class can still be accessed
like this: `my_single_generic_class[int] my_class_function(...)` or
`my_multi_generic_class[type1: int, type2: str] other_class_function()`.

```
generic_class[id, value]: [Id, Value]
{   ;;renew(M Id: id, M Value: value): {}
}

# creating an instance using type inference:
Class_instance: generic_class(Id: 5, Value: "hello")
 
# creating an instance with template/generic types specified:
Other_instance: generic_class[id: dbl, value: string](Id: 3, Value: "4")
```

### default-named generics

If you have a generic class like `my_generic[type1, type2]`, you can use them as a
default-named function argument like `My_generic[type1, type2]`, which is short for
`My_generic: my_generic[type1, type2]`.  This works even for generics over values,
e.g., if `fixed_array[Count]` is an array of size `Count`, then `Fixed_array[3]`
can be a declaration for a fixed array of size 3.  We can distinguish between 
`Fixed_array[3]` being (1) this declaration or (2) a request to access the fourth
element in an array based on whether `Fixed_array` is in scope.

### generic class type mutability

It may be useful to create a generic class that whose specified type
can have writeable or readonly fields.  This can be done using `Variable_name\` generic_type`
inside the generic class definition to define variables, and then specifying
the class with `[type1: specified_readonly_type, type2; specified_writeable_type]`.

```
mutable_types[x, y, z]:
[   # these fields are always readonly:
    R_x: x
    R_y: y
    R_z: z
    # these fields are always writeable:
    W_x; x
    W_y; y
    W_z; z
    # these fields are readonly/writeable based on what is passed in
    # to `mutable_types` for each of `x`, `y`, and `z`, respectively.
    V_x` x
    V_y` y
    V_z` z
]
{   # you can also use these in method/function definitions:
    ::some_method(Whatever_x` x, Whatever_y` y): null
}

# the following specification will make `V_x` and `V_z` writeable
# and `V_y` readonly:
my_specification: mutable_types[x; int, y: string, z; dbl]
```

We use a new syntax here because it would be confusing
to reinterpret a generic class declaration of a variable declared using `:`
as writeable in a specification with a `;`.

Note that if the generic class has no backticks inside, then it is a compile error
if you try to specify the generic class with a `;` type.  E.g., if we have the declaration
`generic[a]: [A]`, then the specification `My_gen: generic[a; int](5)` is a compile error.
If desired, we can switch to `generic[a]: [A\`]` to make the specification correct.

### virtual generic methods

You can also have virtual generic methods on generic classes, which is not allowed by C++.

```
generic[of]: [Value; of]
{   ::method(~U): u
        U_Value: u = (U * Value) ?? panic()
        U + U_value
}

Generic; generic[str]
Generic Value = "3"
print(Generic method(2_i32))    # prints "35" via `2_i32 + i32(2_i32 * "3")`

specific[of: number]: all_of[generic[of], m: [Scale; of]]
{   ;;renew(M Scale; of = 1, Generic Value.): {}

    ::method(~U): u
        Parent_result: Generic::method(U)
        Scale * Parent_result
}

Specific(Value: 10_i8, Scale: 2_i8)
print(Specific method(0.5)) # should print "11" via `2 * (0.5 + dbl(0.5 * 10))`
```

Just like with function arguments, we can elide a generic field value if the
field name is already a type name in the current scope.  For example:

```
@My_namespace at: int
value: [X: flt, Y: flt]
My_lot; lot[@My_namespace at, value]
# Equivalent to `My_lot; lot[at: @My_namespace at, value]`.
```

### generic type constraints

To constrain a generic type, use `[type: constraints, ...]`.  In this expression,
`constraints` is simply another type like `non_null` or `number`, or even a combination
of classes like `all_of[container[id, value], number]`.  It may be recommended for more
complicated type constraints to define the constraints like this:
`my_complicated_constraint_type: all_of[t1, one_of[t2, t3]]` and declaring the class as
`new_generic~[of: my_complicated_constraint_type]`, which might be a more readable way to do
things if `my_complicated_constraint_type` is a helpful name.

### generic type defaults

Type defaults follow the same pattern as type constraints but the default types are
not abstract.  So we use `[type: default_type, ...]` where `default_type` is a class
that is non-abstract.

### overloading generic types

Note that we can overload generic types (e.g., `array[int]` and `array[Count: 3, int]`),
which is especially helpful for creating your own `hm` result class based on the general
type `hm[er, ok]`, like `@Namespace er: one_of[Oops, My_bad], hm[of]: hm[ok: of, @Namespace er]`.
Here are some examples:

```
# Note that in oh-lang we could define this as `pair[@First of, @Second of]`
# so we don't need to specify `first: int, second: dbl`, but for illustration
# in the following examples we'll make the generic parameters named.
pair[first, second]: [First, Second]
pair[of]: pair[first: of, second: of]

# examples using pair[of]: ======
# an array of pairs:
Pair_array: array[pair[int]]([[First: 1, Second: 2], [First: 3, Second: 4]])
# a pair of arrays:
Pair_of_arrays: pair[array[int]]([First: [1, 2], Second: [3, 4]])

# examples using pair[first, second]: ======
# an array of pairs:
Pair_array: array[pair[first: int, second: dbl]]
(   [First: 1, Second: 2.3]
    [First: 100, Second: 0.5]
)
# a lot of pairs:
Pair_lot: lot[at: str, pair[first: int, second: dbl]]
(   "hi there": [First: 1, Second: 2.3]
)
```

### default named generic types

TODO: talk about inferring things here

The default name for a type is `of`, mostly to avoid conflicts with
`type` which is a valid verb (e.g., to type in characters), but also
to reduce the letter count for generic class types.  Default names
are useful for generics with a single type requirement, and can be
used for overloads, e.g.:

```
a_class[x, y, N: count]: array[[X, Y], Count: N]

a_class[of]: a_class[x: of, y: of, N: 100]
```

Similar to default-named arguments in functions, default-named generics
allow you to specify the generic without directly using the type name.
For example:

```
# use the default-name `type` here:
a_class[of, N: count]: a_class[x: of, y: of, N]

# so that we can do this:
An_instance: a_class[dbl, N: 3]
# equivalent but not idiomatic: `An_instance: a_class[of: dbl, N: 3]`.
```

Similar to default-named arguments in functions, there are restrictions.
You are not able to create multiple default-named types in your generic
signature, e.g., `my_generic[@A of, @B of]`, unless we use `@First` and
`@Second` namespaces, e.g., `my_generic[@First of, @Second of]`.  These
should only be used in cases where order intuitively matters.

### generic overloads must use the original class or a descendant

To avoid potential confusion, overloading a generic type must use
the original class or a descendant of the original class for any
overloads.  Some examples:

```
some_class[x, y, N: count]: [ ... ]

# this is OK:
some_class[of, N: count]: some_class[x: of, y: of, N]

# this is also OK:
child_class[of]: some_class[x: of, y: of, N: 256]
{   # additional child methods
    ...
}
some_class[of]: child_class[of]

# this is NOT OK:
some_class[t, u, v]: [ ...some totally different class... ]
```

Note that we probably can support a completely specified generic class,
e.g., `some_class: some_class[my_default_type]`; we can still distinguish
between the two usages of `some_class[specified_type]` and `some_class`.

### type tuples

One can conceive of a tuple type like `[x, y, z]` for nested types `x`, `y`, `z`.
They are grammatically equivalent to a `lot` of types (where usually order doesn't matter),
and their use is make it easy to specify types for a generic class.  This must be done
using the spread operator `...` in the following manner.

```
tuple_type: [x, y, z]

# with some other definition `my_generic[w, x, y, z]: [...]`:
some_specification: my_generic[...tuple_type, w: int]

# you can even override one of your supplied tuple_type values with your own.
# make sure the override comes last.
another_spec[@Override of]: my_generic[...tuple_type, w: str, x: @Override of]

# Note that even if `tuple_type` completely specifies a generic class
# `some_generic[x, y, z]: [...]`, we still need to use the spread operator
# because `some_generic tuple_type` would not be valid syntax.  Instead:
a_specification: some_generic[...tuple_type]
```

Here is an example of returning a tuple type.

```
tuple[Dbl]: [number, vector2: any]
    if abs(Dbl) < 128.0
        [number: flt, vector2: [X: flt, Y: flt]]
    else
        [number: dbl, vector2: [X: dbl, Y: dbl]]

my_tuples: tuple[random() * 256.0]
My_number; my_tuples number(5.0)
My_vector; my_tuples vector2(X: 3.0, Y: 4.0)
```

See also [`new[...]: ...` syntax](#returning-a-type).


### default field names with generics

Note that generic classes like `generic[of]: [Of]` do not work the same way as
generic arguments in functions like `fn[of](Of)`.  The latter uses a default-named
argument in a reference object and the former creates an object whose field
is always named `Of`, regardless of what the generic type `of` is.  Thus
`fn[of](Of)` can be called with `fn[int](5)` (without `Int: 5` specified), while
creating a generic class `generic[of]: [Of]` will always have the field named as `Of`,
e.g., `generic[int]: [Of: int]` instead of `generic[int]: [Int: int]`, and which
should be instanced as `Generic[int]: [Of: 3]`.

There's a slight bit of inconsistency here, but it makes defining generic classes
much simpler, especially core classes like `hm[ok, er]: one_of[ok, er] { ... #( extra methods )# }`,
so we always refer to a good value as `Ok` and an error result as `Er`, rather
than whatever the internal values are.

## common class methods

All classes have a few compiler-provided methods which cannot be overridden.

* `(M;)!: m` creates a temporary with the current instance's values, while
    resetting the current instance to a default instance -- i.e., calling `renew()`.
    Internally, this swaps pointers, but not actual data, so this method
    should be faster than copy for types bigger than the processor's word size.
* `..map(an(M.): ~t): t` to easily convert types or otherwise transform
    the data held in `M`.  This method consumes `M`.  You can also overload
    `map` to define other useful transformations on your class.
* `::map(an(M:): ~t): t` is similar to `..map(an(M.): ~t): t`,
    but this method keeps `M` constant (readonly).  You can overload as well.
* `m(...): m` class constructors for any `;;renew(...): null` methods.
* `m(...): hm[ok: m, er]` class or error constructors for any methods defined as
    `;;renew(...): hm[ok: m, er]`
* `;;renew(...): null` for any `m(...): m` class constructors.
    This allows any writable variable to reset without doing `X = x(...)`,
    which may be painful for long type names `x`, and instead do `X renew(...)`.
* `;;renew(...): hm[er]` for any `m(...): hm[ok: m, er]` construct-or-error class functions
    This allows any writable variable to reset without doing `X = x(...) assert()`,
    which may be painful for long type names `x`, and instead do `X renew(...) assert()`.
* `Xyz;: (xyz;:)` gives a reference to the class instance, where `xyz` is the actual
    `type_case` type and `Xyz` is the `Variable_case` version of it.
    This is mostly useful for inheritance.

## singletons

Defining a singleton class is quite easy, simply by instantiating a class 
by using `Variable_case` when defining it.

```
Awesome_service: all_of
[   parent_class1, parent_class2, #(etc.)#,
    m: [Url_base: "http://my/website/address.bazinga"]
]
{   ::get(Id: string): awesome_data 
        Json: Http get("${Url_base}/awesome/${Id}") 
        return awesome_data(Json)
}
```

Using `@singleton type_case` on the LHS defines an abstract singleton.
These are useful when you want to be able to grab an instance of the concrete
child-class but only through the parent class reference.

```
### screen.oh ###
@singleton
screen: []
{   ;;draw(Image, Vector2): null
    ;;clear(Color: color Black)
}
### implementation/sdl-screen.oh ###
# TODO: we probably can convert `\/../screen screen` -> `\/../screen`
#       where we're requesting the class name of a file that's named correctly.
Sdl_screen: \/../screen screen
{   ;;draw(Image, Vector2): null
        # actual implementation code:
        M Sdl_surface draw(Image, Vector2)

    ;;clear(Color: color Black)
        M Sdl_surface clear(Color)
}
### some-other-file.oh ###
# this is an error if we haven't imported the sdl-screen file somewhere:
Screen; screen
Screen clear color(R: 50, G: 0, B: 100)
```

You get a run-time error if multiple child-class singletons are imported/instantiated
at the same time.

## sequence building

Sequence building is using the syntax `A@ [B, c()]` to create an object like `[B: A B, C: A c()]`,
similarly with `()` to create a reference object, or `{}` to evaluate a few methods on the same
object.  If you need the LHS of a sequence builder to come in at a different spot, use `@` inside
the parentheses, e.g., `A@ [B + @ x(), if @ y() { C } else { @ Z }, W]`, which corresponds to
`[B: B + A x(), Y: if A y() { C } else { A Z }, W: A W]`.  Note that if you use `@` anywhere in
a parenthetical statement, you need to use it everywhere you want the LHS to appear.
(A parenthetical statement is considered just one of statements here: `[Statement1, Statement2, ...]`.)

Why would you need sequence building?
One reason is that it makes declaring a bunch of private (or protected) variables convenient,
e.g., `simple_class: @private@ [My_var: int, My_var2: str]` instead of
`simple_class: [@private My_var: int, @private My_var2: str]`.  It's not as useful
for class inheritance where you can just use
`my_class: all_of[other_classes..., @private m: [My_var: int, My_var2: str]]`.

Another reason for sequence building:
some languages use a builder pattern, e.g., Java, where you add fields to an object
using setters.  For example, `/* Java */ MyBuilder.setX(123).setY(456).setZ("great").build()`.
In oh-lang, this is mostly obviated by named arguments: `my_class(X: 123, Y: 456, Z: "great")`
could do the same thing.  However, there are still situations where it's useful to chain 
methods on the same class instance, and oh-lang does not recommend returning a reference
to the class via the return type `(m)`.  More idiomatically, we use sequence building
with all the method calls inside a block.  For example, if we were to implement a builder pattern
with setters, we could combine a bunch of mutations like this:

```
# class definition:
my_builder: [...]
{   ;;set(String, Int): null    # no need to return `(m)`
}

# Note, inside the `{}` we allow mutating methods because `my_builder()` is a temporary.
# The resulting variable will be readonly after this definition + mutation chain,
# due to `My_builder` being defined with `:`.
My_builder: my_builder()@
{   set("Abc", 123)
    set("Lmn", 456)
    set("Xyz", 789)
    # etc.
}

# You can also do inline, but you should use commas here.
# Note that this variable can be mutated after this line due to being defined with `;`.
My_builder2; my_builder()@ {set("Def", 987), set("Uvw", 321)}
```

By default, if the left-hand side of the sequence builder is writable (readonly),
the methods being called on the right will be the writable (readonly) versions
when using implicit member access (e.g., not explicitly using `::` or `;;`).
E.g., if `my_builder()` is the left-hand side for the sequence builder, it is a
temporary which defaults to writable.  You can explicitly ask for the readonly
(or writable) version of a method using `::` (or `;;`), although it will be a
compile-error if you are trying to write a readonly variable.

The return value of the sequence builder also depends on the LHS.
If the LHS is a temporary, the return value will be the temporary after it has been called
with all the methods in the RHS of the sequence builder.  E.g., from the above example,
a `my_builder` instance with all the `set` methods called.  Otherwise, if the LHS
is a reference (either readonly or writable), the return value of the sequence
builder will depend on the type of parentheses used:
`{}` returns the value of the last statement in `{}`,
`[]` creates an object with all the fields built out of the RHS methods, and
`()` creates a reference object with all the fields built out of the RHS methods.
Some examples of the LHS being a reference follow:

```
Readonly_array: [0, 100, 20, 30000, 4000]
Results: Readonly_array@
[   [2]            # returns 20
    ::sort()       # returns a sorted copy of the array; `::` is unnecessary
    ::print()      # prints unsorted array; `::` is unnecessary
    # this will throw a compile-error, but we'll discuss results
    # as if this wasn't here.
    ++@;;[3]        # compile error, `Readonly_array` is readonly
]
# should print [0, 100, 20, 30000, 4000] without the last statement
# Results = [Int: 20, Sort: [0, 20, 100, 4000, 30000]]

Writeable_array; [-1, 100, 20, 30000, 4000]
Result: Writeable_array@
{   [2]            # returns 20
    sort()        # in-place sort, i.e., `;;sort()`
    ++@;;[3]        # OK, a bit verbose since `;;` is unnecessary
    # prints the array after all the above modifications:
    ::print()      # OK, we probably don't have a `;;print()` but you never know
    min()
}
# should print [-1, 20, 100, 4001, 30000]
# Result = [20, 4001, -1]
```

### field renaming in sequence builders

You can use field names in sequence builders in order to better organize things.
This is only useful if the LHS is not a temporary, since a temporary LHS is returned
as sequence builder's value, or if you are using the variable for something else inside
the sequence builder.

```
My_class: [...]
Results: My_class@
[   Field1: @ my_method()
    Field2: @ next_method()
]
# The above is equivalent to the following:
Results:
[   Field1: My_class my_method()
    Field2: My_class next_method()
]

# This is a compile error because the LHS of the sequence builder `My_class get_value()`
# is a temporary, so the fields are not used in the return value.
# this also would be a compile error for `()` and `{}` sequence builders.
Results: My_class get_value()@
[   Field1: @ do_something()
    Field2: @ do_something_else()
]   # COMPILE ERROR: Field1 is not used anywhere

# this would be ok:
Results: My_class get_value()@
{   Field1: @ do_something()
    print(@ do_something_else() * Field1)
}
```

### nested sequence builders

There is one exception in oh-lang for shadowing identifiers, and it is for `@`
inside nested sequence builders.  We don't expect this to be a common practice.

```
# Example method sequence builder:
My_class@
[   my_method()@ [next_method(), next_method2(), Nested_field]
    other_method()
    Some_field
]

# Is equivalent to this sequence:
Result: My_class my_method()
Next_method: Result next_method()
Next_method2: Result next_method2()
Nested_field: Result Nested_field
Other_method: My_class other_method()
Some_field: My_class Some_field
# This is constructed (since it's defined with `[]`):
[My_method: [Next_method, Next_method2, Nested_field], Other_method, Some_field]
```

# aliases

Aliases enable writing out logic with semantically similar descriptions, and 
are useful for gently adjusting programmer expectations.  The oh-lang formatter will
substitute the preferred name/logic for any aliases found.

Aliases can be used for simple naming conventions, e.g.:

```
options: choose
[   one_of[Align_inherit_x: 0, Align_center_x, Align_left, Align_right]
]
{   @alias Inherit_align_x: Align_inherit_x
}

Options: options Inherit_align_x    # converts to `options Align_inherit_x` on next format.
```

Aliases can also be used for more complicated logic and even deprecating code.

```
my_class: [X; int]
{   # explicit constructor:
    m(X; int): [X]

    # implicit constructor:
    ;;renew(M X; int): null

    # This was here before...
    # ;;my_deprecated_method(Delta_x: int): null
    #     X += Delta_x

    # But we're preferring direct access now:
    @alias ;;my_deprecated_method(Delta_x: int): null
        X += Delta_x
}

My_class; my_class(X: 4)
My_class my_deprecated_method(Delta_x: 3)   # converts to `My_class X += 3` on next format.
```

While it is possible, it is not recommended to use aliases to inline code.
This is because the aliased code will be "inlined" in the source directly,
so it won't benefit from any future updates to the aliased code.

# modules

Every file in oh-lang is its own module, and we make it easy to reference
code from other files to build applications.  All `.oh` files must be known
at compile time, so referring to other files gets its own special notation.
The operator `\/` begins a file-system search in the current directory.
and two backslashes becomes a search for a library module, e.g., `\\math`.

Subsequent subdirectories are separated using forward slashes, e.g.,
`\/relative/path/to/file` to reference the file at `./relative/path/to/file.oh`,
and `..` is allowed between forward slashes to go to the parent directory relative
to the current directory, e.g., `\/../subdirectory_in_parent_directory/other/file`.
Note that we don't include the `.oh` extension on the final file, but the formatter
will remove this for you so you don't need to do this by hand when copying in file
paths.  You can also use `oh("./relative/path/to/file.oh")`, which does require
the final `.oh` extension.

For example, suppose we have two files, `vector2.oh` and `main.oh` in the same
directory.  Each of these is considered a module, and we can use backslashes
to invoke logic from these external files.

```
# vector2.oh
vector2: [X: dbl, Y: dbl]
{   ;;renew(M X: dbl, M Y: dbl): {}

    @order_independent
    ::dot(Vector2: vector2): dbl
        X * Vector2 X + Y * Vector2 Y
}

# main.oh
Vector2_oh: \/vector2   # .oh extension can be used but will be formatted off.
# alternatively: `Vector2_oh: oh("./vector2.oh")`
Vector2: Vector2_oh vector2(X: 3, Y: 4)
print(Vector2)
# you can also destructure imports like this:
[vector2]: \/vector2    # equivalent to `[vector2]: oh("./vector2.oh")`
```

Note that we cannot import a function like this: `[my_function]: \/other_file`;
to oh-lang this looks like a type.  You either need to specify the overload
that you're pulling in, e.g., `[my_function(Int): str]: \/other_file`,
or request all overloads via `[my_function(Call;): null]: \/other_file`.
Or you can just import the file and use the function as needed:
`Other_file: \/other_file, Other_file my_function(123)`.
TODO: i think we can relax this requirement; if you request `[my_function]` it can just
be the function with all overloads; otherwise we should technically require specifying
type "overloads" for generic types like `hm[of]: hm[ok: of, er]` that come from other files.
there's not a huge difference between types and functions, they both can
take arguments to return something else.

You can use this `\/` notation inline as well, which is recommended
for avoiding unnecessary imports.  It will be a language feature to
parse all imports when compiling a file, regardless of whether they're used,
rather than to dynamically load modules.  This ensures that if another imported file
has compile-time errors they will be known at compile time, not run time.

```
# importing a function from a file in a relative path:
print(\/path/to/relative/file function_from_file("hello, world!"))

# importing a function from the math library:
Angle: \\math atan2(X: 5, Y: -3)
```

Following the principle of laying out your code with the most important, higher-level
logic first, followed by more specific, lower-level logic, the formatter will automatically
move imports to the *bottom* of the file so that you can see the main part of your code
instantly.  Java imports are the absolute worst example of clutter before the main 
part of your code, and have inspired this rule.

To import a path that has special characters, just use the special characters
inline after the `\/`, e.g., `\/sehr/übel` to reference the file at `./sehr/übel.oh`.
For a path that has spaces (e.g., in file or directory names), use parentheses to
surround the path, e.g., `\\[library/path/with spaces]` for a library path or 
`\/(relative/path/with a space/to/a/great file)` for a relative path.  Or you can
use a backslash to escape the space, e.g., `\\library/path/with\ spaces` or
`\/relative/path/with\ a\ space/to/a/great\ file`.  Other standard escape sequences
(using backslashes) will probably be supported.

Note that we take the entire import as
if it were an `Variable_case` identifier.  E.g., `\\math` acts like one identifier, `Math`,
so `\\math atan(X, Y)` resolves like `Math atan(X, Y)`, i.e., member access or drilling down
from `Math: \\math`.  Similarly for any relative import; `\/relative/import/file some_function(Q)`
correctly becomes like `File some_function(Q)` for `File: \/relative/import/file`.

## scripts

While it's generally nice to compile your code for performance, there are times where
it doesn't make sense to optimize for performance at the cost of compile times.  For example,
when prototyping movement in a game, it's useful to get a feel for what your code is doing
by trying out many things.  For this reason, it's important that oh-lang offers an interpreter
for your scripts, so that you can iterate quickly without always waiting for your program to compile.

In order to reduce compile times, you can define scripts to be interpreted in your main binary
using files with an `.ohs` extension.  After you are satisfied with the script, you can promote
it to compiled code by converting the `.ohs` file to a `.oh` file.

Note that one downside of scripting is that what could be compile-time errors become runtime errors.

With that, you can do imports using the `oh` type, and note we need the assertion when dealing
with `.ohs` files, since they can fail at run-time:
`Script: oh("../my_script/doom.ohs") assert(Er: "should compile")`.

TODO: how are we actually going to do this, e.g., need to expose public/protected functions to
the calling code, pulling in other import dependencies should not reload code if we've already loaded
those dependencies in other compiled files, etc.

## tests

Unit tests should be written inside the file that they are testing.  Files should generally be less than
1000 lines of code, including tests, but this is not strictly enforced.  Because unit tests live inside
the files where the code is defined, tests can access private functions for testing.  It is generally
recommended to test the public API exhaustively, however, so private function testing should be redundant.
Tests are written as indented blocks with a `@test` annotation, and include a trailing `:` because
we are declaring a test.

```
@private
private_function(X: int, Y: int): [Z: str]
    Z: "${X}:${Y}"

@protected
protected_function(X: int, Y: int): [Z: str]
    [Z;] = private_function(X, Y)
    Z += "!"
    [Z]

public_function(X1: int, Y1: int, X2: int, Y2: int): null
    print(protected_function(X: X1, Y: Y1) Z, private_function(X: X2, Y: Y2))

@test "foundation works fine":
    test(private_function(X: 5, Y: 3)) == [Z: "5:3"]
    test(private_function(X: -2, Y: -7)) == [Z: "-2:-7"]

@test "building blocks work fine":
    test(protected_function(X: 5, Y: -3)) == [Z: "5:-3!"]
    test(protected_function(X: -2, Y: 7)) == [Z: "-2:7!"]

@test "public function works correctly":
    public_function(X1: -5, Y1: 3, X2: 2, Y2: 7)
    test(Test print()) == ["-5:3!2:7"]

    @test "nested tests also work":
        public_function(X1: 2, Y1: -7, X2: -5, Y2: -3)
        test(Test print()) == ["2:-7!-5:-3"]
```

See [the test definition](https://github.com/oh-lang/oh/blob/main/core/test.oh) for
how the `test` function and `@test` macro work.

Nested tests will freshly execute any parent logic before executing themselves.
This ensures a clean state.  If you want multiple tests to start with the same
logic, just move that common logic to a parent test.

Inside of a `test` block, you have access to a `Test` variable which includes
things like what has been printed (`Test print()`).  In this example, `Test print()`
will pull everything that would have been printed in the test, putting it into
a string array (one string per newline), for comparisons and matching.
It then clears its internal state so that new calls
to `Test print()` will only see new things since the last time `Test print()` was called.

Parametric tests are also possible; just make sure to use `@each` (or another control flow macro)
in order to expand the loops at compile time.

```
@test_only
test_case: [Argument: str, Return: int]

@test_only
Test_cases: lot[at: str, test_case]
[   "hello": [Argument: "hello world", Return: 11]
    "wow": [Argument: "wowee", Return: 5]
]

@test "do_something":
    # this common setup executes before each parametric test;
    # each nested test starts with the common setup from fresh
    # and doesn't continue to use the environment for the next nested test.
    get_environment_set_up()

    Test_cases @each(Name: at, Test_case:)
        @test "testing ${Name}":
            test(do_something(Test_case Argument)) == Test_case Return
```

Integration tests can be written in files that end with `.test.oh` or `.test.ohs` (i.e., as a script).
These can pull in any dependencies via standard file/module imports, including other test files.
E.g., if you create some test helper functions in `helper.test.oh`, you can import these
into other test files (but not non-test files) for usage.

Unit and integration tests are run via `oh test` in the directory you want, or `oh test subdirectory/`;
only tests in that directory (and recursive subdirectories) will be run.

## file access / file system

Files can be opened via the `file` class, which is a handle to a system file.
See [the `file` definition](https://github.com/oh-lang/oh/blob/main/core/file.oh).

TODO: make it possible to mock out file system access in unit tests.

# errors and asserts

## hm

oh-lang borrows from Rust the idea that errors shouldn't be thrown, they should be
returned and handled explicitly.  We use the notation `hm[ok, er]` to indicate
a generic return type that might be `ok` or it might be an error (`er`).
In practice, you'll often specify the generic arguments like this:
`hm[ok: int, er: string]` for a result that might be ok (as an integer) or it might
be an error string.  If your function never fails, but the interface requires using
`hm`, you can use `hm[ok, er: null]` to indicate the result will never be an error.

To make it easy to handle errors being returned from other functions, oh-lang uses
the `assert` method on a result class.  E.g., `Ok: My_hm assert()` which will convert
the `My_hm` result into the `ok` value or it will return the `er` error in `My_hm` from
the current function block, e.g., `Ok: what My_hm { Ok: {Ok}, Er: {return Er} }`.
It is something of a macro like `?` in Rust.  Note that `assert` doesn't panic,
and it *always runs*, not just in debug mode.  See [its section](#assert) for more details.

Note that we can automatically convert a result type into a nullable version
of the `ok` type, e.g., `hm[ok: string, er: error_code]` can be converted into
`string?` without issue, although as usual nulls must be made explicit with `?`.
E.g., `my_function(String_argument?: My_hm)` to pass in `My_hm` if it's ok or null if not,
and `String?: My_hm` to grab it as a local variable.  This of course only works
if `ok` is not already nullable, otherwise it is a compile error.

See [the hm definition](https://github.com/oh-lang/oh/blob/main/core/hm.oh)
for methods built on top of the `one_of[ok, er]` type.

```
Result: if X { ok(3) } else { er("oh no") }
if Result is_ok()
    print("ok")

# but it'd be nice to transform `Result` into the `Ok` (or `Er`) value along the way.
Result is(an(Ok): print("Ok: ${Ok}"))
Result is(an(Er): print("Er: ${Er}"))

# or if you're sure it's not an error, or want the program to terminate if not:
Ok: Result ?? panic("expected `Result` to be non-null!!")
```

A few keywords, such as `is`, are actually [operators](#is-operator), so we can
overload them and use them in this slightly more idiomatic way.  Notice that
we declare an `Ok` variable here so we need to use a colon (e.g., `Ok:`).

```
if Result is Ok:
    print("Ok: ", Ok)
elif Result is Er:
    print("Er: ", Er)
```

Or use `what` if you want to ensure via the compiler that you get all cases:

```
what Result
    Ok:
        print("Ok: ", Ok)
    Er:
        print("Er: ", Er)
```

TODO: we probably want to enable things like `if Result is Ok: and Ok != 0 {...} else {...}`
and similarly for `Er`.

## assert

The built-in `assert` statement will shortcircuit the block if the rest of the statement
does not evolve to truthy.  As a bonus, when returning, all values will be logged to stderr
as well for debugging purposes for debug-compiled code.

```
assert(Some_variable == Expected_value)   # throws if `Some_variable != Expected_value`,
                                        # printing to stderr the values of both `Some_variable`
                                        # and `Expected_value` if so.

assert(Some_class method(100))       # throws if `Some_class method(100)` is not truthy,
                                    # printing value of `Some_class` and `Some_class method(100)`.

assert(Some_class other_method("hi") > 10)    # throws if `Some_class other_method("hi") <= 10`,
                                            # printing value of `Some_class` as well as
                                            # `Some_class other_method("hi")`.
```

If you want to customize the return error for an assert, pass it an explicit
`Er` argument, e.g., `assert(My_value, Er: "Was expecting that to be true")`;
and note that asserts can be called like `My_value assert()` or `Positive assert(Er: "oops")`.

Note that `assert` logic is always run, even in non-debug code.  To only check statements in the
debug binary, use `assert(Debug_only, ...)`, which otherwise has the same signature as `assert(...)`.
Using debug asserts is not recommended, except to enforce the caller contract of private/protected
methods.  For public methods, `assert` should always be used to check arguments.

Note that for functions that return results, i.e., `hm[ok, er]`, `assert` will automatically
return early with an `er` based on the error the `assert` encountered.  If a function does
*not* return a result, then using `assert` will be a run-time panic; to make sure that's
what you want, annotate the function with `@can_panic`, otherwise it's a compile error.

## automatically converting errors to null

If a function returns a `hm` type, e.g., `my_function(...): hm[ok, er]`,
then we can automatically convert its return value into a `one_of[ok, null]`, i.e.,
a nullable version of the `ok` type.  This is helpful for things like type casting;
instead of `My_int: what int(My_dbl) {Ok. {Ok}, Er: {-1}}` you can do
`My_int: int(My_dbl) ?? -1`.  Although, there is another option that
doesn't use nulls:  `int(My_dbl) map(fn(Er_): -1)`, or via
[lambda functions](#lambda-functions): `int(My_dbl) map({$Er_, -1})`.

TODO: should this be valid if `ok` is already a nullable type?  e.g.,
`my_function(): hm[ok: one_of[int, null], er: str]`.
we probably should compile-error-out on casting to `Int?: my_function()` since
it's not clear whether `Int` is null due to an error or due to the return value.
maybe we allow flattening here anyway.

# standard container classes (and helpers)

Brackets are used to create containers, e.g., `Y: "Y-Naught", Z: 10, [X: 3, (Y): 4, Z]`
to create a lot with keys "X", the value of `Y` ("Y-Naught"), and "Z", with
corresponding values 3, 4, and the value of `Z` (10).  Thus any bracketed values,
as long as they are named, e.g., `A: 1, B: 2, C: 3, [A, B, C]`, can be converted
into a lot.  Because containers are by default insertion-ordered, they can implicitly
be converted into an array depending on the type of the receiving variable.
This "conversion" happens only conceptually; constructing an array does construct
a `lot` first and then convert.

```
er: one_of
[   Out_of_memory
    # etc.
]

hm[of]: hm[ok: of, er]

container[at, of: non_null, count_with: select_count]: []
{   # Returns `Null` if `At` is not in this container,
    # otherwise the `of` instance at that `At`.
    # This is wrapped in a reference object to enable passing by reference.
    # TODO: do we like this?  it looks a bit like SFO logic that we killed off.
    # USAGE:
    #   # Get the value at `At: 5` and make a copy of it:
    #   Of?: Container[At: 5]
    #   # Get the value at `At: 7` and keep a mutable reference to it:
    #   (Of?;) = Container[At: 5]
    :;[At]: (Of?:;)

    # Returns the value at `At`, if present, while mooting
    # it in the container.  This may remove the `id` from
    # the container or may set its linked value to the default.
    # (Which depends on the child container implementation.)
    # Returns Null if not present.
    ;;[At]!?: of

    # safe setter.
    # returns an error if we ran out of memory trying to add the new value.
    ;;put(At, Of.): hm[null]

    # safe swapper.  replaces the value at `At` with the `Of` passed in,
    # and puts the previous value into `Of`.  the new or old value can
    # be null which means to delete what was there or that nothing was present.
    # returns an error if we ran out of memory trying to add the new value.
    ;;swap(At, Of?;): hm[null]
    
    @alias ::has(At): M[At] != Null
    @alias ::contains(At): M[At] != Null

    # Returns the number of elements in this container.
    ::count(): count_with

    # can implicitly convert to an iterator (with writeable/readonly references).
    ;:iterator(): iterator[(At:, Of;:)]

    # iterate over values.
    ;:ofs(): iterator[(Of;:)]

    # iterate over IDs.
    ::ats(): iterator[(At:)]
}
```

TODO: discuss the expected behavior of what happens if you delete an element
out of a container when iterating over it (not using the iterator itself, if
it supports delete).  for the default implementation, iterators will only
hold an index to an element in the container, and if that index no longer
indexes an element, we can stop iteration.

## arrays

An array contains a list of elements in contiguous memory.  You can
define an array explicitly using the notation `Array_name: array[element_type]` for the
type `element_type`.
The default-named version of an array does not depend on the element type;
it is always `Array`.  Declared as a function argument, a default-named array of strings
would thus be, e.g., `my_function(Array[string];:.): null`.
To define an array quickly (i.e., without a type annotation), use the notation
`["hi", "hello", "hey"]`.
Example usage and declarations:

```
# this is a readonly array:
My_array: array[dbl]([1.2, 3, 4.5])  # converts all to dbl
My_array append(5)  # COMPILE ERROR: My_array is readonly
My_array[1] += 5    # COMPILE ERROR: My_array is readonly

# writable integer array:
Array[int];         # declaring a writable, default-named integer array
Array append(5)     # now Array == [5]
Array[3] += 30      # now Array == [5, 0, 0, 30]
Array[4] = 300      # now Array == [5, 0, 0, 30, 300]
Array[2] -= 5       # now Array == [5, 0, -5, 30, 300]

# writable string array:
String_array; array[string](["hi", "there"])
print(String_array pop())    # prints "there".  now String_array == ["hi"]
```

The default implementation of `array` might be internally a contiguous deque,
so that we can pop or insert into the beginning at O(1).  We might reserve
`stack` for a contiguous list that grows in one direction only.

```
Array er: one_of
[   Out_of_memory
    # etc...
]
hm[of]: hm[ok: of, Array er]

# some relevant pieces of the class definition
# note that `of` cannot be nullable because containers must have non-null values to count,
# and we don't want to keep track of all the null values in the array to subtract from
# the highest array index to get the non-null count.
# TODO: we could support a nullable array but we'd need extra book keeping.
array[of]: container[id: index, value: of]
{   # TODO: a lot of these methods need to return `hm[of]`.
    # cast to bool, `::!!(): bool` also works, notice the `!!` before the parentheses.
    !!(M): bool
        count() > 0

    # Returns the value in the array if `Index < count()`, otherwise Null.
    # If `Index < 0`, take from the end of the array, e.g., `Array[-1]` is the last element
    # and `Array[-2]` is the second-to-last element.  If `Index < -Array count()` then
    # we will also return Null.
    ::[Index]?: of

    # Gets the existing value at `Index` if the array count is larger than `Index`,
    # otherwise increases the size of the array with default values and returns the
    # one at `Index`.  This has a possibility of panicking because it requires an increase
    # in memory; if that's important to check, ask for a result (see next method).
    ;:[Index]: (Of;:)

    # Gets the existing value at `Index` or creates a default if needed.
    # Can return an error if we run out of memory because this method can
    # expand the array if `Index >= ::count()`.
    ;;[Index]: hm[(Of;)]

    # Returns the value at Array[Index] while resetting it to the default value.
    # We don't bring down subsequent values, e.g., `Index+1` to `Index`, (1) for
    # performance, and (2) we need to preserve the invariant that `Array[Index]!`
    # followed by `!Array::[Index]` should be true, which would be impossible
    # to guarantee if we shifted all subsequent elements down.
    ;;[Index]!: of

    # note we have to import-rename the `count` type to something else.
    ::count(): count_with

    ;;append(Of): null

    # returns a null if no element at that index (e.g., array isn't big enough).
    ;;pop(Index: index = -1)?: of

    # returns a copy of this array, but sorted.
    # uses `o` instead of `m` to indicate that `M` doesn't change.
    ::sort(): o

    # sorts this array in place:
    ;;sort(): null
    ...
}
```

TODO: for transpiling to javascript, do we want to use the standard javascript `Array`
as an internal field or do we want to `@hide` it as the base class for the oh-lang `array`?
i.e., we create oh-lang methods that are aliases for operations on the JS `Array` class?
it depends on if we want other JS libraries to take advantage of oh-lang features or if
we want to make it look as native as possible -- for less indirection.

### fixed-count arrays

We declare an array with a fixed number of elements using the notation
`array[element_type, Count]`, where `Count` is a constant integer expression (e.g., 5)
or a variable that can be converted to the `count` type.  Fixed-count array elements
will be initialized to the default value of the element type, e.g., 0 for number types.

Fixed-count arrays can be passed in without a copy to functions taking
an array as a readonly argument, but will be of course copied into a 
resizable array if the argument is writable.  Some examples:

```
# readonly array of count 4
Int4: array[int, 4] = [-1, 5, 200, 3450]
# writable array of fixed-count 3:
Vector3; array[3, dbl] = [1.5, 2.4, 3.1]
print("Vector3 is [${Vector3[0]}, ${Vector3[1]}, ${Vector3[2]}]")

# a function with a writable argument:
do_something(Array[dbl];): array[2, dbl]
    # you wouldn't actually use a writable array argument, unless you did
    # some computations using the array as a workspace.
    # PRETENDING TO DO SOMETHING USEFUL WITH Array:
    return [Array pop(), Array pop()]

# a function with a readonly argument:
do_something(Array[dbl]): array[dbl, 2]
    return [Const_array[-1], Const_array[-2]]

# COMPILER ERROR: `Vector3` can't be passed as mutable reference
# to a variable-sized array:
print(do_something(Array; Vector3))    # prints [3.1, 2.4]

# OK: can bring in Vector3 by constant reference (i.e., no copy) here:
print(do_something(Array: Vector3))     # prints [3.1, 2.4]
```

There may be optimizations if the fixed-array count is known at compile-time,
i.e., being defined on the stack rather than the heap.  But when the fixed
count is unknown at compile time, the fixed-count array will be defined on the heap:

```
# note you can use an input argument to define the return type's
# fixed-array count, which is something like a generic:
count_up(Count): array[int, Count]
    Result; array[int, Count]
    Count each I: index
        Result[I] = I
    return Result

print(count_up(10))    # prints [0,1,2,3,4,5,6,7,8,9]
```

## lots 

A `lot` is oh-lang's version of a map (or `dict` in python).  Instead of mapping from a `key`
to a `value` type, lots link an `at` to an `of`.  This change from convention is mostly
to avoid overloading the term `map` which is used when transforming values such as `hm`, but also
because `map`, `key`, and `value` have little to do with each other; we don't "unlock" anything
with a C++ `map`'s key, we locate an instance.  Thus we use `at` for a locator
and `of` for the default-named type in `lot`.
The class definition is [here](https://github.com/oh-lang/oh/blob/main/core/lot.oh).

A lot can look up, insert, and delete elements by key quickly (ideally amortized
at `O(1)` or at worst `O(lg(N)`).  You can use this way to define a lot, e.g.,
`Variable_name: lot[at: id_type, value_type]`.  A default-named lot can
be defined via `Lot[at: id_type, value_type]`, e.g., `Lot[dbl, at: int]`.
Note that while an array can be thought of as a lot with the `at` type as `index`,
the array type `array[element_type]` would be useful for densely
packed data (i.e., instances of `element_type` for most indices), while the lot 
type `lot[element_type, at: index]` would be useful for sparse data.

To define a lot (and its contents) inline, use this notation:

```
Jim1: "Jim C"
Jim2: "Jim D"
Jim: 456
# lot linking string to ints:
Employee_ids: lot[at: int, str]
(   # option 1.A: `X: Y` syntax
    "Jane": 123
    # option 1.B: `[At: X, Of: Y]` syntax
    [At: "Jane", Of: 123]
    # option 1.C: `[X, Y]` syntax, ok if ID and value types are different
    ["Jane", 123]
    # option 1.D:
    Jane: 123
    # if you have some variables to define your id, you need to take care (see WARNING below).
    # option 2.A, wrap in parentheses to indicate it's a variable not an ID
    (Jim1): 203
    # option 2.B
    [At: Jim1, Of: 203]
    # option 2.C
    [Jim1, 203]
    # WARNING! not a good option for 2; no equivalent of option 1.D here.
    # Jim1: Jim1_id # WARNING, looks like option 1.C, which would define "Jim1" instead of "Jim C"
    # option 3: `X` syntax where `X` is a known variable, essentially equal to `@@X: X`
    Jim
)
# note that commas are optional if elements are separated by newlines,
# but required if elements are placed on the same line.
```

To define a lot quickly (i.e., without a type annotation), use the notation
`["Jane": 123, "Jim": 456]`.

Lots require an ID type whose instances can hash to an integer or string-like value.
E.g., `dbl` and `flt` cannot be used, nor can types which include those (e.g., `array dbl`).

```
Dbl_database; dbl[int]      # OK, int is an OK ID type
Dbl_dbl_database; dbl[dbl]  # COMPILE ERROR, dbl is an invalid ID type.
```

However, we allow casting from these prohibited types to allowed ID types.  For example:

```
Name_database; string[int]
Name_database[123] = "John"
Name_database[124] = "Jane"
print(Name_database[123.4]) # RUNTIME ERROR, 123.4 is not representable as an `int`.
print(Name_database[123.4 round(Stochastically)])   # prints "John" with 60% probability, "Jane" with 40%.

# note that the definition of the ID is a readonly array:
Stack_database; lot[at: array[int], string]
Stack_database[[1,2,3]] = "stack123"
Stack_database[[1,2,4]] = "stack124"
# prints "stack123" with 90% probability, "stack124" with 10%:
print(Stack_database[map([1.0, 2.0, 3.1], {$Dbl round(Stochastically)})])
# things get more complicated, of course, if all array elements are non-integer.
# the array is cast to the ID type (integer array) first.
Stack_database[[2.2, 3.5, 4.8] map({$Dbl round(Stochastically)})]
# result could be stored in [2, 3, 4], [2, 3, 5], [2, 4, 4], [2, 4, 5],
#                           [3, 3, 4], [3, 3, 5], [3, 4, 4], [3, 4, 5]
# but the ID is decided first, then the lot is added to.
```

Note: when used as a lot ID, objects with nested fields become deeply constant,
regardless of whether the internal fields were defined with `;` or `:`.
I.e., the object is defined as if with a `:`.  This is because we need ID
stability inside a container; we're not allowed to change the ID or it could
change places inside the lot and/or collide with an existing ID.

The default lot type is `insertion_ordered_lot`, which means that the order of elements
is preserved based on insertion; i.e., new IDs come after old IDs when iterating.
(Equality checking doesn't care about insertion order, however.)
Other notable lots include `at_ordered_lot`, which will iterate over elements in order
of their sorted IDs, and `unordered_lot`, which has an unpredictable iteration order.
Note that `at_ordered_lot` has `O(lg(N))` complexity for look up, insert, and delete,
while `insertion_ordered_lot` has some extra overhead but is `O(1)` for these operations,
like `unordered_lot`.

## sets

A set contains some elements, and makes checking for the existence of an element within
fast, i.e., O(1).  Like with container IDs, the set's element type must satisfy certain properties
(e.g., integer/string-like).  The syntax to define a set is `Variable_name: set[element_type]`.
You can elide `set` for default named arguments like this: `Set[element_type];` (or `:` or `.`).

```
er: one_of
[   Out_of_memory
    # etc...
]
hm[of]: hm[ok: of, er]

set[of: hashable]: container[id: of, value: true]
{   # Returns `True` iff `Of` is in the set, otherwise Null.
    # NOTE: the `true` type is only satisfied by the instance `True`;
    # this is not a boolean return value but can easily be converted to boolean.
    ::[Of]?: true

    # TODO: use `[]` for the unsafe API, `all()` or `put()` for the safe API (returning a `hm`)

    # Adds `Of` to the set and returns `True` if
    # `Of` was already in the set, otherwise `Null`.
    # this can be an error in case of running out of memory.
    ;;[Of]: hm[true?]

    # Ejects `Of` if it was present in the set, returning `True` if true
    # and `Null` if not.
    # A subsequent, immediate call to `::[Of]?` returns Null.
    ;;[Of]!?: true 

    # Modifier for whether `Of` is in the set or not.
    # The current value is passed into the callback and can be modified;
    # if the value was `Null` and is converted to `True` inside the function,
    # then the set will get `Of` added to itself.  Example:
    #   `Set[X] = if Condition {True} else {Null}` becomes
    #   `Set[X, fn(Maybe True?;): {Maybe True = if Condition {True} else {Null}}]`
    # TODO: if we used `True?` as the identifier everywhere we wouldn't need to do `Maybe True`, e.g.,
    #   `Set[X, fn(True?;): {True? = if Condition {True} else {Null}}]`
    # TODO: remove these methods and add `refer` methods
    ;;[Of, fn(@Maybe True?; true): ~t]: t

    # Fancy getter for whether `Of` is in the set or not.
    ::[Of, fn(@Maybe True?): ~t]: t

    ::count(): count_with

    # Unions this set with values from an iterator, returning True if
    # all the values from the iterator were already in this set, otherwise False.
    # can error if running out of memory.
    ;;all(Iterator[of].): hm[bool]

    # can convert to an iterator.
    ::iterator(): iterator[(Of:)]

    @hide ;:ofs(): iterator[(True;:)]
    @hide ;:ats(): iterator[(Of)]

    # Removes the last element added to the set if this set is
    # insertion ordered, otherwise any convenient element.
    # Returns an error if there is no element available.
    ;;pop(): hm[of]
    ;;pop(Of)?: M[Of]!

    @alias ;;remove(Of)?: M[Of]!
    ...
}
```

Like the IDs in lots, items added to a set become deeply constant,
even if the set variable is writable.

TODO: discussion on `insertion_ordered_set` and `unordered_set`, if we want them.

TODO: make it easy to pass in a set as an argument and return a lot with e.g. those IDs.
  maybe this isn't as important as it would be if we had a dedicated object type.

```
fn(Pick_from: ~o, Ids: ~k from ids(o)): pick(o, k)
    return pick(Pick_from, Ids)
```

TODO: discuss how `in` sounds like just one key from the set of IDs (e.g., `k in ids(o)`)
and `from` selects multiple (or no) IDs from the set (`k from ids(o)`).

## iterator

For example, here is a way to create an iterator over some incrementing values:

```
my_range[of: number]: all_of
[   @private m:
    [   Less_than: of
        Next_value: of = 0
    ]
    iterator[of]
]
{   ;;renew(Start_at: of = 0, M Less_than: of = 0): null
        Next_value = Start_at

    ;;next()?: of
        if Next_value < Less_than
            return Next_value++
        return Null

    ::peak()?: if Next_value < Less_than
        Next_value 
    else
        Null
}

my_range(Less_than: index(10)) each Index:
    print(Index)
# prints "0" to "9"
```

We want to avoid pointers, so iterators should just be indices into
the container that work with the container to advance, peak, etc.
Thus, we need to call `Iterator next` with the container to retrieve
the element and advance the iterator, e.g.:

```
Array: [1,2,3]
Iterator; iterator[int]
assert(Iterator next(Array) == 1)
assert(next(Array, Iterator;) == 2)  # you can use global `next`
assert(Iterator::peak(Array) == 3)
assert(peak(Iterator, Array) == 3)   # or you can use global `peak`
assert(Iterator next(Array) == 3)
assert(Iterator next(Array) == Null)
assert(Iterator peak(Array) == Null)
# etc.
```

The way we achieve that is through using an array iterator:

```
# by requesting the `next()` value of an array with this generic iterator,
# the iterator will become an array iterator.  this allows us to check for
# `@only` annotations (e.g., if `Iterator` was not allowed to change) and
# throw a compile error.
next(Iterator; iterator[t] @becomes array_iterator[t], Array: array[~t])?: t
    Iterator = array_iterator[t]()
    Iterator;;next(Array)

# TODO: probably can do `(Of;:.)`
array_iterator[of]: all_of
[   @private m: [Next; index]
    iterator[of]
]
{   ;;renew(Start: index = 0):
        Next = Start

    ;;next(Array[of])?: if Next < Array count()
        Array[Next++]
    else
        Null

    ::peak(Array[of])?: if Next < Array count()
        Array[Next]
    else
        Null
    
    # note that this function doesn't technically need to modify this
    # `array_iterator`, but we keep it as `;;` since other container
    # iterators will generally need to update their index/ID.
    ;;remove(Array[of];)?: if Next < Array count()
        Array remove(Next)
    else
        Null
}
```

We can also directly define iterators on the container itself.
We don't need to define both the `iterator` version and the `each` version;
the compiler can infer one from the other.  We write the `each` option
as a method called `each`.

```
array[of]: []
{   # TODO: technically this should be a `Block`, right?
    # look at `If_block` example below.
    ::each(fn(Of): loop): null
        # The `count` type has an `each` iterator method:
        count() each Index:
            if fn(M[Index]) == Break
                break

    # no-copy iteration, but can mutate the array elements.
    ;;each(fn(Of;): loop): null
        count() each Index:
            if fn(M[Index];) == Break
                break

    # mutability template for both of the above:
    ;:each(fn(Of;:): loop): bool
        count() each Index:
            if fn(M[Index];:) == Break
                return True
        return False
}
```

# standard flow constructs / flow control

We have a few standard control statements or flow control keywords in oh-lang.

TODO -- `return`
TODO -- description, plus `if/else/elif` section

Conditional statements including `if`, `elif`, `else`, as well as `what`,
can act as expressions and return values to the wider scope.  This obviates the need
for ternary operators (like `X = do_something() if Condition else Default_value` in python
which inverts the control flow, or `int X = Condition ? do_something() : Default_value;`
in C/C++ which takes up two symbols `?` and `:`).  In oh-lang, we borrow from Kotlin the idea that
[`if` is an expression](https://kotlinlang.org/docs/control-flow.html#if-expression),
and similarly for `what` statements (similar to
[`when` in kotlin](https://kotlinlang.org/docs/control-flow.html#when-expressions-and-statements)).

## then statements

We can rewrite conditionals to accept an additional `then` "argument".  For `if`/`elif`
statements, the syntax is `if Expression -> Then:` to have the compiler infer the `then`'s
return type, or `elif Expression -> Whatever_name: then[whatever_type]` to explicitly provide it
and also rename `Then` to `Whatever_name`.  Similarly for `what` statements, e.g.,
`what Expression -> Whatever_name: then[whatever]` or `what Expression -> Then:`.  `else`
statements also use the `->` expression, e.g., `else -> Then:` or `else -> Whatever: then[else_type]`.
Note that we use a `:` here because we're declaring an instance of `then`; if we don't use
`then` logic we don't use `:` for conditionals.  Also note that `then` is a thin wrapper
around the [`block`](#blocks) class (i.e., a reference that removes the `::loop()` method that
doesn't make sense for a `then`).  If you want to just give the type without renaming,
you can do `if Whatever -> Then[my_if_block_type]:`.

```
if Some_condition -> Then:
    # do stuff
    if Some_other_condition -> @Some_namespace Then:
        if Something_else1
            Then exit()
        if Something_else2
            @Some_namespace Then exit()
    # do other stuff

Result: what Some_value -> Then[str]:
    5
        ...
        if Other_condition
            Then exit("Early return for `what`")
        ...
    ...

# if you are running out of space, try using parentheses.
if
(       Some Long Condition
    &&  Some Other_fact
    &&  Need_this Too
) -> Then:
    print("good")
    ...

# of you can just use double indents:
if Some Long Condition
    &&  Some Other_fact
    &&  Need_this Too
->      Then:
    print("good")
    ...
```

## if statements

```
X: if Condition
    do_something()
elif Other_condition
    do_something_else()
else
    calculate_side_effects(...) # ignored for setting X
    Default_value

# now X is either the result of `do_something()`, `do_something_else()`,
# or `Default_value`.  note, we can also do this with braces to indicate
# blocks, and can fit in one line if we have fewer conditions, e.g.,

Y: if Condition {do_something()} else {calculate_side_effects(...), Default_value}
```

Note that ternary logic short-circuits operations, so that calling the function
`do_something()` only occurs if `Condition` is true.  Also, only the last line
of a block can become the RHS value for a statement like this.

TODO: more discussion about how `return` works vs. putting a RHS statement on a line.

Of course you can get two values out of a conditional expression, e.g., via destructuring:

```
[X, Y]: if Condition
    [X: 3, Y: do_something()]
else
    [X: 1, Y: Default_value]
```

Note that indent matters quite a bit here.  Conditional blocks are supposed to indent
at +1 from the initial condition (e.g., `if` or `else`), but the +1 is measured from
the line which starts the conditional (e.g., `[X, Y]` in the previous example).  Indenting
more than this would trigger line continuation logic.  I.e., at +2 or more indent,
the next line is considered part of the original statement and not a block.  For example:

```
# WARNING, PROBABLY NOT WHAT YOU WANT:
Q?: if Condition
        What + Indent_twice
# actually looks to the compiler like:
Q?: if Condition What + Indent_twice
```

Which will give a compiler error since there is no internal block for the `if` statement.

### if without else

You can use the result of an `if` expression without an `else`, but the resulting
variable becomes nullable, and therefore must be defined with `?:` (or `?;`).

```
greet(): str
    "hello, world!"

Result?: if Condition { greet() }
```

This also happens with `elif`, as long as there is no final `else` statement.


### is operator

You can use the `is` operator to convert statements like `X is(fn(Another_type: ...): ...)`
into more idiomatic things like `if X is Another_type: ...`.

```
# not idiomatic:
my_decider(X: one_of[type1, type2]):
    X is
    (   fn(Type1):
            print("X was type1: ", Type1)
    )
    # or using lambda functions:
    X is
    (   print("X was type2: ", $Type2)
    )

# idiomatic:
my_decider(X: one_of[type1, type2]):
    if X is Type1:
        print("X was type1: ", Type1)
    elif X is Type2:
        print("X was type2: ", Type2)
```

This is how you might declare similar functionality for your own class,
by overloading the `is` operator.

```
example_class: [Value: int]
{   # the standard way to use this method uses syntax sugar:
    #   if Example_class is Large:
    #       print("was large: ${Large}")
    :;.is(If_block[declaring: (Large:;. int), ~t]): never
        if Value > 999
            If_block then(Declaring: (Large` Value))
        else
            If_block else()

    # another way to do this
    :;.is(then(Large:;. int): ~t, else(): ~u): flatten[t, u]
        if M Value > 999
            then(Large` M Value)
        else
            # TODO: can we distinguish `else` from `else()` because we always
            #       require parentheses around functions now??
            else()
}
```

## what statements

`what` statements are comparable to `switch-case` statements in C/C++,
but in oh-lang the `case` keyword is not required.  You can use the keyword
`else` for a case that is not matched by any others, i.e., the default case.
You can also use `Any:;.` to match any other case, if you want access to the
remaining values.  (`else` is therefore like an `Any_:` case.)
We switch from the standard terminology for two reasons: (1) even though
`switch X` does describe that the later logic will branch between the different
cases of what `X` could be, `what X` is more descriptive as to checking what `X` is,
which is the important thing that `what` is doing at the start of the statement.
(2) `switch` is something that a class instance might potentially want to do,
e.g., `My_instance switch(Background1)`, and having `switch` as a keyword negates
that possibility.

TODO: explain how `case` values are cast to the same type as the value being `what`-ed.

You can use RHS expressions for the last line of each block to return a value
to the original scope.  In this example, `X` can become 5, 7, 8, or 100, with various
side effects (i.e., printing).

```
X: what String
    "hello"
        print("hello to you, too!")
        5
    # you can do multiple matches over multiple lines:
    "world"
    "earth"
        # String == "world" or "earth" here.
        print("it's a big place")
        7
    #or you can do multiple matches in a single line with commas:
    "hi", "hey", "howdy"
        # String == "hi", "hey", or "howdy" here.
        print("err, hi.")
        8
    else
        100

# Note again that you can use braces to make these inline.
# Try to keep usage to code that can fit legibly on one line:
Y: what String { "hello" {5} "world" {7} else {100} }
```

You don't need to explicitly "break" a `case` statement like in C/C++.
Because of that, a `break` inside a `what` statement will break out of
any enclosing `for` or `while` loop.  This makes `what` statements more
like `if` statements in oh-lang.

```
Air_quality_forecast: ["good", "bad", "really bad", "bad", "ok"]
Meh_days; 0
Air_quality_forecast each Quality:
    what Quality
        "really bad"
            print("it's going to be really bad!")
            break   # stops `for` loop
        "good"
            print("good, that's good!")
        "bad"
            print("oh no")
        "ok"
            ++Meh_days
```

The `what` operation is also useful for narrowing in on `one_of` variable types.
E.g., suppose we have the following:

```
status: one_of[Unknown, Alive, Dead]
vector3: [X; dbl, Y; dbl, Z; dbl]
{   ::length(): sqrt(X^2 + Y^2 + Z^2)
}

update: one_of
[   status
    position: vector3
    velocity: vector3
]
# example usage of creating various `update`s:
Update0: update status Alive
Update1: update position(X: 5.0, Y: 7.0, Z: 3.0)
Update2: update velocity(X: -3.0, Y: 4.0, Z: -1.0)
```

We can determine what the instance is internally by using `what` with
variable declarations that match the `one_of` type and field name.
We can do this alongside standard `switch`-like values, like so,
with earlier `what` cases taking precedence.  

```
...
# checking what `Update` is:
what Update
    # no trailing `:` because we're not declaring anything here:
    status Unknown
        print("unknown update")
    Status:
        # match all other statuses:
        print("got update: $(Status)")
    Position: vector3
        print("got update: $(Position)")
    Velocity: vector3
        print("got update: $(Velocity)")
```

We don't recommend function style here, e.g.,
`what Update { (Position: vector3): print("got position update: $(Position)") }`,
mostly because we want to allow `return` inside a `what` case to return
from the function that encloses `what`, and not `return` just inside the `what` case.
However, that's something we do support, in case your `what` case is complicated.

```
speed: one_of[
    None
    Slow
    Going_up
    Going_down
    Going_sideways
    Dead
]
check_speed(Update): speed
    what Update
        # you can mix and match non-function and function notation:
        status Dead
            Dead
        # here we use function notation:
        (Velocity: vector3):
            if Velocity length() < 5.0
                # this returns early from the `what` case
                return Slow
            print("going slow, checking up/down")
            if Velocity Y abs() < 0.2
                Going_sideways
            elif Velocity Y > 0.0
                Going_up
            else
                Going_down
        Position: where Position length() is_nan()
            Dead
        else
            None
```

Note that variable declarations can be argument style, i.e., including
temporary declarations (`.`), readonly references (`:`), and writable
references (`;`), since we can avoid copies if we wish.  This is only
really useful for allocated values like `str`, `int`, etc.  However, note
that temporary declarations via `.` can only be used if the argument to
`what` is a temporary, e.g., `what My_value!` or `what Some_class value()`.
There is no need to pass a value as a mutable reference, e.g., `what My_value;`;
since we can infer this if any internal matching block uses `;`.

```
whatever: one_of
[   str
    card: [Name: str, Id: u64]
]

Whatever; whatever str("this could be a very long string, don't copy if you don't need to")

what Whatever!      # ensure passing as a temporary by mooting here.
    Str.
        print("can do something with temporary here: ${Str}")
        do_something(Str!)
    Card.
        print("can do something with temporary here: ${Card}")
        do_something_else(Card!)
```

### where operator

The `where` operator can be used to further narrow a conditional.  It
is typically used in a `what` statement like this:

```
cows: one_of
[   One
    Two
    many: i32
]
Cows: some_function_returning_cows()
what Cows
    One
        print("got one cow")
    Two
        print("got two cows")
    Many: where Many <= 5       # optionally `Many: i32 where Many <= 5`
        print("got a handful of cows")
    Many:                       # optionally `Many: i32`
        print("got ${Many} cows")
```

It can also be used in a conditional alongside the `is` operator.
Using the same `cows` definition from above for an example:

```
Cows: some_function_returning_cows()
if Cows is Many: where Many > 5
    # executes if `Cows` is `cows many` and `Many` is 6 or more.
    print("got ${Many} cows")
else
    # executes if `Cows` is something else.
    print("not a lot of cows")
```

`where` is similar to the [`Require`](#require) field, but
`Require` needs to be computable at compile-time, and `where`
can be computed at run-time.

### what operator

Can we overload the `what` operator?  oh-lang would like to avoid walling off
parts of the code that you can't touch.  We'd need to figure out a good syntax
here.

```
my_vec2: [X; dbl, Y; dbl]
{   ::what
    (   do(QuadrantI. dbl): ~a
        do(QuadrantII. dbl): ~b
        do(QuadrantIII. dbl): ~c
        do(QuadrantIV. dbl): ~d
        else(): ~e
    ): flatten[a, b, c, d, e]
        if X == 0.0 or Y == 0.0
            else()
        elif X > 0.0
            if Y > 0.0
                do(QuadrantI. +X + Y)
            else
                do(QuadrantIV. +X - Y)
        else
            if Y > 0.0
                do(QuadrantII. -X + Y)
            else
                do(QuadrantIII. -X - Y)
}
```


### what implementation details

```
# The implementation can be pretty simple.
switch (Update.hm_Is)
{   case update::status::hm_Is:
        DEFINE_CAST(update::status *, Status, &Update.hm_Value);
        if (*Status == status::Unknown)
        {   // print("unknown update")
            ...
        }
        else
        {   // print("known status: ${Status}")
            ...
        }
        break;
    case update::position::hm_Is:
        DEFINE_CAST(update::position *, Position, &Update.hm_Value);
        // print("got position update: ${Position}")
        break;
    ...
}
```

Implementation details for strings: at compile time we do a fast hash of each 
string case, and at run time we do a switch-case on the fast hash of the considered
string.  (If the hashes of any two string cases collide, we redo all hashes with a
different salt.)  Of course, at run-time, there might be collisions, so before we
proceed with a match (if any), we check for string equality.  E.g., some pseudo-C++
code:

```
switch (fast_hash(Considered_string, Compile_time_salt))
{   case fast_hash(String_case1, Compile_time_salt): // precomputed with a stable hash
    {   if (Considered_string != String_case1)
        {   goto __Default__;
        }
        // logic for String_case1...
        break;
    }
    // and similarly for other string cases...
    default:
    {   // Locating here so that we can also get no-matches from hash collisions:
        __Default__:
        // logic for no match
    }
}
```

TODO: do we even really want a `fall_through` keyword?  it makes it complicated that it
will essentially be a `goto` because fall through won't work due to the check for string
equality.

TODO: we probably don't want to pollute code with compiler optimizations.  probably should
wait to include this and see if we actually need it for performance.

We'll add a compiler hint with the best compile-time `Salt` to the `what` statement,
so that future transpilations are faster.  The compiler will still try more salts
if the chosen salt doesn't work, however, e.g., in the situation where new cases
were added to the `what` statement.

```
X: what String    #salt(1234)
    "hello"
        print("hello to you, too!")
        5
    "world"
        print("it's a big place")
        7
    else
        100
```

Similarly, any class that supports a compile-time fast hash with a salt can be
put into a `what` statement.  Floating point classes or containers thereof
(e.g., `dbl` or `array[flt]`) are not considered *exact* enough to be hashable, but
oh-lang will support fast hashes for classes like `int`, `i32`, and `array[u64]`,
and other containers of precise types, as well as recursive containers thereof.

```
# note it's not strictly necessary to mention you implement `hashable`
# if you have the correct `hash` method signature.
my_hashable_class: all_of[hashable, m: [Id: u64, Name; string]]
{   # we allow a generic hash builder so we can do cryptographically secure hashes
    # or fast hashes in one definition, depending on what is required.
    # This should automatically be defined for classes with precise fields (e.g., int, u32, string, etc.)!
    ::hash(~Builder;):
        Builder hash(Id)    # you can use `hash` via the builder or...
        Name hash(Builder;) # you can use `hash` via the field.

    # equivalent definition via sequence building:
    ::hash(~Builder;): Builder@
    {   hash(Id)
        hash(Name)
    }
}

# note that defining `::hash(~Builder;)` automatically defines a `fast_hash` like this:
# fast_hash(My_hashable_class, ~Salt): salt
#   Builder: \\hash fast(Salt)
#   Builder hash(My_hashable_class)
#   return Builder build()

My_hashable_class: my_hashable_class(Id: 123, Name: "Whatever")

what My_hashable_class
    my_hashable_class(Id: 5, Name: "Ok")
        print("5 OK")
    my_hashable_class(Id: 123, Name: "Whatever")
        print("great!")
    My_hashable_class:
        print("it was something else: ${My_hashable_class}")
```

Note that if your `fast_hash` implementation is terrible (e.g., `fast_hash(Salt): Salt`),
then the compiler will error out after a certain number of attempts with different salts.

For sets and lots, we use a hash method that is order-independent (even if the container
is insertion-ordered).  E.g., we can sum the hash codes of each element, or `xor` them.
Arrays have order-dependent hashes, since `[1, 2]` should be considered different than `[2, 1]`,
but the lot `["hi": 1, "hey": 2]` should be the same as `["hey": 2, "hi": 1]` (different
insertion order, but same contents).

## for-each loops

TODO: Can we write other conditionals/loops/etc. in terms of `indent/block` to make it easier to compile
from fewer primitives?  E.g., `while Condition -> Do: {... Do exit(3) ...}`, where
`do` is a thin wrapper over `block`?  or maybe `do -> Loop: {... Loop exit(3) ...}`

oh-lang doesn't have `for` loops but instead uses `each` syntax on an iterator.
The usual syntax is `Iterator each Iterand;:. {do_something(Iterand)}`.  If your
iterand variable is already defined, you should use `Iterator each Iterand {...}`.
Note that all container classes have an `each` method defined, and some
"primitive" classes like the `count` class do as well.

```
# iterating from 0 to `Count - 1`:
Count: 5
Count each Int:
    print(Int)  # prints 0 to 4 on successive lines

# iterating over a range:
range(1, 10) each Int:
    print(Int)  # prints 1 to 9 on successive lines.

# iterating over non-number elements:
vector2: [X: dbl, Y: dbl]
Array[vector2]: [[X: 5, Y: 3], [X: 10, Y: 17]]

Array each Vector2:
    print(Vector2)

# if the variable is already declared, you avoid the declaration `:` or `;`:
# NOTE the variable should be writable!
Iterating_vector; vector2
Array each Iterating_vector
    print(Iterating_vector)
# this is useful if you want to keep the result of the last element outside the for-loop.
```

You can get the result of an `each` operation but this only really
makes sense if the `each` block has a `break` command in it.
Like `return`, `break` can pass a value back.

```
# Result needs to be nullable in case the iteration doesn't break anything.
Result?: range(123) each Int:
    if Int == 120
        break Int

# you can use an `else` which will fire if the iterator doesn't have
# any values *or* if the iteration never hit a `break` command.
# in this case, `Result` can be non-null.
Result: range(123) each Int:
    if Int == 137
        break Int
else
    44
```

Of course, you can use the `else` block even if you don't capture a result.

```
range(123) each Int:
    print(Int)
    if Int == 500
        break
else
    print("only fires if `break` never occurs")
```

Here are some examples of iterating over a container while mutating some values.

```
A_array; array[int] = [1, 2, 3, 4]
# this is clearly a reference since we have `Int` in parentheses, `(Int;)`:
A_array each(Index, Int;)
    Int += Index
A_array == [1, 3, 5, 7] # should be true

B_array; array[int] = [10, 20, 30]
B_array each(Int;)
    Int += 1
B_array == [11, 21, 31] # should be true

C_array; array[int] = [88, 99, 110]
Start_referent; int = 77
(Iterand_value;) = Start_referent
C_array each Iterand_value  # TODO: do we need `each(Iterand_value)` here??
    Iterand_value -= 40
C_array == [48, 59, 70] # should be true
```

You should be careful not to assume that `;` (or `:`) means a reference
unless the RHS of the `each` is wrapped in parentheses.

```
B_array; array[int] = [10, 20, 30]
# WARNING! this is not the same as the previous `B_array` logic.
B_array each Int;
    # NOTE: `Int` is a mutable copy of each value of `B_array` element.
    Int += 1
    # TODO: we probably should have a compile error here since
    #       `Int` is not used to affect anything else and
    #       `Int;` here is *NOT* a reference to the `C_array` elements.
    #       e.g., "use (Int;) if you want to modify the elements"
B_array == [11, 21, 31] # FALSE

C_array; array[int] = [88, 99, 110]
Iterand_value; 77 
C_array each Iterand_value
    # NOTE: `Iterand_value` is a mutable copy of each `C_array` element.
    Iterand_value -= 40
C_array == [48, 59, 70] # FALSE
C_array == [88, 99, 110] # true, unchanged.
Iterand_value == 70 # true
```

TODO: there may be some internal inconsistency here.  let's make sure
the way we define `;;each(...)` makes sense for the non-parentheses
case and the parentheses case.

TODO: we need to discuss the same for `if Result is (Ok;)`, etc.,
vs. `if Result is Ok;`.  The latter is a copy, the former is a no-copy reference.

# printing and echoing output

TODO: allow tabbed print output.  instead of searching through each string,
maybe we just look at `print` and add the newlines at the start.  Each thread should
have its own tab stop.  E.g.,

```
array[of]: []
{   ...
    ::print(): null
        if count() == 0
            return print("[]")
        print("[")
        with indent():
            each Of:
                print(Of)
        print("]")
}
```

TODO: defining `print` on a class will also define the `string()` method.
essentially any `print`s called inside of the class `print` will be redirectable to
a string-stream, etc.  `indent` should maybe do something different for `string()`;
maybe just add commas *after* the line instead of spaces before the line to be printed.

TODO: we should also have a print macro here in case we want to stop printing,
e.g., in case the string buffer is full (e.g., limited output).  if the print
command triggers a stop at any point, then abort (and stop calling the method)

## blocks

You can write your own `assert` or `return`-like statements using `block` logic.  The `block`
class has a method to return early if desired.  Calling `Block exit(...)` shortcircuits the
rest of the block (and possibly other nested blocks).  This is annotated by using the `jump`
return value.  You can also call `Block loop()` to return to the start of the block.
You don't usually create a `block` instance; you'll use it in combination with the global
`indent` function.

```
# indent function which returns whatever value the `Block` exits the loop with.
indent(fn(Block[~t]): never): t
# indent function which populates `Block Declaring` with the value passed in.
indent(~Declaring., fn(Block[~t, declaring]): never): t

@referenceable_as(then)
block[of, declaring: null]:
[   # variables defined only for the lifetime of this block's scope.
    # TODO: give examples, or maybe remove, if this breaks cleanup with the `jump` ability
    Declaring` declaring
]
{   # exits the `indent` with the corresponding `of` value.  example:
    #   Value; 0
    #   what indent
    #   (   fn(Block[str]): never
    #           @Old Value: Value
    #           Value = Value // 2 + 9
    #           # sequence should be: 0, 9, 4+9=13, 6+9=15, 7+9=16, 8+9=17
    #           if @Old Value == Value
    #               Block exit("exited at ${@Old Value}")
    #           # note we need to `loop` otherwise we won't satisfy the `never`
    #           # part of the indent function.
    #           Block loop()
    #   )
    #       Str.
    #           print(Str)       # should print "exited at 17"
    ::exit(Of.): jump

    # like a `continue` statement; will bring control flow back to
    # the start of the `indent` block.  example:
    #   Value; 0
    #   indent
    #   (   fn(Block[str]):
    #           if ++Value >= 10 {Block exit("done")}
    #           if Value % 2
    #               Block loop()
    #           print(Value)
    #           Block loop()
    #   )
    #   # should print "2", "4", "6", "8"
    @hide_from(then)
    ::loop(): jump
}
```

### blocks to define a variable

```
My_int: indent
(   Block[int]:
        if some_condition()
            Block exit(3)
        Block loop()
)
```

### then with blocks

When using `then`, it's recommended to always exit explicitly, but like with the
non-`then` version, the conditional block will exit with the value of the last
executed line.  There is a rawer version of this syntax that does require an
explicit exit, but also doesn't allow any `return` functions since we are literally
defining a `(Then): never` with its block inline.  This syntax is not recommended
unless you have identical block handling in separate conditional branches, but
even that probably could be better served by pulling out a function to call in
both blocks.

```
if Some_condition -> Then[str]:
    if Other_condition
        if Nested_condition
            Then exit(X)
    else
        Then exit("whatever")
    # COMPILE ERROR, this function returns here if
    # `Other_condition && !Nested_condition`.

# here's an example where we re-use a function for the block.
My_then: then[str]
    ... complicated logic ...
    exit("made it")

Result: if Some_condition -> My_then
elif Some_thing_else
    print("don't use `My_then` here")
    "no"
else -> My_then
```

### function blocks

Similar to conditionals, we allow defining functions with `block` in order
to allow low-level flow control.  Declarations like `my_function(X: int): str`,
however, will be equivalent to `my_function(X: int, Block[str]): never`.  Thus
there is no way to overload a function defined with `block` statements compared
to one that is not defined explicitly with `block`.

```
# the `never` return type means that this function can't use `return`, either
# explicitly via `return ...` or implicitly by leaving a value as the last
# evaluated statement (which can occur if you don't use `Block exit(...)`
# or `Block loop(...)` on the last line of the function block).
# i.e., you must use `Block exit(...)` to return a value from this function.
my_function(X: int, Block[str]): never
    inner_function(Y: int): dbl
        if Y == 123
            Block exit("123")    # early return from `my_function`
        Y dbl() ?? panic()
    range(X) each Y:
        inner_function(Y)
    Block exit("normal exit")
```

## coroutines

We'll reserve `co[of]` for a coroutine for now, but I think futures are all
that is necessary.

# futures

oh-lang wants to make it very simple to do async code, without additional
metadata on functions like `async` (JavaScript).  You can indicate that
your function takes a long time to run by returning the `um[of]` type,
where `of` is the type that the future `um` will resolve to, but callers
will not be required to acknowledge this.  If you define some overload
`my_overload(X: str): um[int]`, an overload `my_overload(X: str): int`
will be defined for you that comes *before* your async definition, so that
the default type of `Value` in `Value: my_overload(X: "asdf")` is `int`.
We generally recommend a timeout `er` being present, however, so for
convenience, we define `um[of, er]: um[hm[ok: of, er]]`.

The reason we obscure futures in this way is to avoid needing to change any
nested function's signatures to return futures if an inner function becomes
a future.  If the caller wants to treat a function as a future, i.e., to run
many such futures in parallel, then they ask for it explicitly as a future
by calling a function `f()` via `f() Um`, which returns the `um[of]` type,
where `of` is the default overload's return type.  You can also type the
variable explicitly as `um[of]`, e.g., `F: um[of] = f()`.  Note that
`F: um(f())` is a compile error because casting to a future would still run
`f()` serially.  You can use `F: um(Immediate: 123)` to create an "immediate
future"; `F: um(Immediate: h())` similarly will run `h()` serially and put
its result into the immediate future.  If `h` takes a long time to run, prefer
`F: h() Um` of course.

```
# you don't even need to type your function as `um[~]`, but it's highly recommended:
some_very_long_running_function(Int): um[string]
    Result; ""
    range(Int) each @New Int:
        sleep(Seconds: @New Int)
        Result += str(@New Int)
    Result

# this approach calls the default `string` return overload, which blocks:
print("starting a long running function...")
My_name: some_very_long_running_function(10)
print("the result is ${My_name} many seconds later")

# this approach calls the function as a future:
print("starting a future, won't make progress unless polled")
# `Future` here has the type `um[string]`:
Future: some_very_long_running_function(10) Um
# Also ok: `Future: um[string] = some_very_long_running_function(10)`
# Also ok: `Future: um[~] = some_very_long_running_function(10)` (infers the inner type)
# Also ok: `Future: um[~inner] = some_very_long_running_function(10)` (infers inner type and gives it a name)
# which is useful if you want to use the `inner` type later in this block.
print("this `print` executes right away")
Result: string = Future
print("the result is ${Result} many seconds later")
```

That is the basic way to resolve a future, but you can also use
the `::decide(): of` method for an explicit conversion from `um[of]`
to `of`.  Ultimately futures are more useful when combined for
parallelism.  Here are two examples, one using an array of futures
and one using an object of futures:

```
# you don't even need to type your function as `um[~]`, but it's highly recommended:
after(Seconds: int, Return: string): um[string]
    sleep(Seconds)
    Return

Futures_array; array[um[string]]
# no need to use `after(...) Um` here since `Futures_array`
# elements are already typed as `um[string]`:
Futures_array append(after(Seconds: 2, Return: "hello"))
Futures_array append(after(Seconds: 1, Return: "world"))
print("this executes immediately.  deciding futures now...")
Results_array: decide(Futures_array)
print(Results_array) # prints `["hello", "world"]` after 2ish seconds.

# here we put them all in at once.
# notice you can use `Field: um[type] = fn()` or `Field: fn() Um`.
Futures_object:
[   Greeting: after(Seconds: 2, Return: "hello") Um
    Noun: um[string] = after(Seconds: 1, Return: "world")
]
print(decide(Futures_object)) # prints `[Greeting: "hello", Noun: "world"]`

# if your field types are already futures, you don't need to be
# explicit with `Um`.
future_type: [Greeting: um[str], Noun: um[str]]
# note that we need to explicitly type this via `the_type(Args...)`
# so that the compiler knows that the arguments are futures and should
# receive the `um` overload.
Futures: future_type
(   Greeting: after(Seconds: 2, Return: "hi")
    Noun: after(Seconds: 1, Return: "you")
)
# this whole statement should take ~2s and not ~3s; the two fields are
# initialized in parallel.
Futures decide() print()    # prints `[Greeting: "hi", Noun: "you"]`
```

Notice that all containers with `um` types for elements will have
an overload defined for `decide`, which can be used like with the
`Futures_array` example above.  Similarly all object types with `um`
fields have a `decide` function that awaits all internal fields that
are futures before returning.  You can also use `Container decide()`
instead of `decide(Container)` in case that makes more sense.

We will also include a compile error if something inside a futures
container is defined without `um`:

```
# if any field in an object/container is an `um` class, we expect everyone to be.
# this is to save developers from accidentally forgetting an `Um`
Object_or_container:
[   Greeting: after(Seconds: 2, Return: "hello")    # COMPILE ERROR!
    Noun: after(Seconds: 1, Return: "world") Um     # ok
]
```

If you do need to pass in an immediate future as a container element
(e.g., to simplify the API when calling with different conditions),
use `um(Immediate: ...)` to make it clear that you want it that way.

# enums and masks

## enumerations

We can create a new type that exhaustively declares all possible values it can take.
The syntax is `type_case: one_of` followed by a list of named values
(each an `Variable_case` identifier), with optional values they take, or subtypes
(each a `type_case` identifier) with their corresponding type definitions.  Enumerations
are mutually exclusive -- no two values may be held simultaneously.  See
masks for a similar class type that allows multiple options at once.

Enums are by default the smallest standard integral type that holds all values,
but they can be signed types (in contrast to masks which are unsigned).
If desired, you can specify the underlying enum type using `one_of i8[...]` instead
of `one_of[...]`, but this will be a compile error if the type is not big enough to
handle all options.  It will not be a compile warning if the `one_of` includes types
inside (e.g., `one_of i8[u32, f32]`); we'll assume you want the tag to be an `i8`.
However, it should be clear that the full type will be at least the size of the
tag plus the largest element in the `one_of`; possibly more to alikely chieve alignment.

Here is an example enum with some values that aren't specified.  Even though
the values aren't specified, they are deterministically chosen.

TODO: to be consistent, if we're "defining" something, we should use `:`.
should we use `one_of[First_value_defaults_to_zero:, Second_value_increments:, ...]`?
depends on if we want to require it always with function arguments and with brackets.

```
my_enum: one_of
[   First_value_defaults_to_zero
    Second_value_increments
    Third_value_is_specified: 123
    Fourth_value_increments
]
assert my_enum First_value_defaults_to_zero == 0
assert my_enum Second_value_increments == 1
assert my_enum Third_value_is_specified == 123
assert my_enum Fourth_value_increments == 124
```

You can even pass in existing variable(s) to the enum, although they should be
compile-time constants.  This uses the same logic as function arguments to
determine what the name of the enum value is.

```
Super: 12
Crazy: 15
# the following will define
# `other_enum Other_value1 = 0`,
# `other_enum Super = 12`,
# and `other_enum Other_value2 = 15`.
other_enum: one_of
[   Other_value1
    Super
    Other_value2: Crazy
]
```

Here is an example enum with just specified values, all inline:

```
# fits in a `u1`.
bool: one_of[False: 0, True: 1]
```

Enums provide a few extra additional methods for free as well, including
the number of values that are enumerated via the class function `count(): count_arch`,
and the min and max values `min(): enum_type`, `max(): enum_type`.  You can also
check if an enum instance `Enum` is a specific value `This_value` via
`Enum is_this_value()` which will return true iff so.

```
Test: bool = False  # or `Test: bool False`

if Test == True     # OK
    print("test is true :(")
if Test is_false()   # also OK
    print("test is false!")

# get the count (number of enumerated values) of the enum:
print("bool has ${bool count()} possibilities:")
# get the lowest and highest values of the enum:
print("starting at ${bool min()} and going to ${bool max()}")
```

Because of this, it is a bit confusing to create an enum that has `Count` as an
enumerated value name, but it is not illegal, since we can still distinguish between the
enumerated value (`enum_name Count`) and total number of enumerated values (`enum_name count()`).
Similarly for `Min`/`Max`.

Also note that the `count()` method will return the total number of
enumerations, not the number +1 after the last enum value.  This can be confusing
in case you use non-standard enumerations (i.e., with values less than 0):

```
sign: one_of
[   Negative: -1
    Zero: 0
    Positive: 1
]

print("sign has ${sign count()} values")   # 3
print("starting at ${sign min()} and going to ${sign max()}")     # -1 and 1

weird: one_of
[   X: 1
    Y: 2
    Z: 3
    Q: 9
]

print(weird count())   # prints 4
print(weird min())     # prints 1
print(weird max())     # prints 9
```

### default values for a `one_of`

Note that the default value for a `one_of` is the first value, unless zero is an option
(and it's not the first value), unless `null` is an option -- in increasing precedence.
E.g., `one_of[Option_a, Option_b]` defaults to `Option_a`, `one_of[A: -1, B: 0, C: 1]` 
defaults to `B`, and `one_of[Option_c, Null]` defaults to `Null`, and
`one_of[A: -1, B: 0, C: 1, Null]` also defaults to `Null`.
TODO: `[Null]` should collapse to `[]` based on how containers work.  Null is
always the absence of a value and should never be considered present.  we can make
`one_of` a macro but that might be a pain to be consistent (and use `@one_of` everywhere).
maybe require using `?` for these sorts of things
like `tye_type: one_of[A: -1, B: 0, C: 1]?` or `the_type?: one_of[A: -1, B: 0, C: 1]`
otherwise `one_of[a, b, c, null]` is ok because `null` is a type.

### testing enums with lots of values

Note that if you are checking many values, a `what` statement may be more useful
than testing each value against the various possibilities.  Also note that you don't need
to explicitly set each enum value; they start at 0 and increment by default.

```
option: one_of
[   Unselected
    Not_a_good_option
    Content_with_life
    Better_options_out_there
    Best_option_still_coming
    Oops_you_missed_it
    Now_you_will_be_sad_forever
]

print("number of options should be 7:  ${option count()}")

Option1: option Content_with_life

# avoid doing this if you are checking many possibilities:
if Option1 is_not_a_good_option()   # OK
    print("oh no")
elif Option1 == Oops_you_missed_it # also OK
    print("whoops")
...

# instead, consider doing this:
what Option1
    Not_a_good_option
        print("oh no")
    Best_option_still_coming
        print("was the best actually...")
    Unselected
        fall_through
    else
        print("that was boring")
```

Note that we don't have to do `option Not_a_good_option` (and similarly for other options)
along with the cases.  The compiler knows that since `Option1` is of type `option`,
that you are looking at the different values for `option` in the different cases.

## one_of types

TODO: what's the difference between `one_of[Dbl, Int]` and `one_of[dbl, int]`?
probably nothing??  but `one_of[New_identifier: 0, Other_identifier: 3]` would
be different than `one_of[new_identifier: 0, other_identifier: 3]`? or not??
in both cases, it seems like `0` and `3` are specifying the tag.  but would
`one_of[new_id: [X: dbl], other_id: [Y: str]]` be different than
`one_of[New_id: [X: dbl], Other_id: [Y: str]]`?...  maybe we just force lowercase.
TODO: discuss things like `one_of[1, 2, 5, 7]` in case you want only specific instances.

Nulls are
highly encouraged to come last in a `one_of`, because they will match any input, so
casting like this: `one_of[null, int](1234)` would actually become `Null` rather than
the expected value `1234`, since casts are attempted in order of the `one_of` types.

Take this example `one_of`.

```
tree: one_of
[   leaf: [Value; int]
    branch:
    [   Left; tree
        Right; tree
    ]
]
```

When checking a `tree` type for its internal structure, you can use `is_leaf()` or `is_branch()`
if you just need a boolean, but if you need to manipulate one of the internal types, you should
use `::is(fn(Internal_type): null): bool` or `;;is(fn(Internal_type;): null): bool` if you need to modify
it, where `internal_type` is either `leaf` or `branch` in this case.  For example:

```
Tree; tree = if Abc
    leaf(Value: 3)
else
    branch(Left: leaf(Value: 1), Right: leaf(Value: 5))

if Tree is_leaf()
    # no type narrowing, not ideal.
    print(Tree)

# narrowing to a `leaf` type that is readonly, while retaining a reference
# to the original `Tree` variable.  the nested function only executes if
# `Tree` is internally of type `leaf`:
Tree is(fn(Leaf): print(Leaf))

# narrowing to a `branch` type that is writable.  `Tree` was writable, so `Branch` can be.
# the nested function only executes if `Tree` is internally of type `branch`:
Tree is
(   fn(Branch;):
        print(Branch Left, " ", Branch Right)
        # this operation can affect/modify the `Tree` variable.
        Branch Left some_operation()
)
```

Even better, use the [`is` operator](#is-operator) and define a block:

```
# you can also use this in a conditional; note we don't wrap in a lambda function
# because we're using fancier `Block` syntax.
if Tree is Branch;
    Branch Left some_operation()
    print("a branch")
else
    print("not a branch") 
```

If you need to manipulate most of the internal types, use `what` to narrow the type
and handle all the different cases.

```
what Tree
    Leaf: 
        print(Leaf)
    Branch;
        # this operation can affect/modify the `Tree` variable.
        print(Branch Left, " ", Branch Right)
        Branch Left some_operation()
```

Since function arguments are references by default, the above options are useful
when you want to modify the `Tree` variable by changing its internals whether
selectively it's a `leaf` or a `branch`.  If you want to make a copy, you can do so
via type casting: `New_leaf?; leaf = Tree` or `My_branch?; branch = Tree`; these
variables will be null if the `Tree` is not of that type, but they will also be
a copy and any changes to the new variables will not be reflected in `Tree`.

```
one_of[..., t]: []
{   # returns true if this `one_of` is of type `T`, also allowing access
    # to the underlying value by passing it into the function.
    # we return `never` here because we don't want people to use the
    # value and expect it to return something based on the callback's return type,
    # or be confused if it should always return true if the internal type is `t`.
    ;:is(fn(T;:): null): never

    # type narrowing.
    # the signature for `if Tree is Branch; {#[do stuff with `Branch`]#}`
    # the method returns true iff the block should be executed.
    # the block itself can return a value to the parent scope.
    ;:.is(), Block[declaring:;. t, exit: ~u]: bool
}
```

### flattening and containing

Note that `one_of[one_of[a, b], one_of[c, d]]` is not the same as
`one_of[a, b, c, d]`.  To get that result, use `flatten`, e.g.,
`flatten[one_of[a, b], one_of[c, d]]` will equal `one_of[a, b, c, d]`.
This is safe to use on other types, so `flatten[one_of[c, d], e]`
is `one_of[c, d, e]`.

If you want to check if a condition is true for a type,
you can use notation like `x is one_of[a, b]`.  This is true if `x`
is `a`, `b`, or even `one_of[a, b]`, and false otherwise.
You can also use `contains` to go the opposite direction, e.g.,
`one_of[a, b] contains(x)` is the same as `x is one_of[a, b]`.
(Note we use parentheses in `contains(...)` because we return a value,
i.e., a boolean, not a type.)

### `one_of`s as function arguments

The default name for a `one_of` argument is `One_of`.  E.g.,

```
# this is short for `my_function(One_of: one_of[int, str]): dbl`:
my_function(One_of[int, str]): dbl
    dbl(One_of) ?? panic()

print(my_function(123))      # prints 123.0
print(my_function("123.4"))  # prints 123.4
```

Internally this creates multiple function overloads:  one for when the argument's
type is unknown at compile time, and one for each of the possible argument types
when it is known (e.g., `int` and `str` in this case).

If you need to use multiple `one_of`s in a function and still want them to be
default-named, it's recommended to give specific names to each `one_of`, e.g.,

```
int_or_string: one_of[int, str]
weird_number: one_of[u8, i32, dbl]

my_function(Int_or_string, Weird_number): dbl
    return dbl(Int_or_string) * Weird_number
```

However, you can also achieve the same thing using namespaces,
if you don't want to add specific names for the `one_of`s.

```
# arguments short for `@A One_of: one_of[int, str]` and `@B One_of: one_of[u8, i32, dbl]`.
my_function(@A One_of[int, str], @B One_of[u8, i32, dbl]): dbl
    return dbl(@A One_of) * @B One_of
```

Again, this fans out into multiple function overloads for each of the cases
of compile-time known and compile-time unknown arguments.

TODO: ensure that we can use `Call` or similar to create our own version
of a `one_of` with metaprogramming or whatever.

## select

If you *don't* want to allow the combined case as an argument, e.g., `one_of[a, b]`,
you can use `select[a, b]`.  This will only generate overloads with
the distinct types and not the combined type, like so:

```
my_fn(Select[int, str]): str
    "hello ${Select}"

# becomes only these two functions internally:
my_fn(Int): str
    "hello ${Int}"

my_fn(Str): str
    "hello ${Str}"

# when calling:
my_fn(5)            # OK
my_fn("world")      # OK
My_one_of; one_of[int, str](3)
# ... some logic that might change My_one_of
my_fn(My_one_of)    # COMPILE ERROR
```

This can be used to restrict generics to a list of only certain types,
e.g., `primitives: select[i8, i16], my_generic[of: primitives]: [Of]`
will only allow specification as `my_generic[i8]` or `my_generic[i16]`,
and not `my_generic[one_of[i8, i16]]`.

Like with `one_of`, you can check if a `select` contains some type by
using `contains`, e.g., `a_or_b: select[a, b], a_or_b contains(x)`
is true if `x` is `a` or `b`, but not `one_of[a, b]`.  Note that
`select` will expand so that `a_or_b contains(select[a, b])` could
be true or false depending on the situation.  In this case, it would
be a compile error because `select[a, b]` needs to be put into a named
type (e.g., `a_or_b: select[a, b]`) since it is a bit of a meta-type.

## masks

Masks are generalized from enumerations to allow multiple values held simultaneously.
Each value can also be thought of as a flag or option, which are bitwise OR'd together
(i.e., `|`) for the variable instance.  Under the hood, these are unsigned integer types.
Trying to assign a negative value will throw a compiler error.
Unlike enums which hold only `one_of` the fields at a time, masks hold `any_or_none_of`
which is a bit too verbose; we use `choose[A, B, C]` to declare a mask which can
be `A`, `B`, `C`, some combination of all three, or no values at all.

Like with enums, you can specify the integer type that backs the mask, but in this case
it must be an unsigned type, e.g., `choose u32[...]`.  Note that by default, the `mask_type`
is exactly as many bits as it needs to be to fit the desired options, rounded up to
the nearest standard unsigned type (`u8`, `u16`, `u32`, `u64`, `u128`, etc.).
We will add a warning if the user is getting into the `u128+` territory.

TODO: is there a good generalization of "any type in an enum" functionality
that rust provides, for masks?  e.g., `choose[a, b, c]` for types `a,b,c`?

Also like enums, masks don't need to specify their values; unlike enums, if you do specify them,
they must be powers of two.  Like enums, they have an `is_this_value()` method
for a `This_value` option, which is true if the mask is exactly equal to `This_value`
and nothing else.  You can use `has_this_value()` to see if it contains `This_value`,
but may contain other values besides `This_value`.

TODO: should this be `contains_this_value()` to be consistent with containers?

TODO: is there a way to make this `any_of` and use 0 as the `Null` value?

```
food: choose
[   Carrots
    Potatoes
    Tomatoes
]
# this creates a mask with `food Carrots == 1`,
# `food Potatoes == 2`, and `food Tomatoes == 4`.
# there is also a default `food None == 0`.

Food: food = Carrots | Tomatoes
Food has_carrots()   # true
Food has_potatoes()  # false
Food has_tomatoes()  # true
Food is_carrots()    # false, since `Food` is not just `Carrots`.
```

And here is an example with specified values.

```
# the mask is required to specify types that are powers of two:
non_mutually_exclusive_type: choose
[   X: 1
    Y: 2
    Z: 4
    T: 32
]
# `non_mutually_exclusive_type None` is automatically defined as 0.

# has all the same static methods as enum, though perhaps they are a bit surprising:
non_mutually_exclusive_type count() == 16
non_mutually_exclusive_type min() == 0
non_mutually_exclusive_type max() == 39   # = X | Y | Z | T

Options; non_mutually_exclusive_type()
Options == 0        # True; masks start at 0, or `None`,
                    # so `Options == None` is also true.
Options |= X        # TODO: make sure it's ok to implicitly add the mask type here.
                    #       maybe it's only ok if no `X` is in scope, otherwise
                    #       you need to make it explicit.
Options |= non_mutually_exclusive_type Z   # explicit mask type

Options has_x()  # True
Options has_y()  # False
Options has_z()  # True
Options has_t()  # False

Options = T
Options is_t()   # True
Options has_t()  # True
```

## interplay with `one_of`

We can also create a mask with one or more `one_of` fields, e.g.:

```
options: choose
[   one_of[Align_center_x, Align_left, Align_right]
    one_of[Align_center_y, Align_top, Align_bottom]

    one_of[Font_very_small, Font_small, Font_normal: 0, Font_large, Font_very_large]
]
```

It is a compiler error to assign multiple values from the same `one_of`:

```
Options; options = Align_center_x | Align_right     # COMPILER ERROR!
```

Note that internally, an `OR` combination of the `one_of` values may actually be valid;
it may be another one of the `one_of` values in order to save bits.  Otherwise, each
new value in the `one_of` would need a new power of 2.  For example, we can represent
`one_of[Align_center_x, Align_left, Align_right]` with only two powers of two, e.g.,
`Align_center_x = 4`, `Align_left = 8`, and `Align_right = 12`.  Because of this, there
is special logic with `|` and `&` for `one_of` values in masks.

```
Options2; options = Align_center_x
Options2 |= Align_right    # will clear out existing Align_center_x/Left/Right first before `OR`ing
if Options2 & Align_center_x
    print("this will never trigger even if Align_center_x == 4 and Align_right == 12.")
```

You can also explicitly tell the mask to avoid assigning a power of two to one of the
`one_of` values by setting it to zero (e.g., `one_of[..., Value: 0, ... ]`.
For example, the font size `one_of` earlier could be represented by 3 powers of two, e.g.,
`Font_very_small = 16`, `Font_small = 32`, `Font_large = 64`, `Font_very_large = 96`.
Note that we have the best packing if the number of non-zero values is 3 (requiring 2 powers of two),
7 (requiring 3 powers of two), or, in general, one less than a power of two, i.e., `2^P - 1`,
requiring `P` powers of two.  This is because we need one value to be the default for each
`one_of` in the `mask`, which will be all `P` bits set to zero; the remaining `2^P - 1`
combinations of the `P` bits can be used for the remaining `one_of` values.  A default
name can thus be chosen for each `one_of`, e.g., `one_of[..., Whatever_name: 0, ...]`.

## named value-combinations

You can add some named combinations by extending a mask like this.

```
my_mask: choose[X, Y]
{   X_and_y: X | Y
}

Result: my_mask = X_and_y
print(Result & X) # truthy, should be 1
print(Result & Y) # truthy, should be 2
```

# lifetimes and closures

## lifetimes of variables and functions

Variable and function lifetimes are usually scoped to the block that they
were defined in.  Initialization happens when they are encountered, and
descoping/destruction happens in reverse order when the block ends.  With
functions we have to be especially careful when they are impure.
If an impure function's lifetime exceeds that of any of its hidden
inputs' lifetimes, we'll get a segfault or worse.

Let's illustrate the problem with an example:

```
# define a re-definable function.
live_it_up(String); index
    return String count_bytes()

if Some_condition
    Some_index; index = 9
    # redefine:
    live_it_up(String); index
        return String count_bytes() + ++Some_index

    print(live_it_up("hi"))   # this should print 12
    print(live_it_up("ok"))   # this should print 13

print(live_it_up("no"))       # should this print 14 or 2??
```

Within the `if Some_condition` block, a new variable `Some_index` gets defined,
which is used to declare a new version of `live_it_up`.  But once that block
is finished, `Some_index` gets cleaned up without care that it was used elsewhere,
and if `live_it_up` is called with the new definition, it may segfault (or start
changing some other random variable's data).  Therefore, we must not allow the
expectation that `live_it_up("no")` will return 14 outside of the block.

We actually don't want `live_it_up("no")` to return 2 here, either; we want this
code to fail at compilation.  We want to detect when users are trying to do
closures (popular in garbage-collected languages) and let them know this is
not allowed; at least, not like this.

You can define variables and use them inside impure functions
but they must be defined *before* the impure function is first declared.  So this
would be allowed:

```
Some_index; index = 9
live_it_up(String); index
    return String count_bytes()

if Some_condition
    # redefine:
    live_it_up(String); index
        return String count_bytes() + ++Some_index

    print(live_it_up("hi"))   # this should print 12
    print(live_it_up("ok"))   # this should print 13

print(live_it_up("no"))       # prints 14
```

Alternatively, we allow an impure function to "take" a variable into
its own private scope so that we could redefine `live_it_up` here with a new
internal variable.  We still wouldn't allow references; they'd have to be
new variables scoped into the function block.

```
if Some_condition
    live_it_up(String); index
        # "@dynamic" means declare once and let changes persist across function invocations.
        # Note that this functionality cannot be used to store references.
        @dynamic X; int = 12345
        return String count_bytes() + ++X
```

Similarly, returning a function from within a function is breaking scope:

```
next_generator(Int; int): fn(): int
    return ():
        return ++Int
```

However, technically this is OK because arguments in functions are references,
and they should therefore be defined before this function is called.
Even if `Int;` here is a temporary, it is defined before this function
(and not deleted after the function call).

Here is an example where the return value is a function which uses
the another function from the input.  This is ok because the returned
function has internals that have greater lifetimes than itself; i.e.,
the input function will be destroyed only after the output function
is descoped.

```
# function that takes a function as an argument and returns a function
# example usage:
#   some_fn(): "hey"
#   # need to specify the overload
#   other_fn(): int = wow(fn(): str = some_fn)
#   print(other_fn()) # 3
wow(Input fn(): string): fn(): int
    (): int
        Input fn() count_bytes()
```

## handling system callbacks

We want to allow a `caller`/`callee` contract which enables methods defined
on one class instance to be called by another class instance, without being
in the same nested scope.  The `caller` which will call the callback needs
to be defined before the `callee`, e.g., as a singleton or other instance.
When the `callee` is descoped, it will deregister itself with the `caller`
internally, so that the `caller` will no longer call the `callee`.

```
callee[of]: []
{   ;;call(Of@): null

    ;;hang_up(): null
        ... # some internal implementation
}

caller[of]:
[   # use `of@` to pass in the mutability of `of` from `caller` into `callee`,
    Callees[ptr[callee[of@]]];
]
{   ::run_callbacks(Of@):
        Callees each Ptr: {Ptr call(Of@)}
}

audio: caller[array[sample], Mutable]
{   # this `audio` class will call the `call` method on the `callee` class.
    # TODO: actually show some logic for the calling.

    # amount of time between samples:
    Delta_t: flt

    # number of samples
    Count; 500
}

audio_callee: all_of
[   m: [Frequency; flt(440), Phase; flt]
    callee[array[sample];]
]
{   ;;call(Array[sample];): for Index: index < count(Array)
        Array[Index] = sample(Mono: \\math sin(2 * \\math Pi * Phase))
        Phase += Frequency * Audio Delta_t
}

some_function(): null
    Callee; audio_callee
    Callee Frequency = 880
    Audio call(Callee;)
    sleep(Seconds: 10)
    # `Audio hang_up(Callee;)` automatically happens when `Callee` is descoped.
```

# grammar/syntax

Note on terminology:

* Declaration: declaring a variable or function (and its type) but not defining its value

* Definition: declaration along with an assignment of the function or variable.

* `Identifier`: starts with an alphabetical character, can have numerical characters after that.
    Note that underscores are **not** permitted, since they are an operator.  

* `function_case`/`type_case`: Identifier which starts with a lowercase alphabetical character.

* `Variable_case`: Identifier which starts with an uppercase alphabetical character.

TODO: use an oh-lang version.
See [the grammar definition](https://github.com/hm-lang/core/blob/main/transpiler/grammar.hm).

# tokenizer

TODO

# transpiling

Every variable instance has two different storage layouts, one for "only type" and one for
"dynamic type."  "Only-type" variables require no extra memory to determine what type they are.
For example, an array of `i32` has some storage layout for the `array` type, but
each `i32` element has "only type" storage, which is 4 consecutive bytes, ensuring that the
array is packed tightly.  "Dynamic-type" variables include things like objects and instances
that could be one of many class types (e.g., a parent or a child class).  Because of this,
dynamic-type variables include a 64 bit field for their type at the start of the instance,
acting much like a vtable.  However, the type table includes more than just method pointers.

```
// C++ code
typedef u64 type_id;

struct variable_type
{   string Name;
    type_id Type_id;
};

typedef array<variable_type> arguments_type;

struct overload_type
{   type_id Instance_type_id; // 0 if this is not a method overload
    arguments_type Input;
    arguments_type Output;
};

struct overload
{   type_id Instance_type_id; // 0 if this is not a method overload
    arguments_type Input;
    arguments_type Output;

    void *Function_pointer;
};

struct overload_matcher
{   array<overload> Overloads;

    // TODO: can `reference` be a no-copy no-move type class?
    // TODO: can we make this an `array_element_reference` under the hood with type erasure?
    const_nullable_reference<overload> match(const overload_type &Overload_type) const;
};

// C++ code: info for a type
struct type_info
{   type_id Id;
    string Name;

    // Class types have variables, methods, and class functions defined in here:
    array<variable_type> Fields;

    // A function type_info should have nonempty Overloads:
    overload_matcher Overload_matcher;
};
```

TODO: storage for dynamic types, can we create wrapper classes with enough memory and
cast to them (i.e., without allocation)?  need to know the possible memory layouts beforehand,
i.e., all possible child classes.  since we should know
all imports ahead of time, however, we could just get the largest child implementation and use that.
(maybe check if one child uses a lot more memory than other siblings and push to a pointer.)
for scripts that extend a class, we might fit in as much as we can to the wrapper classes
memory but then use a pointer to the rest.  it's ok if scripts take a performance hit (pointer dereference).

TODO: a general purpose "part of your memory is here, part of your memory is there" class
almost sounds like virtual memory with mappings.  that should probably be non-standard/non-core.

TODO: discuss having all instance methods in some special virtual table, e.g., possibly 
with additional reflection information (for things like `@mutators(my_class) each(Callable;)`
macro code).
we also may need to have a fallback table for functions that are defined locally.
or ideally, we just rely on the global functions so we don't have to specify the vtable
(unless we're overriding things).

## object format

We'll use the following example oh-lang class and other functions for transpilation examples.

```
example_class: 
[   A; f32
    B; f32
    X; i32
    Y; i32
]
{   ;;renew(M X. i32, M Y. i32):
        A = X - Y
        B = X + Y

    ::readonly_method(Z. i32): i32
        X * Y - Z

    ;;writable_method(Q. f32): f32
        A *= Q
        B *= 1.0 / (1.0 + abs(Q))
        A * B
}

example_function(X: i64, A: dbl): [Y: i64, B: dbl]
    [Y: X - 1, B: A * X]
```

## C API

```
// example_class.h
typedef struct example_class
{   float A;
    float B;
    int32_t X;
    int32_t Y;
}   example_class_t;

typedef struct example_class_renew_input_X_Y
{   int32_t X;
    int32_t Y;
}   example_class_renew_input_X_Y_t;

typedef struct example_class_readonly_method_input_Z_t
{   int32_t Z;
}   example_class_readonly_method_input_Z_t;

typedef struct example_class_writable_method_input_Q_t
{   float Q;
}   example_class_writable_method_input_Q_t;

void example_class_renew_X_Y(example_class_t *M, example_class_renew_input_X_Y_t input);
int32_t example_class_readonly_method_input_Z_output_i32(
    const example_class_t *M, example_class_readonly_method_input_Z_t input
);
float example_class_writable_method_input_Q_output_f32(
    example_class_t *M, example_class_writable_method_input_Q_t input
);

typedef struct example_function_input_A_X_t
{   double A;
    int64 X;
}   example_function_input_A_X_t;

typedef struct example_function_output_B_Y_t
{   double B;
    int64 Y;
}   example_function_output_B_Y_t;

example_function_output_B_Y_t example_function_input_A_X_output_B_Y(
    example_function_input_A_X_t input
);
```

# compiling

If your code compiles, we will also format it.

If there are any compile errors, the compiler will add some special
comments to the code that will be removed on next compile, e.g.,
`#@! ^ there's a syntax problem`

## metaprogramming

TODO: we'd like to provide ways to define custom block functions like `if X {...}`,
e.g., `whenever Q {...}`.  probably the best way here is to use `Block`, e.g.,
`if(Bool, Block[~t]): t`.  but it'd be also good to support the `declaring` part
of `block`, via, e.g., `check Nullable, NonNull: do_something(NonNull)`, where
we have 
```
check(T?` ~t, Blockable[~u, declaring` t])?: u
    what T
        T`
            Blockable block(T`)
        Null: {Null}
```
without some deep programming, we won't be able to have the option of doing things like
`return X + Y`, since `return` breaks order of operations.
We probably can allow it, but restricted to existing operators like `is`.
But if users want fully custom stuff, they'd need to define their own macros
that start with `@`.

# implementation

## global functions via class methods

```
# oh-lang
my_class: []
{   ::readonly_method(Int): null
    ;;mutating_method(Int): null
}

# C++
void hm::user::readonly_method(readonly_ref<hm::my_class> My_class, readonly_ref<big_int> Int);
void hm::user::mutating_method(mutating_ref<hm::my_class> My_class, readonly_ref<big_int> Int);
```

## types specified locally or remotely

We'll want to support types (a `u64`) being declared remotely or locally.
E.g., when creating an array of `i64`, we don't want to take up room in the array
for the same `i64` type on each array element.  Conversely, if the array element type
has child classes, then we need to keep track of the type on each array element.
(We need to do this unless the array has an `@only` annotation on the internal type.)

TODO: more discussion
