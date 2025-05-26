# why

oh-lang tries to achieve **consistency** above all else.  
**Convenience**, **clarity**, **concision**, **coolness**, and **simplicity** come next.

If you don't care about the why, you may be interested in [general syntax](#general-syntax).

## consistency

In most languages, primitive types have different casing than class types that are
created by end-user developers.  This is usually an inconsistency by convention,
e.g., `PascalCase` for class names and `snake_case` for primitive types in C++,
but Rust requires this inconsistency by fiat.
In oh-lang, functions and types are `trailing_underscore_lower_snake_case_`,
like `my_function_` and `dbl_` or `str_` for types.
All variables and identifiers are `lower_snake_case`, like `true`, `false`,
or `my_var_x`.  oh-lang doesn't recommend distinguishing
between constant identifiers and non-constant identifiers with casing.
In fact, any capital letters in an identifier create a [namespace](#namespace)
which can be used to disambiguate variables with the same name.

TODO: do we use SFO to allow defining types like `x: int`?

Why snake case: long names are hard to parse with Pascal case, and we want to support descriptive
names.  Code is read more than it is written, so we don't need to optimize underscores out
(e.g., like in `PascalCase`) at the cost of making the code harder to read.
The reason why we use similar casing for identifiers and functions is because it's too
easy to refactor a type (or function) like `the_array_` into something like `special_array_`
and miss the corresponding update for variables like `The_array` into `Special_array`.
It also makes internationalization not dependent on unicode parsing; we can immediately
determine whether something is a function if it has a trailing `_`.
For the remainder of this document, we'll use `variable_case`,
`type_case_`, and `function_case_`, although the latter two are indistinguishable without context.
In context, functions and types are followed by optional generics (in `[]` brackets),
while functions alone have parentheses `()` with optional arguments inside.
Because types can act as functions, we don't syntactically distinguish between `type_case_`
and `function_case_` otherwise.

Another change is that oh-lang uses `:` (or `;`) for declarations and `=` for reassignment,
so that declaring a variable and specifying a variable will work the same inside and outside
function arguments.  For example, declaring a function that takes an integer named `x`,
`my_function_(x: int_): null_`, and declaring an integer variable named `x` uses the same syntax:
`x: int_`.  Similarly, calling a function with arguments specified as `my_function_(x: 5)` and
defining a variable works the same outside of a function: `x: 5`.  There is a slight difference
because we can declare variables with `.` in function arguments, which indicates a temporary.  E.g.,
in `(x: int_, y; str_, z. dbl_)`, we declare `x` as a readonly reference, `y` a writable reference,
and `z` a temporary, whereas outside of function arguments, `[x: int_, y; str_, z. dbl_]` indicates
that `x` is readonly (though it can be written in constructors or first assignment),
that `y` is writable, and `z` is volatile (and writable).

In some languages, e.g., JavaScript, objects are passed by reference and primitives
are passed by value when calling a function with these arguments.  In oh-lang,
arguments are passed by reference by default, for consistency.  I.e., on the left
hand side of an expression like `x = 5`, we know that we're using `x` as a reference,
and we extend that to function calls like `do_something_(x)`.  Note that it's possible
to pass by value as well; see [passing by reference or by value](#pass-by-reference-or-pass-by-value). 
See [passing by reference gotchas](#passing-by-reference-gotchas) for the edge cases.

In oh-lang, determining the number of elements in a container uses the same
method name for all container types; `count_(container)` or `container count_()`,
which works for `array_`, `lot_` (map/dict), `set_`, etc.  In some languages, e.g., JavaScript,
arrays use a property (`array.length`) and maps use a different property (`map.size`).

## convenience

oh-lang also prioritizes convenience; class methods can be called like a function with
the instance as an argument or as a method on the instance, e.g., `the_method_(my_class)`
or `class the_method_()`.  This extends to functions with definitions like
`my_two_instance_function_(my_class_a, my_class_b, width: int_, height: int_)` which
can be called as `(my_class_a, my_class_b) my_two_instance_function_(width: 5, height: 10)`,
or by calling it as a method on either one of the instances, e.g.,
`my_class_a my_two_instance_function_(my_class_b, width: 5, height: 10)`, without needing to
define, e.g., `my_class_b my_two_instance_function_(my_class_a, width: 5, height: 10)` as well.

For convenience, `array[3] = 5` will work even if `array` is not already at least size 4;
oh-lang will resize the array if necessary, populating it with default values,
until finally setting the fourth element to 5.  This is also to be consistent with
other container types, e.g., lots (oh-lang's version of a map/dictionary), since `lot["At"] = 50`
works in a similar fashion, e.g., resizing the `lot_` as necessary to add an element.
In some standard libraries (e.g., C++), `array[3] = 5` is undefined behavior
if the array is not already at least size 4.

Similarly, when referencing `array[10]` or `lot["At"]`, a default will be provided
if necessary, so that e.g. `++array[10]` and `++lot["At"]` don't need to be guarded
as `array[10] = if count_(array) > 10 {array[10] + 1} else {array count_(11), 1}` or
`lot["At"] = if lot["At"] != null {lot["At"] + 1} else {1}`.

## clarity

Functions are called with named arguments always, although names can be omitted in
certain circumstances [when calling a function](#calling-a-function).

Nullable variables should always be annotated by `?`, both in a declaration
`y?: some_possibly_null_returning_function_()` and also when used as a function
argument, e.g., `call_with_nullable_(some_value?: y)`.  This is to avoid surprises
with null, since `call_with_nullable_(some_value?: null)` is equivalent to
`call_with_nullable_()`, which can be a different overload.

## concision

When defining a function, variable arguments that use the default name for a type can
elide the type name; e.g., `my_function_(int): str_` will declare a function that takes 
an instance of `int_`, i.e., `int` expands to `int: int_`.
See [default-named arguments](#default-name-arguments-in-functions).
This is also true if [namespaces](#namespaces) are used, e.g., `my_function_(MY_NAMESPACE_int): str_`.
If a declaration operator (like `;`, `:`, or `.`) is not used, we'll default to creating
a readonly reference `:` overload but guess at a few weak overloads for `;` and `.` that
make sense.  These weak overloads can be overridden with explicit `;` and `.` overloads.
TODO: if we want to go the v-lang `fn(x int_)` route, could we also mostly define functions as
readonly via `fn_(x int_) int_` (i.e., for `fn_(x: int_): int_`)?  i like how `:` guides the eye, tho.
If not using a default name, you can get the same weak overloads using e.g., `fn_(x int_)`,
i.e., not using an explicit `:;.` operator between an argument and the type.
We can use `:` to explicitly only create the readonly reference overload, e.g.,
`my_function_(int:): str_` to create a function which takes a readonly integer reference, or
you can use `my_function_(int;): str_` for a function which can mutate the passed-in integer
reference or `my_function_(int.): str_` for a function which takes a temporary integer.
This also works for generic classes like `my_generic_[of_]` where `of_` is a template type;
`my_function_(my_generic_[int_];)` is short for `my_function_(my_generic; my_generic_[int_])`.

When calling a function, we don't need to use `my_function_(x: x)` if we have a local
variable named `x` that shadows the function's argument named `x`.  We can just
call `my_function_(:x)` to specify the readonly reference overload (`x: x`),
`other_function_(;y)` to specify the writable reference overload (`y; y`), or
`tmp_function_(.z)` to specify a temporary overload `z. @hide z!`, which also
stops you from using `z` again in the rest of the block.  Using `tmp_function_(z!)`
calls the temporary overload and would allow `z` to still be used afterwards,
but `z` will be [reset to the default](#prefix-and-postfix-exclamation-points).
Alternatively, you can do `tmp_function_(z o_())` to make a deep copy, and
this will also automatically call the temporary overload without specifying `.`.
If no declaration operator is used when calling a function, e.g., `my_function_(x)`,
then we'll infer `:` if `x` is readonly (i.e., defined with `:`)
and `;` if `x` is writable (i.e., defined `;`), or `.` if `x` is a temporary.
(In this last case, oh-lang acts like Rust and will disable `x` from being reused
in the rest of the block, since it was "used up" in the temporary overload.)
If no temporary overload is present, the compiler will retry for a writable overload.
If no writable overload is present, the compiler will retry for a readonly overload.
Note the declaration operator (`.;:`) goes on the left when calling a function
with an existing variable, and on the right when declaring a variable or argument.
I.e., `;x` expands to `x; x`, while `x;` expands to `x; x_`.
TODO: i'm forgetting to do this and don't always like it.  can we make due without it
and always put declarers on the right?  i like the consistency though.

Class methods technically take an argument for `m` everywhere, which is somewhat
equivalent to `this` in C++ or JavaScript or `self` in python, but instead of
writing `the_method_(m:, x: int_): str_`, we can write `::the_method_(x: int_): str_`.
this parallels `my_class::the_method` in C++, but in oh-lang we can analogously use
`;;a_mutating_method_` for a method that can mutate `m`, i.e.,
`a_mutating_method_(m;, x: int_): str_` becomes `;;a_mutating_method_(x: int_): str_`,
or `..one_temporary_method_()` for a method on a temporary `m`, i.e.,
`one_temporary_method_(m.)`.  Inside an instance method definition, you can use `m`
to refer to the class instance, regardless of how it was defined.  You also use
`m the_variable_name` to refer to a field `the_variable_name` defined on the class
inside any methods.

```
vector3_: [x: dbl_, y: dbl_, z: dbl_]
{   ::length_(): dbl_
        sqrt_(m x * m x + m y * m y + m z * m z)
}
```

We support nested types without needing a `m_` prefix, and they are available anywhere
in the class body they are defined.  This is a minor inconsistency with instance methods
(which always require `m`) but it makes overloads much easier to reason about.  However,
to prevent confusion, these nested types cannot shadow any global types.
Here is an example with a generic type, where it would be convenient
to refer to another generic subtype if we already have the class.

```
my_generic_[at_, of_]: [lot;]
{   lot_: @only insertion_ordered_lot_[at_, of_]
    ...
}

# TODO: we probably don't want anyone to use `lot_`, since we'll make that globally available.
# ERROR: `lot_` is shadowed inside of `my_generic_`, use import renaming to avoid this.
#       e.g., `[core_lot_: lot_]: \\core/lot`
[lot_]: \\core/lot
```

After fixing the compile error in the example above, we can use `some_type: my_generic_[at_, of_] lot_`
to refer to the nested type, but can we also use `some_type: lot_[m_: my_generic_[at_, of_]]`.
We don't override `lot_[my_generic_[at_, of_]]` because a single type might be an override of `lot_[of_]`;
this isn't the case for `lot_` specifically but for other types like `array_` there are definitely overloads.
See [type manipulation](#type-manipulation) for more details.

Note this is actually ok, because we can distinguish overloads based on arguments.

```
vector2_: [x: dbl_, y: dbl_]
{   ::atan_(): dbl
        atan_(m x, m y)      # also ok: `\\math atan_(m x, m y)` could avoid the import below.
}
[atan_(x: dbl_, y: dbl_): dbl_]: \\math
```

Also in the spirit of conciseness, `o` can be used for an *o*ther instance of the same type,
and `g_` can be used for the current generic class (without the specification) while
`m_` always refers to the current class type (including the specification if generic).

```
vector3_[of_: number_]: [x; of_, y; of_, z; of_]
{   # `g_` is used for this generic class without the current specification,
    # in this case, `vector3_`.
    g_(FIRST_value: ~value, SECOND_value, THIRD_value): g_[value]
        [x: FIRST_value, y: SECOND_value, z: THIRD_value]

    ::dot_(o): of_
        m x * o x + m y * o y + m z * o z
}

dot_(vector3_(1, 2, 3), vector3_(-6, 5, 4)) == 1 * -6 + 2 * 5 + 3 * 4

TODO: make sure we add `m` back to all fields.
```

Class getters/setters *do not use* `::get_x_(): dbl_` or `;;set_x_(dbl.): null_`, but rather
just `::x_(): dbl_` and `;;x_(dbl.): null_` for a private variable `x; dbl_`.  This is one
of the benefits of using `function_case_` for functions/methods and `variable_case`
for variables; we can easily distinguish intent without additional verbs.
Of course, overloads are also required here to make this possible.

Because we use `::` for readonly methods and `;;` for writable methods, we can
easily define "const template" methods via `:;` which work in either case `:` or `;`.
This is mostly useful when you can call a few other methods internally that have specific
`::` and `;;` overloads, since there's usually some distinct logic for readonly vs. writable.
E.g., `;:the_method_(x;: str_): m check_(;:x)` where `check_` has distinct overloads for `::` and `;;`.
See [const templates](#const-templates) for more details.

oh-lang uses result-passing instead of exception-throwing in order to make it clear
when errors can occur.  The `hm_[ok_, er_]` class handles this, with `ok_` being the
type of a valid result, and `er_` being the type of an error result.  You can specify
the types via `hm_[ok_: int_, er_: str_]` for `ok_` being `int_` and `er_` being a `str_`.
If the `ok_` and `er_` types are distinct, you don't need to wrap a return value in
`ok_(valid_result)` and `er_(error_result)`; you can just return `valid_result` or `error_result`.
See [the `hm` section](#hm) for more details.  It is a compile error to not handle
errors when they are returned (e.g., something like a `no-unused-result`), although
often there are overloads (without an `hm_` result being returned) which just panic
at runtime in case of an error.

## coolness

**Coolness** is a fairly subjective measure, but we do use it to break ties.
While there are a lot of good formatting options out there, 
[Horstmann brace style](https://en.wikipedia.org/wiki/Indentation_style#Horstmann) is
hands-down the raddest indentation style.  Similarly, `variable_case`
and `function_case_`/`type_case_` make for more readable long names, but they also
look cooler than their `dromedaryCase` and `PascalCase` counterparts.

## simplicity

We don't require a different function name for each method to convert a result class
into a new one, e.g., to transform the `ok_` result or the `er_` error.  In oh-lang, we
allow overloading, so converting a result from one type to another, or extracting a
default value for an error, all use an overloaded `map` method, so there's no mental
overhead here.  Since overloads are not possible in Rust, there is an abundance of methods, e.g.,
[`Result::map_*` documentation](https://doc.rust-lang.org/std/result/enum.Result.html#method.map),
which can require constantly poring over documentation just to find the right one.  

We also don't use a different concept for interfaces and inheritance.
The equivalent of an interface in oh-lang is simply an abstract class.  This way
we don't need two different keywords to `extend` or `implement` a class or interface.
In fact, we don't use keywords at all; to just add methods (or class functions/variables),
we use this syntax, `child_class_: parent_class_ { ::extra_methods_(): int_, ... }`,
and to add instance variables to the child class we use this notation:
`child_class_: all_of_[parent_class_, m_: [child_x: int_, child_y: str_]] { ... methods }`.

oh-lang handles generics/templates in a way more similar to zig or python rather than C++ or Rust.
When compiled without any usage, templates are only tested for syntax/grammar correctness.
When templates are *used* in another piece of code, that's when the specification kicks in
and all expressions within the generic are compiled to see if they are allowed with the
specified types.  Any errors are still compile-time errors, but you get to have the simplicity
of duck typing without needing to specify your type constraints fully.

```
my_generic_[of_](a: ~of_, b: of_): of_
    # this clearly requires `of_` to implement `*`
    # but we didn't need to specify `[of_: number_]` or similar in the generic template.
    a * b

print_(my_generic_(a: 3, b: 4))                 # OK
print_(my_generic_(a: [1, 2, 3], b: [4, 5]))    # COMPILE ERROR: no definition for `array_[int_] * array_[int_]`
```

Similarly, duck typing means that if you define an appropriate `::hash` function on your class,
you don't need to mention that your class is `hashable`.  A check for `some_class is some_other_class`
will not require strict descent from `some_other_class` but only that the same methods and fields
are defined.

## safety

oh-lang supports "safe" versions of functions where it's possible that we'd run out of
memory or otherwise throw.  By default, `array[100] = 123` will increase the size
of the array if necessary, and this could potentially throw in a memory-constrained
environment (or if the index was large).  If you need to check for these situations,
there is a safe API, e.g., `hm: (array[100] = 123)` and the result `hm` can then
be checked for `is_er_()`, etc.  In order to avoid another syntax for safe assignment,
we use operator overloading (via return name).  Most of the time you don't want to
hide errors from other developers, however, but if you do, you should make the
program panic/terminate rather than continue.  Example code:

```
custom_container_[of_]: [vector[10, of_];]
{   # make an overload for `m[ordinal]` where `ordinal_` is a 1-based indexing type.
    :;[ordinal]: hm_[ok_: (of:;), er_: str_]
        if ordinal > 10
            er_("index too high")
        else
            ok_((of:; vector[ordinal]))

    @can_panic
    :;[ordinal]: (of:;)
        m[ordinal] hm assert_()

    # for short, you can use this `@hm_or_panic` macro, which will essentially
    # inline the logic into both methods but panic on errors.
    :;[ordinal]: @hm_or_panic_[ok_: (of:;), er_: str_]
        if ordinal > 10
            er_("index too high")
        else
            ok_((of:; vector[ordinal]))
}
```

Almost all operations similarly have a result-like syntax, because they can fail.
E.g., `a * b` can overflow for fixed-width integers (or run out of memory for `int`).
Similarly for `a + b` and `a - b`.  (`a // b` is safe for fixed-width, but could
potentially OOM for `int`.)  If overflow/underflow is desired, use the overload
which returns a variable named `wrap`, e.g., `x: (a + b) wrap` or `wrap: a + b`.
Otherwise `a + b` will panic on overflow and terminate the program.  The alternative
is to handle the error explicitly: `hm: a + b` then something like this:
`what hm {ok: {print_(ok)}, er: {print_("got error: ${er})}}`.

TODO: do we want primitive types to NOT panic on overflow, but wrapper types
like `count` and `index` to panic?

# general syntax

* `print_(...)` to echo some values (in ...) to stdout, `error_(...)` to echo to stderr
    * use string interpolation for printing dynamic values: `print_("hello, ${variable_1}")`
    * use `print_(no_newline: "keep going ")` to print without a newline
    * default overload is to print to null, but you can request the string that was printed
        if you use the `print_(str.): str_` or `error_(str.): str_` overloads.
        e.g., `another_fn_(value: int): print_("Value is ${value}")` will return `null`,
        whereas `another_fn_(value: int): str {print_("Value is ${value}")}` will
        return "Value is 12" (and print that) if you call `another_fn_(value: 12)`.
* `type_case_`/`function_case_` identifiers like `x_` are function/type-like, see [identifiers](#identifiers)
* `variable_case` identifiers like `x` are instance-like, see [identifiers](#identifiers)
* use `#` for [comments](#comments)
* outside of arguments, use `:` for readonly declarations and `;` for writable declarations
* for an argument, `:` is a readonly reference, `;` is a writable reference, and `.` is a temporary
    (i.e., passed by value), see [pass-by-reference or pass-by-value](#pass-by-reference-or-pass-by-value)
* use `:` to declare readonly things, `;` to declare writable things.
    * use `a: x_` to declare `a` as an instance of type `x_`, see [variables](#variables),
        with `a` any `variable_case` identifier.
    * use `fn_(): x_` to declare `fn_` as a function returning an instance of type `x_`, see [functions](#functions),
        with any arguments inside `()`.  `fn_` can be renamed to anything
        `function_case_`, but `fn_` is the default.
    * use `a_: y_` to declare `a_` as a constructor that builds instances of type `y_`
        with `a_` any `type_case_` identifier.
    * use `new_[]: y_` to declare `new_` as a function returning a type `y_`, with any arguments inside `[]`.
        `new_` can be renamed to anything `type_case_`, but `new_` is the default.
    * while declaring *and defining* something, you can avoid the type if you want the compiler to infer it,
        e.g., `a: some_expression_()`
    * thus `:=` is usually equivalent to `:` (and similarly for `;=`), except in the case of defining
        a function via another function, i.e., function aliasing.  E.g.,
        `fn_(x: int_): str_ = other_fn_` will alias `other_fn_(x: int_): str_` to `fn_`, while
        `fn_(x: int_): return_type_` just declares a function that returns an instance of `return_type_`.
* when not declaring things, `:` is not used; e.g., `if` statements do not require a trailing `:` like python
* commas `,` are equivalent to a line break at the current tab and vice versa
    * `do_something_(), do_something_else_()` executes both functions sequentially 
    * see [line continuations](#line-continuations) for how commas can be elided across newlines for e.g., array elements
* `()` for reference objects, organization, and function calls/declarations
    * `(w: str_ = "hello", x: dbl_, y; dbl_, z. dbl_)` to declare a reference object type, `w` is an optional field
        passed by readonly reference, `x` is a readonly reference, `y` is a writable reference,
        and `z` is passed by value.  See [reference objects](#reference-objects) for more details.
    * `my_str: "hi", (x: str_) = my_str` to create a [reference](#references) to `my_str` in the variable `x`.
    * `(some_instance x_(), some_instance Y;, w: "hi", z. 1.23)` to instantiate a reference object instance
        with `x` and `w` as readonly references, `y` as mutable reference, and `z` as a temporary.
    * `"My String Interpolation is $(x, y: z)"` to add `(x: *value-of-x*, y: *value-of-z*)` to the string.
    * `f_(a: 3, b: "hi")` to call a function, and `f_(a: int_, b: str_): null_` to declare a function.
    * `a@ (x_(), y)` to call `a x_()` then `a y` with [sequence building](#sequence-building)
        and return them in a reference object with fields `x` and `y`, i.e., `(x: a x_(), y: a y)`.
        This allows `x` and `y` to be references.  This can be useful e.g., when `a` is an expression
        that you don't want to add a local variable for, e.g., `my_long_computation_()@ (x_(), Y)`.
* `[]` are for types, containers (including objects, arrays, and lots), and generics
    * `[x: dbl_, y: dbl_]` to declare a plain-old-data class with two double-precision fields, `x` and `y`
    * `[x: 1.2, y: 3.4]` to instantiate a plain-old-data class with two double-precision fields, `x` and `y`
    * `"My String interpolation is $[x, y]"` to add `[*value-of-x*, *value-of-y*]` to the string.
    * `some_class_[n: number_, of_]: some_other_class_[count: n, at_: int_, of_]` to define a class type
        `some_class` being related to `some_other_class`, e.g., `some_class_[n: 3, str_]` would be
        `some_other_class_[count: 3, at_: int, of_: str_]`.
    * For generic/template classes, e.g., classes like `array_[count, of_]` for a fixed array of size
        `count` with elements of type `of_`, or `lot_[int_, at_: str_]` to create a map/dictionary
        of strings mapped to integers.  See [generic/template classes](#generictemplate-classes).
    * For generic/template functions with type constraints, e.g., `my_function_[of_: non_null_](x: of_, y: int_): of_`
        where `of_` is the generic type.  See [generic/template functions](#generictemplate-functions) for more.
    * `[greeting: str_, times: int_] = destructure_me_()` to do destructuring of a return value
        see [destructuring](#destructuring).
    * `a@ [x_(), y]` to call `a x_()` then `a y` with [sequence building](#sequence-building)
        and return them in an object with fields `x` and `y`, i.e., `[x: a x_(), y: a y]`.
        You can also consider them as ordered, e.g.,
        `results: a@ [x_(), y], print_("${results[0]}, ${results[1]})`.
* `{}` for blocks and sequence building
    * `{...}` to effectively indent `...`, e.g., `if condition {do_thing_()} else {do_other_thing_(), 5}`
        * Used for defining a multi-statement function inline, e.g., `fn_(): {do_this_(), do_that_()}`.
            (Note that you can avoid `{}` if the block is one statement, like `fn_(): do_this_()`.)
        * Note that braces `{}` are optional if you actually go to the next line and indent,
            but they are recommended for long blocks.
    * `a@ {x_(), y}` with [sequence building](#sequence-building), 
        calling `a x_()` and `a y`, returning `a` if it's a temporary otherwise `a y`
    * `"My String Interpolation is ${missing_(), x}"` to add `x` to the string.
        Note that only the last element in the `${}` is added, but `missing_()` will still be evaluated.
* `~` to infer or generalize a type
    * `my_generic_function_(value: ~u_): u_` to declare a function that takes a generic type `u_`
        and returns it.  For more details, see [generic/template functions](#generictemplate-functions).
    * `my_result; array_[~] = do_stuff_()` is essentially equivalent to `my_result; do_stuff_() array`, i.e.,
        asking for the first array return-type overload.  This infers an inner type via `[~]` but doesn't name it.
    * `named_inner; array_[~infer_this_] = do_stuff_()` asks for the first array return-type overload,
        and defines the inner type so it can be used later in the same block, e.g.,
        `first_value; infer_this_ = named_inner[0]`.
        Any `type_case_` identifier can be used for `infer_this_`.
* `$` for inline block and lambda arguments
    * [inline blocks](#block-parentheses-and-commas) include:
        * `$[...]` as shorthand for a new block defining `[...]`, e.g., for a return value:
            `array: if some_condition $[1, 2, 3] else $[4, 5]`
        * `$(...)` as shorthand for a new block defining `(...)`, e.g., a reference object:
            `result: if x > y $(max: x, min: y) else $(min: x, max: y)`
        * `${...}` is almost always equivalent to `{...}`, except inside of string interpolation,
            so we'll likely alias `${...}` to `{...}` outside of strings.
    * `$arg` as shorthand for defining an argument in a [lambda function](#lambda-functions)
        * `my_array map_({$int * 2 + 1})` will iterate over e.g., `my_array: [1, 2, 3, 4]`
            as `[3, 5, 7, 9]`.  The `$` variables attach to the nearest brace/indent as
            function arguments, variables with `$$` would attach to the second nearest brace/indent, etc.
* all arguments are specified by name so order doesn't matter, although you can have default-named arguments
  for the given type which will grab an argument with that type (e.g., `int` for an `int_` type).
    * `(x: dbl_, int)` can be called with `(1234, x: 5.67)` or even `(y, x: 5.67)` if `y` is an `int_`
* variables that are already named after the correct argument can be used without `:`
    * `(x: dbl_, y: int_)` can be called with `(x, y)` if `x` and `y` are already defined in the scope,
        i.e., eliding duplicate entries like `(x: x, y: y)`.
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
readonly_var: int_
mutable_var; int_

# declaring + defining a variable:
mutable_var; 321

# you can also give it an explicit type:
readonly_var: int_(123)

# you can also define a variable using an indented block;
# the last line will be used to initialize the variable.
# here we will infer the type; it's implicit in whatever
# `some_helper_value + 4` is.
my_var:
    # this helper variable will be descoped after calculating `my_var`.
    some_helper_value: some_computation_(3)
    some_helper_value + 4

# you can also give it an explicit type:
other_var; explicit_type_
    "asdf" + "jkl;"
```

## defining strings

```
# declaring a string:
name: "Barnabus"

# using interpolation in a string:
greeting: "hello, ${name}!"

# declaring a multiline string
important_items:
        &|Fridge
        &|Pancakes and syrup
        &|Cheese
# this is the same as `important_items: "Fridge\nPancakes and syrup\nCheese\n"`

# a single-line multiline string still includes a newline at the end.
just_one_line: &|This is a 'line' "you know"
# this is equivalent to `just_one_line: "This is a 'line' \"you know\"\n"

# declaring a multiline string with interpolation
multiline_interpolation:
        &|Special delivery for ${name}:
        &|You will receive ${important_items} and more.
# becomes "Special delivery for Barnabus\nYou will receive Fridge\nPancakes and syrup\nCheese\n and more."

# interpolation over multiple file lines.
# WARNING: this does not comply with Horstmann indenting,
# and it's hard to know what the indent should be on the second line.
evil_long_line: "this is going to be a long discussion, ${
        name}, can you confirm your availability?"
# INSTEAD, use string concatenation, which automatically adds a space if necessary:
good_long_line: "this is going to be a long discussion,"
    &   "${name}, can you confirm your availability?"

# you can also nest interpolation logic, although this isn't recommended:
nested_interpolation: "hello, ${if condition {name} else {'World${"!" * 5}'}}!"
```

Notice that the `&` operator works on strings to add a space (if necessary)
between the two operands.  E.g., `'123' & '456'` becomes `'123 456'`.  It also
strips any trailing whitespace on the left operand and any leading whitespace
on the right operand to ensure things like `'123\n \n' & '\n456'` are still just `'123 456'`.
This makes it the perfect operator for string concatenation across lines where we want
to ensure a space between words on one line and the next.
TODO: it could also be used as a postfix or prefix operator, e.g., `&'   hi'` is 'hi'
and `'hey\n  '&` is 'hey'.  not sure this is better than `'   hi' strip_()` though.

## defining arrays

See [arrays](#arrays) for more information.

```
# declaring a readonly array
my_array: array_[element_type_]

# defining a writable array:
array_var; array_[int_](1, 2, 3, 4)
# We can also infer types implicitly via one of the following:
#   * `array_var; array_(1, 2, 3, 4)`
#   * `array_var; [1, 2, 3, 4]`
array_var[5] = 5    # array_var == [1, 2, 3, 4, 0, 5]
++array_var[6]      # array_var == [1, 2, 3, 4, 0, 5, 1]
array_var[0] += 100 # array_var == [101, 2, 3, 4, 0, 5, 1]
array_var[1]!       # returns 2, zeroes out array_var[1]:
                    # array_var == [101, 0, 3, 4, 0, 5, 1]

# declaring a long array (note the Horstmann indent):
long_implicitly_typed:
[   4   # commas aren't needed here.
    5
    6
]

# declaring a long array with an explicit type:
long_explicitly_typed: array_[i32_]
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
my_lot: lot_[at: id_type_, value_type_]

# defining a writable lot:
votes_lot; lot_[at_: str_, int_]("Cake": 5, "Donuts": 10, "Cupcakes": 3)
# We can also infer types implicitly via one of the following:
#   * `votes_lot; lot_(["Cake": 5, ...])`
#   * `votes_lot; ["Cake": 5, ...]`
votes_lot["Cake"]           # 5
++votes_lot["Donuts"]       # 11
++votes_lot["Ice Cream"]    # inserts "Ice Cream" with default value, then increments
votes_lot["Cupcakes"]!      # deletes from the lot (but returns `3`)
votes_lot::["Cupcakes"]     # null
# now votes_lot == ["Cake": 5, "Donuts": 11, "Ice Cream": 1]
```

## defining sets

See [sets](#sets) for more details.

```
# declaring a readonly set
my_set: set_[element_type_]

# defining a writable set:
some_set; set_[str_]("friends", "family", "fatigue")
# We can also infer types implicitly via the following:
#   * `some_set; set_("friends", ...)`
some_set::["friends"]   # `true`, without changing the set.
some_set::["enemies"]   # null (falsey), without changing the set.
some_set["fatigue"]!    # removes "fatigue", returns `true` since it was present.
                        # `some_set == set_("friends", "family")`
some_set["spools"]      # adds "spools", returns null (wasn't in the set), but now is.
                        # `some_set == set_("friends", "family", "spools")`
```

## defining functions

See [functions](#functions) for a deeper dive.

```
# declaring a "void" function:
do_something_(with: int_, x; int_, y; int_): null_

# defining a void function
# braces {} are optional (as long as you go to the next line and indent)
# but recommended for long functions.
do_something_(with: int_, x; int_, y; int_): null
    # because `x` and `y` are defined with `;`, they are writable
    # in this scope and their changes will persist back into the
    # caller's scope.
    x = with + 4
    y = with - 4

# calling a function with temporaries is allowed, even with references:
do_something_(with: 5, x; 12, y; 340)

# calling a function with variables matching the argument names:
with: 1000
x; 1
y; 2
# because `with`, `x`, and `y` are defined in the same way
# as the arguments, you can avoid specifying the `;` or `:`
do_something_(with, x, y)
# but you can also be explicit, which is recommended
# if you want to ensure a specific overload is used.
do_something_(:with, ;x, ;y)

# calling a function with argument renaming:
mean: 1000
mutated_x; 1
mutated_y; 2
do_something_(with: mean, x; mutated_x, y; mutated_y)
```

```
# declaring a function that returns other values:
do_something_(x: int_, y: int_): [w: int_, z: int_]

# defining a function that returns other values.
# braces are optional as long as you go to the next line and indent.
do_something_(x: int_, y: int_): [w: int_, z: dbl_]
{   # NOTE! return fields `w` and `z` are in scope and can be assigned
    # directly in option A:
    z = \\math atan_(x, y)
    w = 123
    # option B: can just return `w` and `z` in an object:
    [z: \\math atan_(x, y), w: 123]
}
```

We don't require `{}` in function definitions because we can distinguish between
(A) creating a function from the return value of another function, (B) passing a function
as an argument, and (C) defining a function inline in the following ways.  (A) uses
`my_fn_(args): return_type_ = fn_returning_a_fn_()` in order to get the correct type on `my_fn_`,
(B) uses `outer_fn_(rename_to_this_(args): return_type = use_this_fn_)` and requires a single
`function_case_` identifier on the RHS, while (C) uses `defining_fn_(args): do_this_(args)`
and uses inference to get the return type (for the default `do_this_(args)` function).

```
# case (A): defining a function that returns a lambda function
make_counter_(counter; int_): do_(): int_
    do_(): ++counter
# TODO: equivalent? `make_counter_(counter; int_): do_(): ++counter`
counter; 123
counter_(): int = make_counter_(;counter)
print_(counter_())    # 124
# `counter` is also 124 now.
```

Note that because we support [function overloading](#function-overloads), we need
to specify the *whole* function [when passing it in as an argument](#functions-as-arguments).

```
# case (B): defining a function with some lambda functions as arguments
do_something_(you_(): str_, greet_(name: str_): str_): str_
    greet_(name: you_())

# calling a function with some functions as arguments:
my_name_(): "World"
do_something_
(   you_(): str_ = my_name_
    greet_(name: str_): str_
        "Hello, ${name}"
)

# case (C): defining a few functions inline without `{}`
hello_world_(): print_(do_something_(you_(): "world", greet_(name: str_): "Hello, ${name}"))
```

### defining generic functions

There are two ways to define a generic function: (1) via type inference `~x`
and (2) with an explicit generic specification `[types_...]` after the function name.
You can combine the two methods if you want to infer a type and specify a
condition that the type should satisfy, e.g., `fn_[x_: number_](~x): x_`,
where `~x` expands to `x: ~x_`, meaning that `x_` is inferred, and the
brackets require `x_` to be a number type.  Any types that are not inferred
but are explicitly given in brackets must be added at the callsite, e.g.,
`fn_[x_: number_, y_](~x, after: y_): y_` should be called like `fn_[y_: int_](123.4, after: 5)`.

Note that default names apply to either case; `~x` is shorthand for `x: ~x_`
which would not need an argument name, and `fn_[value_](value): null` would
require `value_` specified in the brackets but not in the argument list,
e.g., `fn_[value_: int_](123)`.  In brackets, the "default name" for a type is
`of_`, so you can call a function like `fn_[of_](of): null_` as `fn_[int_](123)`.

Some examples:

```
# this argument type is inferred, with a default name
fn_(~x): x_
# call it like this:
fn_(512)

# this argument type is inferred but need to name it as `x: ...`
fn_(~NAMED_x): x_
# call it like this:
fn_(x: 512)

# another way to infer an argument but require naming it as `x: ...`
fn_(x: ~t_): t_
# we call it like this:
fn_(x: 512)

# explicit generic with condition, not inferred:
fn_[x_: condition_or_parent_type_](x): x_
# call it like this, where `int_` should satisfy `condition_or_parent_type_`
fn_[x_: int_](5)

# explicit generic with condition, inferred
fn_[x_: condition_](~x): x_
# call it like this, where `dbl_` should satisfy `condition_`
fn_[x_: dbl_](3.14)

# explicit generic without a default name:
fn_[x_](value: x_): null_
# call it like this:
fn_[x_: str_](value: "asdf")

# explicit default-named generic, but argument is not default named:
fn_[of_](value: of_): of_
# call it like this; you can omit `of_: ...` in brackets:
fn_[int_](value: 123)
```

See [generic/template functions](#generictemplate-functions) for more details.


## defining classes

See [classes](#classes) for more information on syntax.

```
# declaring a simple class
vector3_: [x: dbl_, y: dbl_, z: dbl_]

# declaring a "complicated" class.  the braces `{}` are optional
# but recommended due to the length of the class body.
my_class_: [x; int_]
{   # here's a class function that's a constructor
    m_(x. int_): m_
        ++count
        [x]

    ;;descope_(): null_
        --count

    # here's a class variable (not defined per instance)
    @private
    count; count_arch_ = 0

    # here's a class function (not defined per instance)
    # which can be called via `my_class_ count_()` outside this class
    # or `count_()` inside it.
    count_(): count_arch_
        count
    # for short, `count_(): count`

    # methods which keep the class readonly use a `::` prefix
    ::do_something_(y: int): int_
        x * y

    # methods which mutate the class use a `;;` prefix
    ;;update_(y: int_): null_
        # because there's an implicit `m;` here, it'll look for
        # ;;do_something_(y) first, but resolve to `::do_something_(y)`:
        x = do_something_(y)
}
```

Inside a class body, we don't need to use `m` to scope instance variables/functions
or `m_` to scope class variables/functions, because we always produce a
compile error if we notice any variables/functions that would shadow
global variables/functions.  Import renaming is recommended to solve
this issue.
TODO: is everything ok for keywords like `each` and `is` which can also be methods?
i think so because we would use LHS expressions for anything that would conflict,
so we wouldn't resolve to `m`, we'd use the LHS.

Inheritance of a concrete parent class and implementing an abstract class
work the same way, by specifying the parent class/interface in an `all_of_`
expression alongside any child instance variables, which should be tucked
inside an `m_` field.  Despite requiring the `m_` field in the `all_of_`,
we don't need to specifically look up fields in the child via `m field_name`;
we can still just use `field_name` since `m` fields are automatically
brought into scope for any methods.
TODO: make sure that's desired; it kinda makes sense to only enscope
it as `m` if we have an `all_of_[m_: [field_name: ...]]`.
TODO: i think i like `reset` more than `renew`.  we don't use `new` anywhere
except for `new_[...]` which might make sense to update now.

```
parent1_: [p1: str]
{   ::do_p1_(): null_
        print_("doing p1 ${p1}")
}

parent2_: [p2: str]
{   ::do_p2_(): null_
        print_("doing p2 ${p2}")
}

child3_: all_of_[parent1_, parent2_, m_: [c3: int_]]
{   # this passes p1 to parent1 and c3 to child3 implicitly,
    # and p2 to parent2 explicitly.
    ;;renew_(parent1 p1. str_, p2. str_, m c3. int_): null_
        # same as `parent2_ renew_(m;, p2)` or `parent2_;;renew_(p2)`:
        parent2 renew_(p2)

    ::do_p1_(): null_
        # this logic repeats `parent1 do_p1_())` `m c3` times.
        m c3 each _int:
            # same as `parent1_ do_p1_(m)` or `parent1_::do_p1_()`.
            parent1 do_p1_()
    
    # do_p2_ will be used from parent2_ since it is not overridden here.
}
```

For those aware of storage layout, order matters when using `all_of_`;
the struct will be started with fields in `a_` for `all_of_[a_, b_, c_]`
and finish with fields in `c_`; the child fields do not need to be first
(or last); they can be added as `a_`, `b_`, or `c_`, of course as `m_: [...]`.
Generally it's recommended to add child fields last.

### defining generic classes

With classes, generic types must be explicitly declared in brackets.
Any conditions on the types can be specified via `[the_type: the_condition, ...]`.

```
# default-named generic
generic_[of_]: [@private of]
{   # you can use inference in functions, so you can use `generic_(12)`
    # to create an instance of `generic_` with `of_: int_` inferred.
    # You don't need this definition if `[of]` is public.
    # NOTE: `g_` is like `m_` for generic classes but without the specification.
    g_(~t.): g_[t_]
        [of. t] 
}

generic[int_](1)            # shorthand for `generic: generic_[int_](1)`.
my_generic: generic_(1.23)  # infers `generic_[dbl_]` for this type.
WOW_generic("hi")           # shorthand for `WOW_generic: generic_("hi")`, infers `generic_[str_]`

# not default named:
entry_[at_: hashable_, of_: number_]: [at, value; of_]
{   ::add_(of): null_
        value += of
}

entry[at_: str_, int_](at: "cookies", value: 123)   # shorthand for `entry: entry_[at_: str_, of_: int_](...)`
my_entry: entry_(at: 123, value: 4.56)              # infers `at_: int_` and `of_: dbl_`.
my_entry add_(1.23)
my_entry value == 5.79
```

See [generic/template classes](#generictemplate-classes) for more information.

## identifiers

Identifiers in oh-lang are very important.  The trailing underscore (or lack thereof)
indicates whether the identifier is a function/type (or a variable), which gives some
space to guide the eye with function calls like `my_function_(x: 3)`.  Similarly for
type (or class) names, since types can work like functions (e.g., `int_(number_string)`).
Variable names like `x` and `max_array_count` do not include a trailing underscore.
Any capitalized letters belong to a [namespace](#namespaces).

There are a few reserved keywords, like `if`, `elif`, `else`, `with`, `return`,
`what`, `in`, `each`, `for`, `while`, `pass`, `where`,
which are function-like but may consume the rest of the statement.
E.g., `return X + 5` will return the value `(X + 5)` from the enclosing function.
There are some reserved namespaces with side effects like `FIRST_`, `SECOND_`,
`THIRD_`, `NAMED_`, `AS_`,
which should be used for their side effects.  For example, `FIRST_` and `SECOND_`
should be used for binary operations like `&&` and `*`.  See [namespaces](#namespaces)
for more details.

There are some reserved variable names, like `m`, which can only
be used as a reference to the current class instance, and `o` which
can only be used as a reference to an *o*ther instance of the same type;
`o` must be explicitly added as an argument, though, in contrast to `m` which can be implicit.
The corresponding types `m_`, and `o_` are reserved, `m_` for class bodies
(to indicate the current type) and `o_` as a method to *clone* (or copy)
the current instance, with function signature `::o_(): m_` or
`::o_(): hm_[ok_: m_, er: ...]` if cloning can fail (e.g., due to OOM).

Most ASCII symbols are not allowed inside identifiers, e.g., `*`, `/`, `&`, etc., but
underscores (`_`) have some special handling.  They are ignored in numbers,
e.g., `1_000_000` is the same as `1000000`, and highly recommended for large numbers.
To indicate a variable (or function) is unused in a block, use a prefix underscore,
such as `_unused_variable`.  If used when defining a function
argument, it will not affect how callers call the function; they'll use the
non-trailing-underscored name.

```
# when defining, we use a leading underscore to indicate the variable is unused.
my_function_(_argument_which_we_will_need_later: int_): null_
    print_("TODO")

# when calling:
my_function_(argument_which_we_will_need_later: 3)
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
# the following definitions are equivalent to `my_array: [5, 6, 7]`.

# this is the block-definition style for a variable
my_array:
    [5, 6, 7]

# this is similar to the block definition.
my_array:   # OK, but...
    [   5
        6
        7
    ]

# note it's unnecessary because we also allow opening brackets
# to get attached to the previous line if the internals are indented.
my_array:   # better!
[   5
    6
    7
]

# if you want to one-line it on a second line it's also possible with a +2 indent.
my_array:
        [5, 6, 7]

# the parentheses trick only works if the inside is indented.
not_defined_correctly:
[5, 6, 7]       # not attached to the previous line.
```

Because of this, some care must be taken when returning a bracket
from a function, since we may try to pair it with the previous line.
If you *don't* want to pair a block with the previous line, use `pass` or `return`.  

```
# example of returning `[x, y]` values from a function.
# there's no issue here because we're not indenting in `[x, y]`:
my_function_(int): [x: int_, y: int_]
    do_something_(int)
    [x: 5 - int, y: 5 + int]

# this indents `[x, y]` (i.e., to split into a multi-line array),
# but note that we need `return` to avoid parsing as `do_something_(int)[x: ...]`.
my_function_(int): [x: int_, y: int_]
    do_something_(int)
    return
    [   x: 5 - int
        y: 5 + int
    ]

# alternatively, you could add a comma between the two statements
# to ensure it doesn't parse as `do_something_(int)[x: ...]`:
my_function_(int): [x: int, y: int]
    do_something_(int),
    [   x: 5 - int
        y: 5 + int
    ]
```

Because parentheses indicate [reference objects](#reference-objects),
which can be returned like brackets, similar care must be taken with `()`.
When it comes to parentheses, you are welcome to use
[one-true-brace style](https://en.wikipedia.org/wiki/Indentation_style#:~:text=One%20True%20Brace),
which will be converted into Horstmann style.

```
some_variable: some_very_long_function_name_because_it_is_good_to_be_specific_(10)
    +   3               # indent at +2 ensures that 3 is added into Some_variable.
    -   other_variable  # don't keep adding more indents, keep at +2 from original.

array_variable:
[   1   # we insert commas
    2   # between each newline
    3   # as long as the indent is the same.
    other_array # here we don't insert a comma after `Other_array`
    [   3       # because the indent changes
    ]           # so we parse this as `other_array[3],`
    5           # and this gets a comma before it.
]

# this is inferred to be a `lot` with a string ID and a `one_of_[int_, str_]` value.
lot_variable;
[   "Some_value": 100
    "Other_value": "hi"
]
lot_variable["Some_other_value"] = if condition {543} else {"hello"}

# This is different than the `lot_variable` because it is an instance
# of a `[some_value: int, other_value: str]` plain-old-data type,
# which cannot have new fields added, even if it was mutable.
object_variable:
[   some_value: 100
    other_value: "hi"
]
```

Note that the close parenthesis must be at the same indent as the line of the open parenthesis.
The starting indent of the line is what matters, so a close parenthesis can be on the same
line as an open parenthesis.

```
some_value:
(       (20 + 45)
    *   continuing + the + line + at_plus_2_indent -
        (       nested * parentheses / are + ok
            -   too
        )
)

another_line_continuation_variable: can_optionally_start_up_here
    +   ok_to_not_have_a_previous_line_starting_at_plus_two_indent * 
        (       keep_going_if_you_like
            -   however_long
        ) + (70 - 30) * 3

# note that the formatter will take care of converting indents like this:
non_horstmann_indent: (
    20 + some_function_(45)
)
# into this:
non_horstmann_indent:   # FIXME: update name :)
(   20 + some_function_(45)
)
```

Note that line continuations must be at least +2 indent, but can be more if desired.
Unless there are parentheses involved, all indents for subsequent line continuations
should be the same.

```
example_plus_three_indent; some_type_
...
example_plus_three_indent
    =       hello
        +   world
        -   continuing
```

Arguments supplied to functions are similar to arrays/lots and only require +1 indent
if they are multiline.

```
if some_function_call_
(   x
    y: 3 + sin_(x)   # default given for y, can be given in terms of other arguments.
    available_digits:
    [   1
        3
        5
        7
        9
    ]
)
    do_something_()

defining_a_function_with_multiline_arguments_
(   times: int_
    greeting: string_
    name: string_("World")  # argument with a default
):      string_             # indent here is optional/aesthetic
    # "return" is optional for the last line of the block,
    # unless you're returning a multiline array/object.
    "${greeting}, ${name}! " * times

defining_a_function_with_multiline_return_values_
(   argument0: int_
):
[   value0: int_    # you may need to add comments because
    value1: str_    # the formatter may 1-line these otherwise
]
    do_something_(argument0)
    # here we can avoid the `return` since the internal
    # part of this object is not indented.
    [value0: argument0 + 3, value1: str_(argument0)]

# ALTERNATIVE: multiline return statement
defining_a_function_with_multiline_return_values_
(   argument0: int_
    argument1: str_
):  [value0: int_, value1: str_]
    do_something_(argument0)
    # this needs to `return` or `pass` since it looks like an indented block
    # otherwise, which would attach to the previous line like
    # `do_something_(argument0)[value0: ...]`
    return
    [   value0: argument0 + 3
        value1: argument1 + str_(argument0)
    ]
    # if you are in a situation where you can't return -- e.g., inside
    # an if-block where you want to pass a value back without returning --
    # use `pass`.

defining_another_function_that_returns_a_generic_
(   argument0: str_
    argument1: int_
):  some_generic_type_
[   type0_: int_
    type1_: str_
]
    do_something_(argument0)
    print_("got arguments ${argument0}, ${argument1}")
    return ...
```

Putting it all together in one big example:

```
some_line_continuation_example_variable:
        optional_expression_explicitly_at_plus_two_indent
    +   5 - some_function_
        (       another_optional_expression
            +   next_variable
            -   can_keep_going
            /   indefinitely
                r: 123.4
        )
```

### block parentheses and commas

You can use `{` ... `}` to define a block inline.  The braces block is grammatically
the same as a standard block, i.e., going to a new line and indenting to +1.
This is useful for short `if` statements, e.g., `if some_condition {do_something_()}`.
Similarly, you can return normal objects or reference objects in blocks via
`$[...]` or `$(...)`, respectively.

Similarly, note that commas are essentially equivalent to a new line and tabbing to the
same indent (indent +0).  This allows you to have multiple statements on one line,
in any block, by using commas.  E.g.,

```
# standard version:
if some_condition
    print_("toggling shutoff")
    shutdown_()

# comma version:
if some_condition
    # WARNING: NOT RECOMMENDED, since it's easy to accidentally skip reading
    # the statements that aren't first:
    print_("toggling shutoff"), shutdown_()

# block parentheses version
if some_condition { print_("toggling shutoff"), shutdown_() }
```

If the block parentheses encapsulate content over multiple lines, note that
the additional lines need to be tabbed to +1 indent to match the +1 indent given by `{`.
Multiline block parentheses are useful if you want to clearly delineate where your blocks
begin and end, which helps some editors navigate more quickly to the beginning/end of the block.

```
# multiline block parentheses via an optional `{`
if some_condition
{   print_("toggling shutdown")
    print_("waiting one more tick")
    print_("almost..."), print_("it's a bit weird to use comma statements")
    shutdown_()
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

* `int_`: signed big-integer
* `rtnl_`: rational number (e.g. an `int_` divided by a positive, non-zero `int_`)
* `str_`: array/sequence of utf8 bytes, but note that `string` is preferred for
    function arguments since it includes other containers which deterministically
    provide utf8 bytes.

Other types which have a fixed amount of memory:

* `null_`: should take up no memory, but can require an extra bit for an optional type.
* `flt_`: single-precision floating-point number, AKA `f32_`
* `dbl_`: double-precision floating-point number, AKA `f64_`
* `bool_`: can hold a True or False value
* `rune_`: a utf8 character, presumably held within an `i32`
* `u8_`: unsigned byte (can hold values from 0 to 255, inclusive)
* `u16_` : unsigned integer which can hold values from 0 to 65535, inclusive
* `u32_` : unsigned integer which can hold values from 0 to `2^32 - 1`, inclusive
* `u64_` : unsigned integer which can hold values from 0 to `2^64 - 1`, inclusive
* `uXYZ_` : unsigned integer which can hold values from 0 to `2^XYZ - 1`, inclusive,
    where `XYZ` is 128 to 512 in steps of 64, and generically we can use
    `unsigned_[Bits: count]: what Bits {8 {u8}, 16 {u16}, 32 {u32}, ..., else {disallowed}}`
* `count_` : `u64_` under the hood, intended to be <= `i64_ max_() + 1` to indicate the amount of something.
* `index_` : signed integer, `i64_` under the hood.  for indexing arrays starting at 0, can be negative
    to indicate we're counting from the back of the array.
* `ordinal_` : `u64_` under the hood.  for indexing arrays starting at 1.

and similarly for `i8_` to `i512_`, using two's complement.  For example,
`i8_` runs from -128 to 127, inclusive, and `u8_(wrap: i8_(-1))` equals `255`.
The corresponding generic is `signed_[bits: count]`.  We also define the
symmetric integers `s8_` to `s512_` using two's complement, but disallowing
the lowest negative value of the corresponding `i8_` to `i512_`, e.g.,
-128 for `s8_`.  This allows you to fit in a null type with no extra storage,
e.g., `one_of_[s8_, null_]` is exactly 8 bits, since it uses -128 for null.
(See [nullable classes](#nullable-classes) for more information.)
Symmetric integers are useful when you want to ensure that `-symmetric`
is actually the opposite sign of `symmetric`; `-i8_(-128)` is still `i8_(-128)`.
The corresponding generic for symmetric integers is `symmetric_[bits: count]`.

Note that the `ordinal_` type behaves exactly like a number but can be used
to index arrays starting at 1.  E.g., `array[ordinal_(1)]` corresponds to `array[index_(0)]`
(which is equivalent to other numeric but non-index types, e.g., `Array[0]`).
There is an automatic delta by +-1 when converting from `index_` to `ordinal_`
or vice versa, e.g., `ordinal_(index_(1)) == 2` and `index_(ordinal_(1)) == 0`.
Note however, that there's a bit of asymmetry here; non-index, numeric types
like `u64_`, `count_`, or `i32_` will convert to `index_` or `ordinal_` without any delta.
It's only when converting between `index_` and `ordinal_` that a delta occurs.

## casting between types

oh-lang attempts to make casting between types convenient and simple.
However, there are times where it's better to be explicit about the program's intention.
For example, if you are converting between two number types, but the number is *not*
representable in the other type, the run-time will return an error.  Therefore you'll
need to be explicit about rounding when casting floating point numbers to integers,
unless you are sure that the floating point number is an integer.  Even if the float
is an integer, the maximum floating point integer is larger than most fixed-width integer
types (e.g., `u32_` or `i64_`), so errors can be returned in that case.  The big-integer type
`int` will not have this latter issue, but may return errors depending on memory constraints.
Notice we use `assert` to shortcircuit function evaluation and return an error result
(like throwing).  See [errors and asserts](#errors-and-asserts) for more details.

```
# Going from a floating point number to an integer should be done carefully...
x: dbl_(5.43)
safe_cast: x int_()                 # Safe_cast is a result type (`hm_[ok_: int_, number_ er_]`)
# also OK: `safe_cast: int_(x)`.
q: x int_() assert_()               # panics since `x` is not representable as an integer
y: x round_(down) int_() assert_()  # y = 5.  equivalent to `x floor_()`
z: x round_(up) int_() assert_()    # z = 6.  equivalent to `x ceil_()`.
r: x round_() int_() assert_()      # r = 5.  rounds to closest integer, breaking ties at half
                                    #         to the integer larger in magnitude.

# Note, representable issues arise for conversions even between different integer types.
a: u32_(1234)
q: a u8_() assert_()                # RUN-TIME ERROR, `a` is not representable as a `u8`.
b: u8_(a & 255) assert_()           # OK, communicates intent and puts `a` into the correct range.
```

Casting to a complex type, e.g., `one_of_[int_, str_](some_value)` will pass through `some_value`
if it is an `int_` or a `str_`, otherwise try `int_(some_value)` if that is allowed, and finally
`str_(some_value)` if that is allowed.  If none of the above are allowed, the compiler will
throw an error.  Note that nullable types absorb errors in this way (and become null), so
`one_of_[int_, null_](some_safe_cast)` will be null if the cast was invalid, or an `int_` if the
cast was successful.

To define a conversion from one class to another, you can define a global function
or a class method, like this:

```
scaled8_:
[   # the actual value held by a `scaled8` is `Scaled_value / Scale`.
    @private
    scaled_value: u8
]
{   # static/class-level variable:
    @private
    scale: 32_u8

    m_(flt): hm_[ok_: m_, er_: one_of_[negative, too_big]]
        scaled_value: round_(flt * scale)
        if scaled_value < 0
            er_(negative)
        elif scaled_value > u8 max_()
            er_(too_big)
        else
            scaled8_(scaled_value u8_() ?? panic_())
        # probably a preferred way to implement this is with less logic,
        # and just return the `number_ er_` instead:
        # `what u8_(scaled_value) { ok. {scaled8_(ok)}, er. {er} }`

    # if there are no representability issues, you can create
    # a direct method to convert to `flt`;
    # this can be called like `flt_(scaled8)` or `scaled8 flt_()`.
    ::to_(): flt_
        # `u8_` types have a non-failing `flt_` method.
        scaled_value flt_() / scale flt_()

    # if you have representability issues, you can return a result instead.
    ::to_(): hm[ok_: int_, number_ er_]
        if scaled_value % scale != 0
            er_(not_an_integer)
        else
            scaled_value // scale
}

# global function; can also be called like `scaled8 dbl_()`.
dbl_(scaled8): dbl_
    # note that we can access private variables of the class *in this file*
    # but if we weren't in the same file we wouldn't have this access.
    scaled8 scaled_value dbl_() / scaled8 scale dbl_()

# global function which returns a result, can be called like `scaled8 u16_()`
u16_(scaled8): hm_[ok_: u16_, number_ er_]
    if scaled8 scaled_value % scaled8 scale != 0
        er_(not_an_integer)
    else
        scaled8 scaled_value // scaled8 scale
```

## types of types

Every variable has a reflexive type which describes the object/primitive that is held
in the variable, which can be accessed via the `type_case_` version of the
`variable_case` variable name.  

```
# implementation note: `int_` comes first so it gets tried first;
# `dbl_` will eat up many values that are integers, including `4`.
x; one_of_[int_, dbl_] = 4
y; x_ = 4.56    # use the type of `x` to define a variable `y`.
```

Note that the `type_case_` version of the `variable_case` name does not have
any information about the instance, so `x` is `one_of_[int_, dbl_]` in the above
example and `y` is an instance of the same `one_of_[int_, dbl_]` type.  For other
ways to handle different types within a `one_of_`, [go here](#one_of-types).

Some more examples:

```
vector3_: [x; dbl_, y; dbl_, z; dbl_]

my_vector3: vector3_(x: 1.2, y: -1.4, z: 1.6)

print(my_vector3_)              # prints `vector3`
print(vector3_ == my_vector3_)  # this prints true
```

Variables that refer to types cannot be mutable, so something
like `some_type; vector3` is not allowed.  This is to make it
easier to reason about types.

## type overloads

Similar to defining a function overload, we can define type overloads for generic types.
For example, the generic result class in oh-lang is `hm_[ok_, er_]`, which
encapsulates an ok value (`ok_`) or a non-nullable error (`er_`).  For your custom class you
may not want to specify `hm_[ok_: my_ok_type_, er_: my_class_er_]` all the time for your custom
error type `my_class_er_`, so you can define `hm_[of_]: hm_[ok_: of_, er_: my_class_er_]` and
use e.g. `hm_[int_]` to return an integer or an error of type `my_class_er_`.  Shadowing variables is
invalid in oh-lang, but overloads are valid.  Note however that we disallow redefining
an overload, as that would be the equivalent of shadowing.

## type manipulation

Plain-old-data objects can be thought of as merging all fields
in this way:
```
object_ == merge_[object_ fields_(), {[$field name: $field value_]}]
```

TODO: good ways to do keys and values for an object type (e.g., like TypeScript).
see if there's a better way to do it, e.g., `object_ valued_[{um_[$value_]}]`, so
it's easy to see that all field names are the same, just values that change.

Here are some examples of changing the nested fields on an object
or a container, e.g., to convert an array or object to one containing futures.

```
# base case, needs specialization.
nest_[m_, new_[of_]: ~n_]: disallowed_

# container specialization.
# e.g., `array_[int_] nest_[{um_[$of_]}] == array_[um_[int_]]`,
# or you can do `nest_[m_: array_[int_], {um_[$of_]}]` for the same effect.
nest_[c_: container_, m_: ~c_[of_: ~nested_, ~at_], new_[of_]: ~n_]: c_[of_: new_[nested_], at_]

# object specialization.
# e.g., `[x: int_, y: str_] nest_[{hm_[ok_: $of_, er_: some_er_]}]`
# or you can do `nest_[{hm_[ok_: $of_, er_: some_er_]}, m_: [x: int_, y: str_]]` for the same effect.
# to make `[x: hm_[ok_: int_, er_: some_er_], y: hm_[ok_: str_, er_: some_er_]]`,
nest_[m_: object_, new_[of_]: ~n_]: merge_
[   m_ fields_()
    {[$field name: new_[$field value_]]}
]
```

Here are some examples of unnesting fields on an object/future/result.

```
# base case, needs specialization
unnest_[of_]: disallowed_

# container specialization
# e.g., `unnest_[array_[int_]] == int_`
unnest_[container_[of_: ~nested_, ~_at_]]: nested_

# `set` needs its own specialization because it has interesting
# `container_` dynamics.  e.g., `unnest_[set_[str_]] == str_`.
unnest_[set_[~nested_]]: nested_

# future specialization
# e.g., `unnest_[um_[str_]] == str_`.
unnest_[um_[~nested_]]: nested_

# result specialization
# e.g., `unnest_[hm_[ok_: str_, er_: int_]] == str`.
unnest_[hm_[ok_: ~nested_, ~_er_]]: _nested

# null specialization
# e.g., `unnest_[int_?] == int`.
unnest_[one_of_[...~nested_, null_]]: one_of_[...nested_]
```

Note that if we have a function that returns a type, we must use brackets, e.g.,
`the_function_[...]: the_return_type_`, but we can use instances like booleans
or numbers inside of the brackets (e.g., `array_[3, int_]` for a fixed size array type).
Conversely, if we have a function that returns an instance, we must use parentheses,
e.g., `the_function_(...): instance_type_`.  In either case, we can use a type as
an argument, e.g., `nullable_(of_): bool_` or `array3_[of_]: array_[3, of_]`.
Type functions can be specialized in the manner shown above, but instance functions
cannot be.  TODO: would we want to support that at some point??

Here is some nullable type manipulation:

```
# the `null` type should not be considered nullable because there's
# nothing that can be unnulled, so ensure there's something not-null in a nullable.
#   nullable_(one_of_[dbl_, int_, str_]) == False
#   nullable_(one_of_[dbl_, int_, null_]) == True
#   nullable_(one_of_[int_, null_]) == True
#   nullable_(one_of_[null_]) == False
#   nullable_(null_) == False
nullable_(of_): of_ contains_(not_[null_], null_)

# examples
#   unnull_[int_] == int_
#   unnull_[int_?] == int_
#   unnull_[one_of_[array_[int_], set_[dbl_], null_]] == one_of_[array_[int_], set_[dbl_]]
unnull_[of_]: if nullable_(of_) {unnest_[of_]} else {of_}

# a definition without nullable, using template specialization:
unnull_[of_]: of_
unnull_[one_of_[...~nested_, null_]]: one_of_[...nested_]
```

# operators and precedence

TODO: add : , ; ?? postfix/prefix ?
TODO: add ... for dereferencing.  maybe we also allow it for spreading out an object into function arguments,
e.g., `my_function(A: 3, B: 2, ...My_object)` will call `my_function(A: 3, B: 4, C: 5)` if `My_object == [B: 4, C: 5]`.

| Precedence| Operator  | Name                      | Type/Usage        | Associativity |
|:---------:|:---------:|:--------------------------|:-----------------:|:-------------:|
|   1       |   `()`    | parentheses               | grouping: `(a)`   | ??            |
|           |   `[]`    | parentheses               | grouping: `[a]`   |               |
|           |   `{}`    | parentheses               | grouping: `{a}`   |               |
|           | `\\x/y/z` | library module import     | special: `\\a/b`  |               |
|           | `\/x/y/z` | relative module import    | special: `\/a/b`  |               |
|   2       |  ` ()`    | function call             | on fn: `a_(b)`    | LTR           |
|           |   `::`    | impure read scope         | binary: `a::b`    | LTR           |
|           |   `;;`    | impure read/write scope   | binary: `a;;b`    |               |
|           |   ` `     | implicit member access    | binary: `a b`     |               |
|           |   ` []`   | subscript                 | binary: `a[b]`    |               |
|           |   `!`     | postfix moot = move+renew | unary:  `a!`      |               |
|           |   `?`     | postfix nullable          | unary: `a?`/`a_?` |               |
|           |   `??`    | nullish OR                | binary: `a??b`    |               |
|   3       |   `^`     | superscript/power         | binary: `a^b`     | RTL           |
|           |   `**`    | also superscript/power    | binary: `a**b`    |               |
|           |   `--`    | unary decrement           | unary:  `--a`     |               |
|           |   `++`    | unary increment           | unary:  `++a`     |               |
|           |   `~`     | template/generic scope    | unary:  `~b_`     |               |
|   4       |   `<>`    | bitwise flip              | unary:  `<>a`     | RTL           |
|           |   `-`     | unary minus               | unary:  `-a`      |               |
|           |   `+`     | unary plus                | unary:  `+a`      |               |
|           |   `!`     | prefix boolean not        | unary:  `!a`      |               |
|   5       |   `>>`    | bitwise right shift       | binary: `a>>b`    | LTR           |
|           |   `<<`    | bitwise left shift        | binary: `a<<b`    |               |
|   6       |   `*`     | multiply                  | binary: `a*b`     | LTR           |
|           |   `/`     | divide                    | binary: `a/b`     |               |
|           |   `%`     | modulus                   | binary: `a%b`     |               |
|           |   `//`    | integer divide            | binary: `a//b`    |               |
|           |   `%%`    | remainder after //        | binary: `a%%b`    |               |
|   7       |   `+`     | add                       | binary: `a+b`     | LTR           |
|           |   `-`     | subtract                  | binary: `a-b`     |               |
|   8       |   `&`     | bitwise AND + string cat  | binary: `a&b`     |               |
|           |   `\|`    | bitwise OR                | binary: `a\|b`    |               |
|           |   `><`    | bitwise XOR               | binary: `a><b`    |               |
|   9       |   `==`    | equality                  | binary: `a==b`    | LTR           |
|           |   `!=`    | inequality                | binary: `a!=b`    |               |
|   10      |   `&&`    | logical AND               | binary: `a && b`  | LTR           |
|           |  `\|\|`   | logical OR                | binary: `a \|\| b`|               |
|           |  `!\|`    | logical XOR               | binary: `a !\| b` |               |
|   11      |   `=`     | assignment                | binary: `a = b`   | LTR           |
|           |  `???=`   | compound assignment       | binary: `a += b`  |               |
|           |   `<->`   | swap                      | binary: `a <-> b` |               |
|   12      |   `->`    | ergo                      | binary: `a -> b`  | LTR           |
|   13      |   `,`     | comma                     | binary/postfix    | LTR           |


TODO: discussion on `~`

## function calls

Function calls are assumed whenever a function identifier (i.e., `function_case_`)
occurs before a parenthetical expression.  E.g., `print_(x)` where `x` is a variable name or other
primitive constant (like `5`), or `any_function_name_(any + expression / here)`.
In case a function returns another function, you can also chain like this:
`get_function_(x)(y, z)` to call the returned function with `(y, z)`.

It is recommended to use parentheses where possible, to help people see the flow more easily.
E.g., `some_function_(some_instance some_field some_method_()) final_field` looks pretty complicated.
This would compile as `(some_function(some_instance)::some_field::some_method())::final_field`,
and including these parentheses would help others follow the flow.  Even better would be to
add descriptive variables as intermediate steps.

We don't allow for implicitly currying functions in oh-lang,
but you can explicitly curry like this:

```
some_function_(x: int_, y; dbl_, z. str_):
    print_("something cool with ${x}, ${y}, and ${z}")

curried_function_(z. str_): some_function_(x: 5, y; 2.4, .z)

# or you can make it almost implicit like this:
$curried_function_{some_function_(x: 5, y; 2.4, .$z)}:
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
my_function_(x: int_): int_
    # define a nested function:
    # COMPILE ERROR
    do_stuff_(x: int_): null_
        # is this the `x` that's passed in from `my_function_`? or from `do_stuff_`?
        # most languages will shadow so that `x` is now `do_stuff_`'s argument,
        # but oh-lang does not allow shadowing.
        print_(x)
    do_stuff(x)
    do_stuff(x: x // 2)
    do_stuff(x: x // 4)
    x // 8
```

There are two ways to get around this; one is [hiding variables](#hiding-variables).
In the above example, the best way is to use a namespace for one or both conflicts.
Namespaced variables have capital letters inside them, which are ignored when matching
function arguments, e.g., `my_function_(MY_NAMESPACE_my_variable_name: int): null_`
where `MY_NAMESPACE_` is ignored when matching arguments, so you'd call with e.g.,
`my_function_(my_variable_name: 3)` or even `my_function_(:ANOTHER_my_variable_name)`
if that namespaced variable `ANOTHER_my_variable_name` is in scope.  In fact, capital
letters will be removed from anywhere in the identifier when matching arguments, so
`MY_variable_OTHER_x_OK` would match to `variable_x` in a function argument.
Namespaces are most useful in function arguments, but they can
annotate any variable that you're declaring.  It is recommended to namespace
the "outer" variable so you don't accidentally use it in the inner scope.

```
my_function_(OUTER_x: int_): int_
    # nested function is OK due to namespace:
    do_stuff_(x: int_): null
        # inner scope, any usage of `OUTER_x` would be clearly intentional.
        print(x)
    # NOTE: we don't need to rename `x: OUTER_x` because `OUTER_x` already resolves to `x`.
    do_stuff_(OUTER_x)
    do_stuff_(x: OUTER_x // 2)
    do_stuff_(x: OUTER_x // 4)
    OUTER_x // 8
```

If it's difficult to namespace the outer variable (e.g., because you don't
want to delta many lines), you can use `@hide` to ensure you don't use the
other value accidentally.

```
my_function_(x: int_): int_
    # nested function is OK due to namespace:
    do_stuff_(OTHER_x: int_): null_
        # inner scope, usage of `x` might be accidental, so let's hide:
        @hide x
        ...
        print(OTHER_x)  # OK
        print(x)        # COMPILE ERROR, `x` was hidden from this scope.
        ...
    do_stuff_(x)
    do_stuff_(x: x // 2)
    do_stuff_(x: x // 4)
    x // 8
```

Similarly, you can define new variables with namespaces, in case you need a new variable
in the current space.  This might be useful in a class method like this:

```
my_class_: [x; dbl_]
{   # this is a situation where you might like to use namespaces.
    ;;do_something_(NEW_x. dbl_): dbl_
        # NOTE: if you just want to create a swapper, you should
        # probably just use this idiom: `;;x_(dbl;): null_ {m x <-> dbl}`.
        OLD_x: m x!
        m x = NEW_x
        OLD_x
}
```

One of the most convenient uses for namespaces is the ability to elide argument
names when calling functions.  E.g., if you have a function which takes a variable named `x`,
but you already have a different one in scope, you can create a new variable with a namespace
`EXAMPLE_NAMESPACE_x: my_new_x_value` and then pass it into the function as
`my_function_(EXAMPLE_NAMESPACE_x)` instead of `my_function_(x: EXAMPLE_NAMESPACE_x)`.
This also works with default-named variables, which is a primary use-case.

```
some_function_(INPUT_index): null_
    # `INPUT_index` is a default-named variable of type `index_`, but we refer to it
    # within this scope using `INPUT_index`.
    even_(index): bool_
        index % 2 == 0
    # you can define other namespaces inline as well:
    INPUT_index each ANOTHER_index:
        if even_(ANOTHER_index)
            print_(ANOTHER_index)
        
x: index_ = 100
some_function_(x)   # note that we don't need to call as `some_function_(index: x)`
                    # nor `some_function_(INPUT_index: x)` (definitely not idiomatic).
```

You can use the same namespace for multiple variables, e.g., `INPUT_rune` and `INPUT_string`,
as long as the variable names don't overlap.

### full list of reserved namespaces

* `FIRST_` - for the first operand in a binary operation (where order matters)
* `SECOND_` - for the second operand in a binary operation (where order matters)
* `NAMED_` - for arguments that should be explicitly named in [functions](#defining-generic-functions)

TODO: maybe change `NAMED_` to `@as` or `@named`.

## member access operators `::`, `;;`, ` `, and subscripts `[]`

We use `::`, `;;`, and ` ` (member access) for accessing variables or functions that belong to
another object.  The `::` operator ensures that the RHS operand is read only, not write,
so that both LHS and RHS variables remain constant.  Oppositely, the `;;` scope operator passes
the RHS operand as writable, and therefore cannot be used if the LHS variable is readonly.
The implicit member access operator ` ` is equivalent to `::` when the LHS is a readonly variable
and `;;` when the LHS is a writable variable.  When declaring class methods, `::` and `;;` can be
unary prefixes to indicate readonly/writable-instance class methods.  They are shorthand for adding a
readonly/writable `m` (self/this) as an argument.

```
example_class_: [x: int_, y: dbl_]
{   ;;renew_(m x: int_, m y: dbl_): null_
        print_("x ${x} y ${y}")

    # this `::` prefix is shorthand for `multiply_(m:, ...): dbl_`:
    ::multiply_(z: dbl_): dbl_
        m x * m y * z
}
```


```
some_class_: [x: dbl_, y: dbl_, a; array_[str_]]
Some_class; some_class_(x: 1, y: 2.3, a: ["hello", "world"])
print_(some_class::a)       # prints ["hello", "world"] with a readonly reference overload
print_(some_class::a[1])    # prints "world"
print_(some_class a[1])     # also prints "world", using ` ` (member access)
some_class;;a[4] = "love"   # the fifth element is love.
some_class::a[7] = "oops"   # COMPILE ERROR, `::` means the array should be readonly.
some_class;;a[7] = "no problem"

nested_class; array_[some_class_]
nested_class[1] x = 1.234        # creates a default [0] and [1], sets [1]'s x to 1.234
nested_class[3] a[4] = "oops"    # creates a default [2] and [3], sets [3]'s a to ["", "", "", "", "oops"]
```

For class methods, `;;` (`::`) selects the overload with a writable (readonly) class
instance, respectively.  For example, the `array` class has overloads for sorting, (1) which
does not change the instance but returns a sorted copy of the array (`::sort(): m`), and
(2) one which sorts in place (`;;sort(): null`).  The ` ` (member access) operator will use
`a:` if the LHS is a readonly variable or `a;` if the LHS is writable.  Some examples in code:

```
# there are better ways to get a median, but just to showcase member access:
get_median_slow_(array_[int_]): hm_[ok_: int_, er_: string_]
    if array count_() == 0
        return er_("no elements in array, can't get median.")
    # make a copy of the array, but no longer allow access to it (via `@hide`):
    SORTED_array: @hide array sort_()   # same as `array::sort_()` since `array` is readonly.
    ok(SORTED_array[SORTED_array count_() // 2])

# sorts the array and returns the median.
get_median_slow_(array[int_];): hm_[ok_: int_, er_: string_]
    if array count_() == 0
        return er_("no elements in array, can't get median.")
    array sort_()   # same as `array;;sort_()` since `array` is writable.
    ok_(array[array count_() // 2])
```

Note that if the LHS is readonly, you will not be able to use a `;;` method.
To sum up, if the LHS is writable, you can use `;;` or `::`, and ` ` (member access) will
effectively be `;;`.  If the LHS is readonly, you can only use `::` and ` `, which are equivalent.

Subscripts `[]` have the same binding strength as member access operators since they are conceptually
similar operations.  This allows for operations like `++a[3]` meaning `++(a[3])` and
`--a b c[3]` equivalent to `--(((a;;b);;c)[3])`.  Member access binds stronger than exponentation
so that operations like `a b[c]^3` mean `((a::b)[c])^3`.

Note that `something_() nested_field` becomes `(something_())::nested_field` due to
the function call having higher precedence, and is often the idiomatic way to request a
specific overload via the return type/name.  You can also use destructuring if you want
to keep a variable for multiple uses: `[nested_field]: something_()`.

## prefix and postfix question marks `?`

Generally speaking, if we want a variable `x` to be nullable, we use the postfix `?`
operator when declaring `x`, and bind it to the variable itself, e.g., `x?: int_`.

TODO: i think i prefer `do_something_(x: ?my_value_for_x)` so it's more obviously
different than `do_something_(x?: my_value_for_x)` when `my_value_for_x` is nullable.
then `do_something_(;?x)` also makes sense as `do_something_(x; ?x)`.

Prefix `?` can be used to short-circuit function evaluation if an argument is null.
For a function like `do_something_(x?: int_): null_`, we can use `do_something_(?x: my_value_for_x)`
to indicate that we don't want to call `do_something_` if `my_value_for_x` is null;
we'll simply return `null`.  E.g., `do_something_(?x: my_value_for_x)` is equivalent
to `if my_value_for_x == null {null} else {do_something_(x: my_value_for_x)}`.
In case `x` is already in scope, we elide the the variable name via `do_something_(?x)`.
It works similarly for writable reference (or temporary) arguments:
`do_something_(x?; int_): null_` could be called like `do_something_(?x; my_value_for_x)`
or `do_something(?;x)` if `x` is in scope.

There's also an infix `??` type which is a nullish or.
`x y ?? z` will choose `x y` if it is non-null, otherwise `z`.

## prefix and postfix exclamation points

The operator `!` is always unary (except when combined with equals for not equals,
e.g., `!=`).  It can act as a prefix operator "not", e.g., `!a`, pronounced "not A",
or a postfix operator on a variable, e.g., `z!`, pronounced "Z mooted" (or "moot Z").  In the first
example, prefix `!` calls the `!(m:): bool_` (or `::!(): bool_`) method defined on `a_`, which creates a
temporary value of the boolean opposite of `a` without modifying `a`.  In the second
case, it calls a built-in method on `z`, which moves the current data out of `z` into
a temporary instance of whatever type `z` is (i.e., `z_`), and resets `z` to a blank/default state.
The method would look like `::()!: m_` or `(m:)!: m_`, but again this is defined for you.
This is a "move and reset" operation, or "moot" for short.  Overloads for prefix `!`
should follow the rule that, after e.g., `z!`, checking whether `z` evaluates to false,
i.e., by `!z`, should return true.

Note, it's easier to think about positive boolean actions sometimes than negatives,
so we allow defining either `!!(m:): bool_` (i.e., `::!!(): bool_`) or `!(m:): bool_` on a class,
the former allowing you to cast a value, e.g., `a`, to its positive boolean form `!!a`, pronounced
"not not A."  Note, you cannot define both `!` and `!!` overloads for a class, since
that would make things like `!!!` ambiguous.

## superscripts/exponentiation

Note that exponentiation -- `^` and `**` which are equivalent --
binds less strongly than function calls and member access.  So something like `a[b]^2` will be
equivalent to `(a[b])^2` and `a b^3` is equivalent to `(a::b)^3`.

## bitshifts `<<` and `>>`

The notation `a << b`, called "bitshift left", means to multiply `a` by `2^b`.  For example, 
`a << 1 == a * 2`, `a << 2 == a * 4`, and `a << 3 == a * 8`.  Conversely, "bitshift right"
`a >> b` means to divide `a` by `2^b`.  Typically, we use bitshifts `<<` and `>>`
only for fixed-width integers, so that `a >> 5 == a // 32`, but there are overloads
for other types that will do the expected full division.  For floats, e.g., 16.0 >> 5 == 0.5.
Note that `a << 0 == a >> 0 == a`, and that negating the second operand is the same
as switching the operation, i.e., `a << b == a >> -b`.

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
E.g., `dbl_(3/4) == 0.75` or `6/4 == rtnl_(3)/rtnl_(2)`.

The integer division operator, `//`, will return an integer, rounded towards zero, e.g.,`3//4 == 0`
and `-3//4 == 0`.  Also, `5//4 == 1` and `-5//4 == -1`, and `12 // 3 == 4` as expected.
If any operand is a double, the resulting value will be an integer double, e.g.,
`5.1 // 2 == 2.0`.

The modulus operator, `%`, will put the first operand into the range given by the second operand.
E.g., `5 % 4 == 1`, `123.45 % 1 == 0.45`, and `-3 % 7 == 4`.
Mathematically, we use the relation `a % b == a - b * floor(a/b)`.

The remainder operator, `%%`, has the property that `a %% b == a - b * (a // b)`;
i.e., it is the remainder after integer division, and corresponds to the C/C++
`%` operator.  The remainder operator, `%%`, differs from the modulus, `%`,
when the operands have opposing signs.  E.g., `-3 % 7 == -3` while
`-3 %% 7 == 4`.  Here's a table of more examples:

|  `a`  |  `b`  | `floor_(a/b)` |  `a % b`  | `a // b`  | `a %% b`  |
|:-----:|:-----:|:-------------:|:---------:|:---------:|:---------:|
|   1   |   2   |      0        |     1     |     0     |     1     |
|  -1   |   2   |     -1        |     1     |     0     |    -1     |
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
This allows chaining like `w >= x < y <= z`, which will evaluate as truthy iff
`w >= x`, `x < y`, and `y <= z`.  Note that these expressions are evaluated
left-to-right and the first inequality to fail will stop any further evaluations
or expressions from executing.

Internally, `x < y` becomes a class which holds onto a value or reference of `y`,
so that it can be chained.  Any future right operands take over the spot of `y`.
Note, oh-lang doesn't expose this internal class,
so `q: x < y > z` instantiates `q` as a boolean.
TODO: we probably should expose so that it can be asked for as a return overload.

## and/or/xor operators

If you are looking for bitwise `AND`, `OR`, and `XOR`, they are `&`, `|`, and `><`, respectively.

The operators `and` and `or` act the same as JavaScript `&&` and `||`, as long as the
left hand side is not nullable.  `xor` is an "exclusive or" operator.

The `or` operation `x or y` has type `one_of_[x_, y_]` (for `x: x_` and `y: y_`).
If `x` evaluates to truthy (i.e., `!!x == true`), then the return value of `x or y` will be `x`.
Otherwise, the return value will be `y`.  Note in a conditional, e.g., `if x or y`, we'll always
cast to boolean implicitly (i.e., `if bool_(x or y)` explicitly).

Similarly, the `and` operation `x and y` also has type `one_of_[x_, y_]`.  If `x` is falsey,
then the return value will be `x`.  If `x` is truthy, the return value will be `y`.
Again, in a conditional, we'll cast `x and y` to a boolean.

If the LHS of the expression can take a nullable, then there is a slight modification.
`x or y` will be `one_of_[x_, y_, null_]` and `x and y` will be `one_of_[y_, null_]`.
The result will be `null` if both (either) operands are falsey for `or` (`and`).

```
non_null_or: x or y         # non_null_or: if x {x} else {y}
non_null_and: x and y       # non_null_and: if !x {x} else {y}
nullable_or?: x or y        # nullable_or?: if x {x} elif y {y} else {null}
nullable_and?: x and y      # nullable_and?: if !!x and !!y {null} else {y}
```

This makes things similar to the `xor` operator, but `xor` always requires a nullable LHS.
The exclusive-or operation `x xor y` has type `one_of_[x_, y_, null_]`, and will return `null`
if both `x` and `y` are truthy or if they are both falsey.  If just one of the operands
is truthy, the result will be the truthy operand.  An example implementation:

```
# you can define it as nullable via `xor_(~x, ~y): one_of_[x_, y_, null_]` or like this:
xor_(~x, ~y)?: one_of_[x_, y_]
    x_is_true: bool_(x)     # `x_is_true: !!x` is also ok.
    y_is_true: bool_(y)
    if x_is_true
        if y_is_true {null} else {x}
    elif y_is_true
        y
    else
        null
```

Thus `xor` will thus return a nullable value, unless you do an assert.

```
nullable_xor?: x xor y
non_null_xor: x xor y assert_() # will shortcircuit this block if `x xor y` is null
```

## reassignment operators

Note that `:`, `;`, and `.` can assign values if they're also being declared.
Thus, `=` is only used for reassignment.  Many binary (two operand) operators
such as `*`, `/`, `+`, `-`, etc., also support being paired with reassignment.
As long as `x @op y` has the same type as `x`, then we can do `x @op = y` for
shorthand of `x = x @op y` for any eligible binary operator `@op`.  Examples
include `x -= 5`, `y &= 0x12`, etc.

Swapping two variables is accomplished by something like `a <-> b`.
Swap uses `<->` since `<=>` is reserved for a future spaceship operator
(encompassing `<`, `<=`, `==`, `=>` and `>` in one).  As a function, swap
would require mutable variables, e.g., `;;x_(SWAP_x;): null_`.
If you define `swap` in this way for your custom class, it will be available
via the shorthand notation `some_class x <-> 1234`.

## ergo operator

`->` is called the "ergo operator" and is used in conditional logic for
more fine-grained flow control to define a `then_` instance which can
break out of loops in more interesting (possibly *less readable*) ways.
Use sparingly.

```
x: if some_condition -> then:
    if other_condition
        then exit_(5)
    then exit_(7)
else -> then:
    then exit_(10)

# the above is equivalent to the following:
x: if some_condition { if other_condition {5} else {7} } else {10}
```

See [then statements](#then-statements) for more examples and details.

# variables

Variables are named using `variable_case` identifiers.  The `:` symbol is used
to declare deeply constant, non-reassignable variables, and `;` is used to declare
writable, reassignable variables.  Note when passed in as arguments to a function,
`:` has a slightly different meaning; a variable with `:` is readonly and not
necessarily deeply constant.  That will be discussed more in
[passing by reference or by value](#pass-by-reference-or-pass-by-value).

```
# declaring and setting a non-reassignable variable that holds a big integer
y: int_ = 5
# also equivalent: `y: 5` or `y: int_(5)`, or `y: 5_int`

# using the variable:
print_(y * 30)

y = 123     # COMPILER ERROR, y is readonly and thus non-reassignable.
y += 3      # COMPILER ERROR, y is readonly and here deeply constant.
```

Mutable/reassignable/non-constant variables can use `variable_name = expression`
after their first initialization, but they must be declared with a `;` symbol.

```
# declaring a reassignable variable that holds a big integer
x; int_

# X is default-initialized to 0 if not specified.
x += 5      # now x == 5 is true.

# you can also define the value inline as well:
w; 7
# also equivalent, if you want to be explicit about the type.
w; int_ = 7
```

Note that we use `;` and `:` as if it were an annotation on the variable name (rather
than the type) so that constant variables are deeply constant, and writable
variables are modifiable/reassignable.  [Go here](#nestedobject-types)
for details on how this works with nested fields, but we think we've
come up with a more sane approach than C++ and JavaScript's `const`.

## nullable variable types

To make it easy to indicate when a variable can be nullable, we reserve the question mark
symbol, `?`, placed just after the variable name like `x?: int_`.  The default value for
an optional type is `null`.

TODO: do we want to allow the `one_of_[..., null_]` option?  this breaks the requirement
to always annotate variables with `x?: ...` if they are null, which i think we want to
keep/require for comptime considerations.

For an optional type with more than one non-null type, we use
`y?: one_of_[some_type_, another_type_]` or equivalently,
`y: one_of_[some_type_, another_type_, null_]` (where `null_` comes last).
Note that `null_` should come last for casts to work correctly (e.g.,
`one_of_[null_, int_](1234)` would cast to null rather than `int_(1234)`).
Normally the first value in a `one_of_` is the default, but if `null_` is an option,
then null is the default.  

In either case, you can use `;` instead of `:` to indicate that the variable is writable.
Note that if you are defining a nullable variable inline, you should
prefix the operator with a `?`, e.g., `x?: nullable_result_(...)`.  It is a compiler error
if a declared variable is nullable but `?` is not used, since we want the programmer to be
aware of the fact that the variable could be null, even though the program will take care
of null checks automatically and safely.  The `?` operator is required for any `one_of_` that
could take on a `null` value, e.g., `one_of_[null, bread, tomato, mozzarella]`.
TODO: `one_of_[null, ...]` would collapse to `one_of_[...]` if we strictly consider `null`
as a missing/non-existent value.

One of the cool features of oh-lang is that we don't require the programmer
to check for null on a nullable type before using it.  The executable will automatically
check for null on variables that can be null.  If a function can be null, the executable
will return null if the function is null, or otherwise execute the function.
This is also helpful for method chaining on classes (see more on those below).
If your code calls a method on an instance that is null, a null will be
returned instead (and the method will not be called).

```
# define a class with a method called `some_method_`:
some_class_: []{ ::some_method_(): int }

nullable?; some_class_ = null

value?: nullable some_method_() # `value` has type `one_of_[int_, null_]` now,
                                # so it needs to be defined with `?`

# eventually we want to support things like this, where the compiler
# can tell if the type is nullable or not:
if nullable != null
    non_null_value: nullable some_method_() # `non_null_value` here must be `int_`.

# however the easier compiler thing to do is use the `is` reduction.
if nullable is some_class:
    non_null_value: some_class some_method_()   # `non_null_value` here must be `int_`.
```

See the [`is` operator](#is-operator) for more details.

It is not allowed to implicitly cast from a nullable type to a non-nullable type,
e.g., `value: nullable some_method_()`.  The compiler will require that we define
`value` with `?:`, or that we explicitly cast via whatever ending type we desire,
e.g., `value: int_(nullable some_method_())`.  Note that `whatever_type_(null)` is
the same as `whatever_type_()`, i.e., the default constructor, and number types
(e.g., `int_()` or `flt_()`)  default to 0.
TODO: doesn't this break the requirement that we pass in the argument as `?:`?
would we need `value: int_(from?: nullable some_method_())`?
probably best to just use `value: nullable some_method_() ?? 0`.

Optional functions are defined in a similar way (cf. section on [nullable functions](#nullable-functions)),
with the `?` just after the function name, e.g., `some_function_?(...args): return_type_`.

## nullable classes

We will allow defining a nullable type by taking a type and specifying what value
is null on it.  For example, the symmetric type `s8_` defines null as `-128` like this:

```
s8_?: s8_
{   null: -128_i8
    ::is_(null_): m == -128_i8
}
```

Similarly, `f32_?` and `f64_?` indicate that `nan` is null via `{null: nan, ::is_(null_): is_nan_(m)}`,
so that you can define e.g. a nullable `f32_` in exactly 32 bits.  To get this functionality,
you must declare your variable as type `s8_?` or `f32_?`, so that the nullable checks
kick in.  Note that while we offer a way to create a null via `f32?: null`, we always
convert equality checks like `f32 == null` into `f32 is_(null_)` due to the fact that
`::is_(null_)` can handle more edge cases (like `nan`, which is not equal to itself).

If you are defining a class and want to also declare the nullable at the same time, you
can do one of the following:

```
my_class_: [@private some_state: int_]
{   ;;renew_(m some_state: int_): {}

    ::normal_method_(): int_
        m some_state + 3

    # the nullable definition, inside a class:
    ?: m_
    {   null: [some_state: -1]
        ::is_null_(): m some_state < 0

        ::additional_null_method_(): int_
            if m is_null_() {0}
            else {m some_state * 5}
    }
}

# nullable definition, outside a class (but same file).
# both internal/external definitions aren't required of course.
my_class_?: my_class_
{   null: [some_state: -1]
    ::is_null_(): m some_state < 0
    ::additional_null_method_(): int_
        if m is_null_() {0}
        else {m some_state * 5}
}
```

Note that any `one_of_` that can be null gets nullable methods.  They are defined globally
since we don't want to make users extend from a base nullable class.

```
# nullish or.
# `nullable ?? x` to return `x` if `nullable` is null,
# otherwise the non-null value in `nullable`.
nullish_or_(~FIRST_a?., SECOND_a.): a_
    what FIRST_a
        non_null: {non_null}
        null {SECOND_a}

# boolean or.
# `nullable || x` to return `x` if `nullable` is null or falsey,
# otherwise the non-null truthy value in `nullable`.
or_(~FIRST_a?., SECOND_a.): a
    what FIRST_a
        non_null:
            if non_null
                non_null
            else
                SECOND_a
        null {SECOND_a}
```

We'll support more complicated pattern matching (like in Rust) using
the `where` operator.  The shorter version of the above `what` statement is:

```
or_(~FIRST_a?., SECOND_a.): a
    what FIRST_a
        NON_NULL_a: where !!NON_NULL_a
            NON_NULL_a
        null
            SECOND_a
```

In this case, you can think of the `what` cases as being evaluated in order,
and the first one to match will be executed.  Internally there are more optimizations.

## nested/object types

You can declare an object type inline with nested fields.  The nested fields defined
with `:` are readonly, and `;` are writable.

```
vector; [x: dbl_, y: dbl_, z: dbl_] = [x: 4, y: 3, z: 1.5]
vector x += 4   # COMPILER ERROR, field `x` of object is readonly 

# note however, as defined, vector is reassignable since it was defined with `;`:
vector = [x: 1, y: 7.2]
# note, missing fields will be default-initialized.
vector z == 0   # should be true.

# to make an object variable readonly, use : when defining:
vector2: [x: 3.75, y: 3.25]
# or you can use `:` with an explicit type specifier and then `=`:
vector2: [x: dbl, y: dbl] = [x: 3.75, y: 3.25]
# then these operations are invalid:
vector2 x += 3          # COMPILER ERROR, variable is readonly, field cannot be modified
vector2 = [x: 1, y: 2]  # COMPILER ERROR, variable is readonly, cannot be reassigned
```

You can define a type/interface for objects you use multiple times.

```
# a plain-old-data class with 3 non-reassignable fields, x, y, z:
vector3_: [x: dbl_, y: dbl_, z: dbl_]

# you can use `vector3_` now like any other type, e.g.:
vector3: vector3_(x: 5, y: 10)
```

We also allow type definitions with writable fields, e.g. `[x; int_, y; dbl_]`.
Depending on how the variable is defined, however, you may not be able to change
the fields once they are set.  If you define the variable with `;`, then you
can reassign the variable and thus modify the writable fields.  But if you define the
variable with `:`, the object fields are readonly, regardless of the field definitions.
Readonly fields on an object are normally deeply constant, unless the instance is
writable and is reset (either via `renew` or reassignment).  This allows you to
effectively change any internal readonly fields, but only in the constructor.

```
# mix_match has one writable field and one readonly field:
mix_match_: [wr; dbl_, ro: dbl_]

# when defined with `;`, the object `mutable_mix` is writable: mutable and reassignable.
mutable_mix; mix_match_ = [wr: 3, ro: 4]
mutable_mix = mix_match_(wr: 6, ro: 3)  # OK, mutable_mix is writable and thus reassignable
mutable_mix renew_(wr: 100, ro: 300)    # OK, will update `ro` to 300 and `wr` to 100
mutable_mix wr += 4                     # OK, mutable_mix is writable and this field is writable
mutable_mix ro -= 1                     # COMPILE ERROR, mutable_mix is writable but this field is readonly.
                                        # if you want to modify the `ro` field, you need to reassign
                                        # the variable completely or call `renew_`.

# when defined with `:`, the object is readonly, so its fields cannot be changed:
readonly_mix: mix_match_ = [wr: 5, ro: 3]
readonly_mix = mix_match_(wr: 6, ro: 4) # COMPILE ERROR, readonly_mix is readonly, thus non-reassignable
readonly_mix renew_(wr: 7, ro: 5)       # COMPILE ERROR, readonly_mix is readonly, thus non-renewable
readonly_mix wr += 4                    # COMPILE ERROR, readonly_mix is readonly
readonly_mix ro -= 1                    # COMPILE ERROR, readonly_mix is readonly
```

Note that oh-lang takes a different approach than C++ when it comes to constant/readonly fields
inside of classes.  In C++, using `const` on a field type bars reassignment of the class instance.
(`non-static const member ‘const t T’, cannot use default assignment operator`.)
In oh-lang, readonly variables are not always deeply constant.  And in the case of readonly class
instance fields, readonly variables are set based on the constructor and shouldn't be modified
afterwards by other methods... except for the constructor if it's called again (i.e., via
`renew_`ing the instance or reassignment).

### automatic deep nesting

We can create deeply nested objects by adding valid identifiers with consecutive `:`.  E.g.,
`[x: y: 3]` is the same as `[x: [y: 3]]`.  Similarly for `()` and `{}`.

## temporarily locking writable variables

You can also make a variable readonly for the remainder of the current block
by using `@lock` before the variable name.  Note that you can modify it one last time
with the `@lock` annotation, if desired.  Also note that the variable may not be deeply constant,
e.g., if lambdas are called which modify it, but you will not be able to explicitly modify it.

```
x; int_ = 4 # defined as writable and reassignable

if some_condition
    @lock x = 7 # locks x after assigning it to the value of 7.
                # For the remainder of this indented block, you can use x but not reassign it.
                # You also can't use writable, i.e., non-const, methods on x.
else
    @lock x # lock x to whatever value it was for this block.
            # You can still use x but not reassign/mutate it.

print_(x)   # will either be 7 (if some_condition was true) or 4 (if !some_condition)
x += 5      # can modify x back in this block; there are no constraints here.
```

## hiding variables

We can hide a variable from the current block by using `@hide` before the variable name.
This doesn't descope the variable, but it does prevent the variable from being used by
new statements/functions.  `@hide` has similar behavior to the `@lock` annotation, in that
you can use the variable one last time with the annotation, if desired.

```
date_string; str_("2023-01-01")

# after this line, `date_string` can't be accessed anymore.
date: date_(@hide date_string)

# note in some circumstances you may also want to include `!` to avoid copying the variable,
# if the underlying class makes use of that same type variable internally, e.g.:
date: date_(@hide date_string!)
# see discussion on `moot` for more information.
```

In fact, hiding variables make it possible to shadow identifiers; i.e.,
for variable renaming.  See the following example:

```
do_something_(date: str_("2023-01-01")):
    date: date_(@hide date!)
```

# functions

Functions are named using `function_case_` identifiers.  The syntax to declare
a function is `function_case_name_(function_arguments...): return_type_`, but if
you are also defining the function the `return_type_` is optional (but generally
recommended for multiline definitions).  Defining the function can occur inline
with `:` or over multiple lines using an indented block.

```
# declaring a function with no arguments that returns a big integer
v_(): int_

# setting/defining/initializing the function:
v_(): int_
    # `return` is optional for the last line in a block.
    # e.g., the following could have been `return 600`.
    600

# inline definition
v_(): 600

# inline, but with explicit type
v_(): int_(600)

# function with X,Y double-precision float arguments that returns nothing
v_(x: dbl_, y: dbl_): null_
    print_("x = ${x}, y = ${y}, atan_(y, x) = ${\\math atan_(x, y)}")
    # Note this could also be defined more concisely using $(),
    # which also prints the expression inside the parentheses with an equal sign and its value,
    # although this will print `x: ..., y: ..., atan: ...`, e.g.:
    # print("$(x, y, \\math atan_(x, y))")

# Note that it is also ok to use parentheses around a function definition,
# but you should use braces `{}`.
excite_(times: int_): str_
{   "hi!" * times
}

# You can define a multi-statement function in one line like this,
# but this is not normally recommended.
oh_(really; dbl_): dbl_ { really *= 2.5, return 50 + really }
```

Note that we disallow the inverted syntax of `function_name_: return_type_(...Args)`
because this looks like declaring a type (e.g., no parentheses on the left hand side)
and the right hand side looks like how we call a function and get an instance (not a type).
See [returning a type](#returning-a-type) for how we'd return a type from a function.

## calling a function

You can call functions with arguments in any order.  Arguments must be specified
with the named identifiers in the function definition.  The only exception is
if the argument is default-named (i.e., it has the same name as the type), then you
don't need to specify its name.  We'll discuss that more in the
[default-name arguments](#default-name-arguments-in-functions) section.

```
# definition:
v_(x: dbl_, y: dbl_): null_

# example calls:
v_(x: 5.4, y: 3)
v_(y: 3, y: 5.4)

# if you already have variables X and Y, you don't need to re-specify their names:
x: 5.4
y: 3
v_(x, y)     # equivalent to `v(x: x, y: y)` but the redundancy is not idiomatic.
v_(y, x)     # equivalent
```

### references

We can create references using [reference objects](#reference-objects) in the following way.
Note that you can use all the same methods on a reference as the original type.

```
my_value; int_(1234567890)
(my_ref; int_) = My_value
# equivalent: `my_ref; (int;) = my_value`
(my_readonly_ref: int_) = my_value
# equivalent: `my_readonly_ref: (int:) = my_value`

# NOTE: `my_value` and the `my_ref` reference need to be writable for this to work.
my_ref = 12345
# my_readonly_ref = 123 # COMPILE ERROR!

# This is true; `my_value` was updated via the reference `my_ref`
my_value == 12345
my_readonly_ref == 12345 # also true.

# There is no need to "dereference" the pointer
print(my_ref * 77)
print(my_readonly_ref * 23)
```

Unlike in C++, there's also an easy way to change the reference to point to
another instance.  This does require a bit more syntax if you are pointing
to a readonly value like `(referent_type:)`, since you'll need to declare it
in a way that lets you modify the reference itself.

```
my_value1: int_(1234)
my_value2: int_(765)
# define `my_ref` as a mutable pointer to an immutable variable:
my_ref; (int:) = my_value1
# that way we can update the pointer like this:
(my_ref) = My_value2
```

Note that by default, references like `(my_ref; int_) = some_reference_()`
will be reassignable, i.e., defined like `my_ref; (int;) = some_reference_()`,
and references like `(my_ref: int_) = some_reference_()` will not be reassignable,
i.e., defined like `my_ref: (int:) = some_reference_()`.  If you want a readonly-
referent reference to be reassignable, use `my_ref; (int:) = ...`.

You can grab a few references at a time using [destructuring](#destructuring)
notation like this:

```
ref3; (str:) = some_ref_()
# this declares+defines `ref1` and `ref2`, and reassigns `ref3`:
(ref1;, ref2:, ref3) = some_function_that_returns_refs_()

# e.g., with function signature:
some_function_that_returns_refs_(): (ref2; int_, ref2: dbl_, ref3; str_)
```

#### reference objects

TODO: we probably need a borrow checker (like Rust):

```
result?; some_nullable_result_()
if result is non_null:
    print_(non_null)
    result = some_other_function_possibly_null_()
    # this could be undefined behavior if `non_null` is a reference to the
    # nonnull part of `result` but `result` became null with `some_other_function_possibly_null_()`
    print_(non_null)
```

Alternatively, we pass around "full references" whenever we can't determine that
borrowing can be done with just a pointer.  Full references include a path from
a safely-borrowed pointer, with checks at each nested value for any additions
that need to be made.  In the above example, we need `non_null` to be a pointer
from `result` that checks if `result` is non-null before any dereferencing.  The
above example can be checked by the compiler, but if `result` was itself a reference
path then we'd need to recheck any dereferences of `non_null`.

In oh-lang, parentheses can be used to define reference objects, both as types
and instances.  As a type, `(x: dbl_, y; int_, z. str_)` differs from the object
type `[x: dbl_, y; int_, z. str_]`.  When instantiated, reference objects with
`;` and `:` fields contain references to variables; objects get their own copies.

Because they contain references, reference object instances cannot outlive the lifetime
of the variables they contain.

```
a_: (x: dbl_, y; int_, z. str_)

# This is OK:
x: 3.0
y; 123
a: (x, y, z. "hello")    # `Z` is passed by value, so it's not a reference.
a y *= 37    # OK

# This is not OK:
return_a_(q: int_): a_
    # x and y are defined locally here, and will be descoped at the
    # end of this function call.
    x: (q dbl_() ?? nan) * 4.567
    y; q * 3
    # ERROR! we can't return x, y as references here.  z is fine.
    (x, y, z. "world")
```

Note that we *can* return reference object instances from functions, but they must be
defined with variables whose lifetimes outlive the input reference object instance.
For example:

```
x: 4.56
return_a_(q; int_): (x: dbl_, y; int_, z. str_)
    q *= 37
    # x has a lifetime that outlives this function.
    # y has the lifetime of the passed-in variable, which exceeds the return type.
    # z is passed by value, so no lifetime concerns.
    (x, y; q, z. "sky")
```

Argument objects are helpful if you want to have arguments that should be
references, but need nesting to be the most clear.  For example:

```
# function declaration
copy_(from: (pixels, rectangle.), to: (pixels;, rectangle.): null_

# function usage
SOURCE_pixels: pixels_() { #( build image )# }
DESTINATION_pixels; pixels_()
size: rectangle_(width: 10, height: 7)

copy_
(   from: 
    (   SOURCE_pixels
        size + vector2_(x: 3, y: 4)
    )
    to:
    (   ;DESTINATION_pixels
        size + Vector2_(x: 9, y: 8)
    )
)
```

We can create deeply nested reference objects by adding valid identifiers with consecutive `:`/`;`/`.`.
E.g., `(x: y: 3)` is the same as `(x: (y: 3))`.  This can be useful for a function signature
like `run_(after: duration_, fn_(): ~t_): t_`.  `duration_` is a built-in type that can be built
out of units of time like `seconds`, `minutes`, `hours`, etc., so we can do something like
`run_(after: seconds: 3, {print_("hello world!")})`, which will automatically pass
`(seconds: 3)` into the `duration_` constructor.  Of course, if you need multiple units of time,
you'd use `run_(after: (seconds: 6, minutes: 1), {print_("hello world!")})` or to be explicit
you'd use `run_(after: duration_(seconds: 6, minutes: 1), {print_("hello world!")})`.


#### reference lifetimes

References are not allowed to escape the block in which their referent is defined.
For example, this is illegal:

```
original_referent: int_ = 3
my_reference: (int:) = original_referent
if some_condition
{   nested_referent: int_ = 5
    # COMPILE ERROR: `nested_referent` doesn't live as long as `my_reference`
    (my_reference) = nested_referent
}
```

However, since function arguments can be references (e.g., if they are defined with
`:` or `;`), references that use these function arguments can escape the function block.

```
fifth_element_(array[int_];): (int;)
    # this is OK because `array` is a mutable reference
    # to an array that already exists outside of this scope.
    # NOTE: this actually returns a pointer to the array with an offset (i.e., 4)
    #       in case the array reallocates, etc.
    (;array[4])

my_array; array_[int_](1, 2, 3, 4, 5, 6)
(fifth;) = fifth_element_(;my_array)
fifth += 100
my_array == [1, 2, 3, 4, 105, 6]    # should be true
```

#### refer function

If you need some special logic before returning a reference, e.g., to create a default,
you can use the `refer_` function with the following signature: `refer_(~r;, fn_(r;): (~t;)`
and similarly for a constant reference (swap `;` with `:` everywhere).  There's also a
key-like interface (e.g., for arrays or lots):

```
# if `at` is passed as a temporary, it should be easily copyable.
refer_(~r;:, at` ~k_, fn_(r;:, k:): (~t;:)): (t;:)`
```

You can also create a reference via getters and setters using the `refer_` function, which
has the following signature: `refer_(~r;, GETTER_fn_(r:): ~t_, SETTER_fn_(r;, t.): null_): (t;)`.
It extends a base reference to `r` to provide a reference to a `t_` instance.
There's also a key-like interface (e.g., for arrays or lots):

```
# if `at` is passed as a temporary, it should be easily copyable.
refer_(~r;:, at` ~k_, GETTER_fn_(r:, k:): ~t_, SETTER_fn_(r;:, k:, t.): null_): (t;)`
```

When calling `refer_`, we want the getters and setters to be known at compile time,
so that we can elide the reference object creation when possible.

```
my_array; [1, 2, 3, 4]

# here we can elide `refer_` here that is inside the method
# `array_[int_];;[index]: (int;)`
my_array[0] = 0     # my_array == [0, 2, 3, 4]

# here we cannot elide `refer_`
(my_reference;) = my_array[2]
my_reference += 3   # my_array == [0, 2, 6, 4]
print(my_reference) # prints `6`
```

### default-name arguments in functions

For functions with one argument (per type) where the variable name doesn't matter,
you can use default-named variables.  For standard ASCII identifiers, the default-name identifier
is just the `variable_case` version of the `type_case_` type (i.e., remove the trailing `_`).

```
# this function declaration is equivalent to `f_(int: int_): int_`:
f_(int:): int_
    int + 5

z: 3
f_(z)                   # ok
f_(4.3 floor_() int_()) # ok
f_(5)                   # ok
f_(int: 7)              # ok but overly verbose
```

If passing functions as an argument where the function name doesn't matter,
there are actually a few options: `_`, `a_`, `an_`, `fn_`, and `do_`.
We recommend `a_` and `an_` for `map`-like operations with a single argument,
choosing `an_` if the argument name starts with a vowel sound (and `a_` otherwise),
and `do_` for multi-argument functions.  We keep `fn_` around
mostly to make it easy for developers new to the language.  Note that if
any functions are defined, including default named functions, no variables
can shadow their `variable_case` form.  And vice versa.

```
# declaring a function that takes a lambda, note the default name.
q_(fn_(): bool_): null_

# defining a function that takes a lambda.
q_(fn_(): bool_): null_
    if fn_()
        print_("function returned true!")
    else
        print_("function returned false!")

q_
(   name_it_what_you_want_(): true
)   # should print "function returned true!"

# or you can create a default-named function yourself:
q_
(   fn_(): bool_
        random_() > 0.5
)   # will print one of the above due to randomness.
# equivalent to `q_(fn_(): random_() > 0.5)` or `q_({random_() > 0.5})`

# defining a lambda usually requires a name, feel free to use a default:
q_(_(): true)
# or you can use this notation, without the name:
q_({true})

# or you can do multiline:
x; bool_
q_
(   fn_():
        x
)
# equivalent to `q_(fn_(): {x})`
# also equivalent to `q_({x})`
```

### the name of a called function in a reference object

Calling a function with one argument being defined by a nested function will use
the nested function's name as the variable name.  E.g., if a function is called
`value_`, then executing `what_is_this_(value_())` will try to call the `what_is_this_(value)`
overload.  If there is no such overload, it will fall back on `what_is_this_(type)` where
`type` is the return value of the `value_()` function.

The only exception is the `::o_()` method, which is the copy method; this passes through
the name of whatever `variable_case` (or `function_case_` truncated to `variable_case`)
that was before it, e.g., `x o_()` is still named `x` and `y_() o_()` is named `y`.
TODO: if we want to allow `o` as a field on objects, we could use `m_` instead of `o_`.
but i think we want both `m` and `o` ways to refer to a class instance anyways so efficiency
isn't a big concern here.  probably could use either, but `o_` will be more idiomatic because
you're creating an*o*ther instance.

```
value_(): int_
    return 1234 + 5

what_is_this_(value: int_): null_
    print_(value)

what_is_this_(value: 10)    # prints 10
what_is_this_(value_())     # prints 1239
```

You can still use `value_()` as an argument for a default-named `int_` argument,
or some other named argument by renaming.

```
takes_default_(int): string_
    string_(int)

takes_default_(value_())    # OK.  we try `value: value_()`
                            # and then the type of `value_()` next

other_function_(not_value: int_): string_
    return "!" * not_value

other_function_(value_())               # ERROR! no overload for `value` or for `int`.
other_function_(not_value: value_())    # OK
```

This works the same for plain-old-data objects, e.g., `[value_()]` corresponds to
`[value: value_()]`.  In case class methods are being called, the class name
and the class instance variable name are ignored, e.g., `[my_class_instance my_function_()]`
is short-hand for `[my_function: my_class_instance my_function_()]`.

### functions as arguments

A function can have a function as an argument, and there are a few different ways to call
it in that case.  This is usually a good use-case for lambda functions, which define
an inline function to pass into the other function.  Because we support
[function overloading](#function-overloads), any externally defined functions need to be
fully specified.  (E.g., this is not allowed: `greet_(int): "hello" + "!" * int, do_greet_(greet_)`.)
This is also because we allow passing in types as function arguments, so anything that is
`function_case_` without a subsequent parenthesized argument list `(args...)` will be considered
`type_case_` instead.
TODO: is there a meaningful distinction here?  since types are functions.  although
functions can return multiple different types, types will return only a single type.
(although they can return a result type for a constructor.)  i bet we could use
the default overload for a function that we pass in as `greet_`.

```
# finds the integer input that produces "hello, world!" from the passed-in function, or -1
# if it can't find it.
detect_(greet_(int): string_): int_
    100 each CHECK_int:
        if greet_(CHECK_int) == "hello, world!"
            return CHECK_int
    return -1

# if your function is named the same as the function argument...
greet_(int): string_
    return "hay"
# you can use it directly, although you still need to specify which overload you're using,
detect_(greet_(int): string_)   # returns -1
# also ok, but a bit verbose:
detect_(greet_(int): greet_(int) string)

# if your function is not named the same, you can do argument renaming;
# internally this does not create a new function:
say_hi_(int): string_
    return "hello, world" + "!" * int
detect_(greet_(int): string_ = say_hi_) # returns 1

# you can also create a function named correctly inline -- the function
# will not be available outside, after this call (it's scoped to the function arguments).
detect_
(   greet_(int): string_
        "hello, world!!!!" substring_(length: int)
)   # returns 13

detect_(greet_(int): {["hi", "hey", hello"][int % 3] + ", world!"}) # returns 2
```

### lambda functions

Lambda functions are good candidates for [functions as arguments](#functions-as-arguments),
since they are very concise ways to define a function.  They utilize an indented block
or set of braces  like `{...function-body...}` with function arguments defined inside using
`$the_argument_name`.  There is no way to specify the type of a lambda function argument,
so the compiler must be able to infer it (e.g., via using the lambda function as an argument,
or by using a default name like `$int` to define an integer).  Some examples:

```
run_asdf_(do_(j: int_, k: str_, l: dbl_): null)_: null_
    print_(do_(j: 5, k: "hay", l: 3.14))

# Note that `$k`, `$j`, and `$l` attach to the same lambda based on looking
# for the first matching `{}`.
run_asdf_({$k * $j + str_($l)})     # prints "hayhayhayhayhay3.14"

# One example with brackets:
my_array: [0.06, 0.5, 4.0, 30.0, 200.0, 1000.0]
# Again, `$k`, `$j`, and `$l` attach to the same lambda.
run_asdf_({$k + str_(my_array[$j] * $l)})   # prints "hay3140"
# The same example with an indent:
run_asdf_
(   $k + str_(my_array[$j] * $l)
)
# this is wrong, this looks like line continuation.
run_asdf_
(       $k + str_(my_array[$j] * $l)
)
```

If you need a lambda function inside a lambda function, use another `$` to escape
one variable into the parent scope, e.g.,

```
# with function signatures
# `run_(fn_(x: any_): any_): any_` and
# `run_nested_(fn_(y: any_): any_): any_`
run_({$x + run_nested_({$y + $$x})})

# or with indents
run_
(   $x + run_nested_
    (   $y + $$x
    )
)
```

But it would probably be more readable to just define the functions normally in this instance.

There is currently no good way to define the name of a lambda function; we may use
`@named(whatever_name_) {$x + $y}`, but it's probably more readable to just define
the function inline as `whatever_name_(x, y): x + y`.

### types as arguments

Generally speaking you can use generic/template programming for this case,
which infers the types based on instances of the type.

```
# generic function taking an instance of `x_` and returning one.
do_something_(~x): x_
    return x * 2

do_something_(123)    # returns 246
do_something_(0.75)   # returns 1.5
```
See [generic/template functions](#generictemplate-functions) for more details
on the syntax.

TODO: i don't like having two ways of doing this.  should we always
assume that arguments in parentheses are values and arguments in brackets are types?
but we do need values in brackets.

However, there are use cases where we might actually want to pass in
the type of something.  We can use `of_` as a type name to get default naming.
```
# `whatever_constraints_` can be something like `number_`,
# or you can elide it if you want no constraints.
do_something_(of_: whatever_constraints_): of_
    return of_(123)

print_(do_something_(dbl_)) # returns 123.0
print_(do_something_(u8_))  # returns u8(123)
```

Or we could do this as a a generic type, like this:
```
do_something_(~x_): x_
    return x_(123)

print_(do_something_(dbl_)) # returns 123.0
print_(do_something_(u8_))  # returns u8(123)
```

### returning a type

We use a different syntax for functions that return types; namely `()` becomes `[]`,
e.g., `type_fn_[args...]: the_type_`.  This is because we do not need
to support functions that return instances *or* constructors, and it becomes clearer
that we're dealing with a type if we use `[]`.  The alternative would be to use
`fn_(int): int` to return an `int_` instance and `fn_(int): int_` to return the
`int_` constructor, but again we never need to mix and match.  The bracket syntax is
related to [template classes](#generictemplate-classes) and
[overloading generic types](#overloading-generic-types).
TODO: maybe this is ok.  it might even be less confusing than using different
brackets for types.  but i do really like `array_[int_]` rather than `array_(int_)`;
so it makes generics easier to think about, especially when combined with functions.
HOWEVER, i think it's probably best to keep it the way it is unless we want to force
all functions to be multiline (either by `{}` or by indenting), because
`fn_(): null` would look like we're already returning a value.

TODO: this should probably be disallowed.  the compiler needs to be able
to reason about the type in a `x_[...]` function.  come up with deterministic examples.

```
# it's preferable to return a more specific value here, like
# `one_of_[int_, dbl_, string_]`, but `any_` works as well.
random_class_[]: any_
    if random_(dbl_) < 0.5
        int_
    elif random_(dbl_) < 0.5
        dbl_
    else
        string_

x: random_class_[] = 123
match x
    int:
        print("x is an int_: ${int}")
    dbl:
        print("x is a dbl_: ${dbl}")
    string:
        print("x is a string_: ${string}")
```

We can also pass in named types as arguments.  Here is an example
where we also return a type constructor.  Named types are just
`type_case_` on both left and right sides (e.g., `class_name_: t_`).

```
random_class_[~x_, named_new_: ~y_]: one_of_[x_, y_]
    if random_(dbl_) < 0.5 {x_} else {named_new_}

# will print `int_` or `dbl_` with 50-50 probability
print_(random_class_[int_, named_new_: dbl_])
```

To return multiple types, you can use the [type tuple syntax](#type-tuples).

### unique argument names

Arguments must have unique names; e.g., you must not declare a function with two arguments
that have the same name.  This is because we wouldn't be able to distinguish between
the two arguments inside the function body.

```
# COMPILER ERROR.  duplicate identifiers
my_fun_(x: int_, x: dbl_): one_of_[int_, dbl_]
```

However, there are times where it is useful for a function to have two arguments with the same
name, and that's for default-named arguments in a function where (1) *order doesn't matter*,
or (2) order does matter but in an established convention, like two sides of a binary operand.
An example of (1) is in a function like `max_`:

```
@order_independent
max_(int, OTHER_int): int_
    if int >= OTHER_int
        int
    else
        OTHER_int

max_(5, 3) == max_(3, 5)
```

The compiler is not smart enough to know whether order matters or not, so we need to annotate
the function with `@order_independent` -- otherwise it's a compiler error -- and we need to use
namespaces (e.g., `OTHER_int`) in order to distinguish between the two variables
inside the function block.  When calling `max_`, we don't need to use those namespaces, and
can't (since they're invisible to the outside world).

There is one place where it is not obvious that two arguments might have the same name, and
that is in method definitions.  Take for example the vector dot product:

```
vector2_: [x; dbl_, y; dbl_]
{   ;;renew_(m x. dbl_, m y. dbl_): {}

    # this is required to create vectors like this: `vector2_(1.0, 2.0)`
    # since we are explicit about `FIRST_` and `SECOND_` we don't need the
    # `@order_dependent` annotation.
    m(FIRST_dbl., SECOND_dbl.): m_
        m(x. FIRST_dbl, y. SECOND_dbl)

    @order_independent
    # can also use `o` instead of `vector2` as the argument name for an `o`ther
    # of the same type as `m`, and then you can omit the `@order_independent`
    # (or `@order_dependent`) annotation.
    ::dot_(vector2): dbl_
        m x * vector2 x + m y * vector2 y
}
vector2: vector2_(1, 2)
other_vector2: vector2_(3, -4)
print_(vector2 dot_(other_vector2))     # prints -5
print_(dot_(vector2, other_vector2))    # equivalent, prints -5
```

The method `::dot_(vector2): dbl_` has a function signature `dot_(m, vector2): dbl_`,
where `m` is an instance of `vector2_`, so ultimately this function creates a global
function with the function signature `dot_(vector2, vector2): dbl`.  Therefore this function
*must* be annotated as `@order_independent` or `@order_dependent`, to avoid confusion.
Otherwise it is a compiler error.  Alternatively to using annotations, you can use
namespaces like `FIRST_` and `SECOND_`.  `m` is assumed to be `FIRST_vector2` in the
above example, but if you use `o` it will be assumed to be `SECOND_vector2`.

As mentioned earlier, we can have order dependence in certain established cases, but these
should be avoided in oh-lang as much as possible, where we prefer unique names.
One example is the cross product of two vectors, where order matters but the
names of the vectors don't.  (The relationship between the two orders is also
somewhat trivial, `a cross_(b) == -b cross_(a)`, and this simplicity should be aspired to.)

```
vector3_: [x; dbl_, y; dbl_, z; dbl_]
{   ;;renew_(m x. dbl_, m y. dbl_, m z. dbl_): {}

    # defined in the class body, we do it like this:
    ::cross_(o): m_
    (   x. m y * o z - m z * o y
        y. m z * o x - m x * o z
        z. m x * o y - m y * o x
    )
}

# defined outside the class body, we do it like this:
# NOTE: both definitions are *not* required, only one.
cross_(FIRST_vector3, SECOND_vector3): vector3_
(   x: FIRST_vector3 y * SECOND_vector3 z - FIRST_vector3 z * SECOND_vector3 y
    y: FIRST_vector3 z * SECOND_vector3 x - FIRST_vector3 x * SECOND_vector3 z
    z: FIRST_vector3 x * SECOND_vector3 y - FIRST_vector3 y * SECOND_vector3 x
)
```

One final note is that operations like `+` should be order independent, whereas `+=` should be
order dependent, since the method would look like this: `;;+=(vector2)`, which is
equivalent to `+=(m;, vector2)`, where the first argument is writable.  These
two arguments can be distinguished because of the writeability, so it's not necessary
to annotate these.  But we still recommend `o` in these cases for code copy-pastability.

## function overloads

Functions can do different things based on which arguments are passed in.
For one function to be an overload of another function, it must be defined in the same file,
and it must have different argument names or return values.  You can also have different
argument modifiers (i.e., `;` and `:` are different overloads, as are nullable types, `?`).

```
greet_(string): null_
    print_("Hello, ${string}!")

greet_(say: string_, to: string_): null_
    print_("${say}, ${to}!")

greet_(say: string_, to: string_, times: int_): null_
    times each _int:
        greet_(say, to)

# so you call this in different ways:
greet_("World")
greet_(to: "you", say: "Hi")
greet_(times: 5, say: "Hey", to: "Sam")

# note this is a different overload, since it must be called with `say;`
greet_(say; string_): null_
    say += " wow"
    print_("${say}, world...")

my_say; "hello"
greet_(say; my_say) # prints "hello wow, world..."
print_(my_say)      # prints "hello wow" since `my_say` was modified
```

Note also, overloads must be distinguishable based on argument **names**, not types.
Name modifiers (i.e., `;`, `:`, and `?`) also count as different overloads.
Note that default-names for different types count as different names (e.g., `fn_(dbl)`
and `fn_(int)` are different overloads).

```
fibonacci_(times: int_): int_
    previous; 1
    current; 0
    times each _int:
        next_previous: current
        current += previous
        previous = next_previous
    current

fibonacci_(times: dbl_): int_
    golden_ratio: dbl_ = (1.0 + \\math sqrt_(5)) * 0.5
    other_ratio: dbl_ = (1.0 - \\math sqrt_(5)) * 0.5
    round_((golden_ratio^times - other_ratio^times) / \\math sqrt_(5))
# COMPILE ERROR: function overloads of `fibonacci_` must have unique argument names,
#                not argument types.

# NOTE: if the second function returned a `dbl_`, then we actually could distinguish between
# the two overloads.  This is because default names for each return would be `int` and `dbl`,
# respectively, and that would be enough to distinguish the two functions.  The first overload
# would still be default in the case of a non-matching name (e.g., `result: fibonnaci_(times: 3)`),
# but we could determine `int: fibonacci_(times: 3)` to definitely be the first overload and
# `dbl: fibonacci_(times: 7.3)` to be the second overload.
```

There is the matter of how to determine which overload to call.  We consider
only overloads that possess all the specified input argument names.  If there are some
unknown names in the call, we'll check for matching types.  E.g., an unnamed `4.5`, 
as in `fn_(4.5)` or `fn_(X, 4.5)`, will be checked for a default-named `dbl` since `4.5`
is of type `dbl_`.  Similarly, if a named variable, e.g., `q` doesn't match in `fn_(q)`
or `fn_(x, q)`, we'll check if there is an overload of `fn_` with a default-named type
of `q`; e.g., if `q` is of type `animal_`, then we'll look for the `fn_(animal):` 
or `fn(x, animal)` overload.

NOTE: we cannot match a function overload that has more arguments than we supplied in
a function call.  If we want to allow missing arguments in the function call, the declaration
should be explicit about that; e.g., `fn_(x?: int): ...` or `fn_(x: 0): ...`.
Similarly, we cannot match an overload that has fewer arguments than we supplied in the call.

Output arguments are similar, and are also matched by name.  This is pretty obvious with
something like `x: calling_(INPUT_args...)`, which will first look for an `x` output name
to match, such as `calling_(INPUT_args...): [x: whatever_type_]`.  If there is no `x` output
name, then the first non-null, default-named output overload will be used.  E.g., if
`calling_(INPUT_args...): dbl_` was defined before `calling_(INPUT_args...): str_`, then `dbl_`
will win.  For an output variable with an explicit field request, e.g., `x: calling_(INPUT_args...) dbl`,
this will look for an overload with output name `dbl` first.  If there is no output named `x`,
then `x: dbl_ = calling_(INPUT_args...)` will also work to get the `dbl` field.  For function calls like
`x: dbl_(calling_(INPUT_args...))`, we will lose all `x` output name information because
`dbl_(...)` will hide the `x` name.  In this case, we'll use the default overload and attempt
to convert it to a `dbl_`.
TODO: we may want to do it as a compile error because it's a conversion not a type specification.

One downside of overloads is that we must specify the whole function
[when passing it in as an argument](#functions-as-arguments).  But we also need to specify
the whole function because we want to be able to distinguish `type_case_` from `function_case_`
(where some parenthesized arguments `(args...)` follow).

TODO: default output arguments.  e.g., `fn_(x: int_): [y: 3]`

### nullable input arguments

When you call a function with an argument that is null, conceptually we choose the
overload that doesn't include that argument.  In other words, a null argument is
the same as a missing argument when it comes to function overloads.  Thus, we are
not free to create overloads that attempt to distinguish between all of these cases:
(1) missing argument, (2) present argument, (3) nullable argument, and (4) default argument.
Only functions for Cases (1) and (2) can be simultaneously defined; any other combination
results in a compile-time error.  Cases (3) and (4) can each be thought of as defining two function
overloads, one for the case of the missing argument and one for the case of the present argument.

Defining conflicting overloads, of course, is undesirable.  Here are some example overloads;
again, only Cases (1) and (2) are compatible and can be defined together.

```
# missing argument (case 1):
some_function_(): dbl_
    987.6

# present argument (case 2):
some_function_(y: int_): dbl_
    2.3 * dbl_(y)

# nullable argument (case 3):
some_function_(y?: int_): dbl_
    if y != null {1.77} else {y + 2.71}

# default argument (case 4):
# `y: 3` is short for `y: @type_of(3) = 3`, and `@type_of(3)` is `int_`.
some_function_(y: 3): dbl_
    dbl_(y)
```

Note that writable arguments `;` are distinct overloads, which indicate either mutating
the external variable, taking it, or swapping it with some other value, depending on
the behavior of the function.  Temporaries are also allowed, so defaults can be defined
for writable arguments.

What are some of the practical outcomes of these overloads?  Suppose
we define present and missing argument overloads in the following way:

```
overloaded_(): dbl_
    123.4
overloaded_(y: int_): string_
    "hi ${y}"
```

The behavior that we get when we call `overloaded_` will depend on whether we
pass in a `y` or not.  But if we pass in a null `y`, then we also will end up
calling the overload that defined the missing argument case.  I.e.:

```
y?; int = ... # `y` is maybe null, maybe non-null

# the following calls `overloaded_()` if `y` is null, otherwise `overloaded_(y)`:
z: overloaded_(y?) # also OK, but not idiomatic: `z: overloaded_(y?: y)`
# `z` has type `one_of_[dbl_, string_]` due to the different return types of the overloads.
```

The reason behind this behavior is that in oh-lang, an argument list is conceptually an object
with various fields, since an argument has a name (the field name) as well as a value (the field value).
An argument list with a field that is `null` should not be distinguishable from an argument list that
does not have the field, since `null` is the absence of a value.

Note that when calling a function with a nullable variable/expression, we need to
indicate that the field is nullable if the expression itself is null (or nullable). 
Just like when we define nullable variables, we use `?:` or `?;`, we need to use
`?:` or `?;` (or some equivalent) when passing a nullable field.  For example:

```
some_function_(x?: int_): int_
    x ?? 1000

# when argument is not null:
some_function_(x: 100)      # OK, expression for `x` is definitely not null
some_function_(x?: 100)     # ERROR! expression for `x` is definitely not null

# when argument is an existing variable:
x?; null
print_(some_function_(x?))  # can do `x?: x`, but that's not idiomatic.

# when argument is a new nullable expression:
some_function_(x?: some_nullish_function_())    # REQUIRED since `some_nullish_function_` can return null
some_function_(x: some_nullish_function_())     # ERROR! `some_nullish_function_` is nullable, need `X?:`.

# where some_nullish_function might look like this:
some_nullish_function_()?: int_
    if some_condition { null } else { 100 }
```

Note however that if a value is definitely null, then we don't allow passing
it in as a nullable argument.

```
some_null_function_(): null_
    print_("go team")
 
# COMPILE ERROR, `x` is always null.
# cleaner: `some_null_function_(), some_function_()`:
some_function_(x?: some_null_function_())
```

We also want to make it easy to chain function calls with variables that might be null,
where we actually don't want to call an overload of the function if the argument is null.

```
# in other languages, you might check for null before calling a function on a value.
# this is also valid oh-lang but it's not idiomatic:
x?: if y != null { overloaded_(y) } else { null }

# instead, you should use the more idiomatic oh-lang version.
# putting a ? *before* the argument name will check that argument;
# if it is null, the function will not be called and null will be returned instead.
# if you needed to name this argument, it would be `arg_name: ?y`:
x?: overloaded_(?y)

# either way, `x` has type `one_of_[string_, null_]`.
```

You can use prefix `?` with multiple arguments; if any argument with prefix `?` is null,
then the function will not be called.

This can also be used with the `return` function to only return if the value is not null.

```
do_something_(x?: int_): int_
    y?: ?x * 3    # `y` is null or `x*3` if `X` is not null.
    return ?y       # only returns if `y` is not null
    #( do some other stuff )#
    ...
    return 3
```

### nullable output arguments

We also support function overloads for *outputs* that are nullable.  Just like with overloads
for nullable input arguments, there are some restrictions on defining overloads with (1) a
missing output, (2) a present output, and (3) a nullable output.  The restriction is a bit
different here, in that we cannot define (1) and (3) simultaneously for nullable outputs.
This enables us to distinguish between, e.g., `x?: my_overload_(y)` and `x: my_overload_(y)`,
which defines a nullable `x` or a non-null `x`.

TODO: discussion on `fn_(): [x?: int_]` differences from `fn_()?: [x: int_]`.

```
# case 1, missing output (not compatible with case 3):
my_overload_(y: str_): null_
    print_(y)

# case 2, present output:
my_overload_(y: str_): [x: int_]
    [x: int_(y) ?? panic_("should be an integer")]

# case 3, nullable output (not compatible with case 1):
my_overload_(y: str_): [x?: int_]
    # this is essentially an implementation of `x?: int_(y), return [x]`
    what int_(y)
        ok: $[x: ok]
        er: $[]

[x]: my_overload_(y: "1234")    # calls (2) if it's defined, otherwise it's a compiler error.
[x?]: my_overload_(y: "abc")    # calls (1) or (3) if one is defined, otherwise it's a compiler error.
```

TODO: can we make `assert_` shorter?  Rust is nice with `?`, but we use that for
nullable stuff.  we should consider being nice.  Maybe `?!`.
Note that if only Case 3 is defined, we can use `assert_`s to ensure that the return
value is not null, e.g., `[x]: my_overload_() assert_()`.  This will throw a run-time
error if the return value for `x` is null.  Note that this syntax is invalid if Case 2
is defined, since there is no need to assert a non-null return value in that case.
This will also work for an overload which returns a result `hm`.

```
# normal call for case 3, defines an `x` which may be null:
[x?]: my_overload_(y: "123")

# special call for case 3; if `x` is null, this will return a run-time error,
# otherwise will define a non-null `x`:
[x]: my_overload_(y: "123") assert_()

# make a default for case 3, in case X comes back as null from the function
[x: -1] = my_overload_(y: "123")
```

If there are multiple return arguments, i.e., via an output type data class,
e.g., `[x: dbl_, y: stra_]`, then we support [destructuring](#destructuring)
to figure out which overload should be used.  E.g., `[x, y]: my_overload_()` will
look for an overload with outputs named `x` and `y`.  Due to assumptions with
[single field objects](#single-field-objects) (SFO), `x: my_overload_()` is
equivalent to `[x]: my_overload_()`.  You can also explicitly request the
return type, e.g., `SOME_int: my_overload_()` or `r: my_overload_() dbl`, which
will look for an overload with an `int_` or `dbl_` return type, respectively.

When matching outputs, the fields count as additional arguments, which must
be matched.  If you want to call an overload with multiple output arguments,
but you don't need one of the outputs, you can use the `@hide` annotation to
ensure it's not used afterwards.  E.g., `[@hide x, y]: my_overload_()`.
You can also just not include it, e.g., `y: my_overload_()`, which is
preferred, in case the function has an optimization which doesn't need to
calculate `x`.

We also allow
[calling functions with any dynamically generated arguments](#dynamically-determining-arguments-for-a-function),
so that means being able to resolve the overload at run-time.

### pass-by-reference or pass-by-value

Functions can be defined with arguments that are passed-by-value using `.`, e.g., via
`temporary_value. type_of_the_temporary_`.  This argument type can be called with
temporaries, e.g., `fn_(arg_name. "my temp string")`, or with easily-copyable types
like `dbl_` or `i32_` like `my_i32: i32_ = 5, fn_(my_arg. my_i32)`, or with larger-allocation
types like `int_` or `str_` with an explicit copy or move: e.g., `my_str: "asdf..."`
with `fn_(tmp_arg. my_str o_())` or `fn_(tmp_arg. my_str!)`, respectively.

The temporary argument, if modified inside the function block, will have no effect on the
things outside the function block.  Inside the function block, pass-by-value
arguments are mutable, and can be reassigned or modified as desired.
Similar to Rust, variables that can be easily copied implement a `::o_(): m_`
method, while variables that may require large allocations should only implement
`;;renew_(o:): hm[ok_: null_, er_: ...]` or `::o_(): hm_[ok_: m_, er_: ...]`
which indicates that an error (like out-of-memory) can occur when trying to copy.
A default definition for these copy constructors are created for most oh-lang classes.

Functions can also be defined with writable or readonly reference arguments, e.g., via
`mutable_argument; type_of_the_writeable_` and `readonly_argument: type_of_the_readonly_`
in the arguments list, which are passed by reference.  This choice has three important
effects: (1) readonly variables may not be deeply constant (see section on
[passing by reference gotchas](#passing-by-reference-gotchas)), (2) you can modify
writable argument variables inside the function definition, and (3) any
modifications to a writable argument variable inside the function block persist
in the outer scope.

When passing by reference for `:` and `;` variables, we cannot automatically 
cast to the correct type.  Two exceptions: (1) if the variable is a temporary
we can cast like this, e.g., `my_value: 123` can be used for a `my_value: u8_` argument,
and (2) child types are allowed to be passed by reference when the function asks
for a parent type.

Return types are never inferred as references, so one secondary difference between
`fn_(int.): ++int` and `fn_(int;): ++int` is that a copy/temporary is required
before calling the former and a copy is made for the return type in the latter.
The primary difference is that the latter will modify the passed-in variable in
the outer scope.  To avoid dangling references, any calls of `fn_(int;)` with a
temporary will actually create a hidden `int` before the function call.  E.g.,
`fn_(int; 12345)` will essentially become `uniquely_named_int; 12345` then
`fn_(int; @hide uniquely_named_int)`, so that `uniquely_named_int` is hidden from the
rest of the block.  See also [lifetimes and closures](#lifetimes-and-closures).
To avoid a copy entirely, you'd need to explicitly annotate the return type;
e.g., you can use `fn_(int;): (int;) {++int}` for the above example, i.e.,
using [reference objects](#reference-objects) for the return value.

In C++ terms, arguments declared as `.` are passed as temporaries (`t &&`),
arguments declared as `:` are passed as constant reference (`const t &)`,
and arguments declared as `;` are passed as reference (`t &`).
Overloads can be defined for all three, since it is clear which is desired
based on the caller using `.`, `:`, or `;`.  Some examples:

```
# this function passes by value and won't modify the external variable
check_(arg123. string_): string_
    arg123 += "-tmp"    # OK since `arg123` is defined as writable, implicit in `.`
    arg123

# this function passes by reference and will modify the external variable
check_(arg123; string_): string_
    arg123 += "-writable"   # OK since `arg123` is defined as writable via `;`.
    arg123

# this function passes by constant reference and won't allow modifications
check_(arg123: string_): string_
    arg123 + "-readonly"

my_value; string_ = "great"
check_(arg123. my_value o_())   # returns "great-tmp".  needs `o_` (copy) since
                                # `.` requires a temporary.
print_(my_value)            # prints "great"
check_(arg123: my_value)    # returns "great-readonly"
print_(my_value)            # prints "great"
check_(arg123; my_value)    # returns "great-writable"
print_(my_value)            # prints "great-writable"
```

Note that if you try to call a function with a readonly reference argument,
but there is no overload defined for it, this will be an error.  Similarly
for writable-reference or temporary-variable arguments.

```
only_readonly_(a: int_): str_
    str_(a) * a

my_a; 10
only_readonly_(a; my_a)         # COMPILE ERROR, no writable overload
only_readonly_(a. int_(my_a))   # COMPILE ERROR, no temporary overload

print_(only_readonly_(a: 3))    # OK, prints "333"
print_(only_readonly_(a: my_a)) # OK, prints "10101010101010101010"

only_mutable_(b; int_): str_
    result: str_(b) * b
    b /= 2
    result

my_b; 10
only_mutable_(b: my_b)          # COMPILE ERROR, no readonly overload
only_mutable_(b. int_(my_b))    # COMPILE ERROR, no temporary overload

print_(only_mutable_(b; my_b))  # OK, prints "10101010101010101010"
print_(only_mutable_(b; my_b))  # OK, prints "55555"

only_temporary_(c. int_): str_
    result; ""
    while c != 0
        result append_(str_(c % 3))
        c /= 3
    result reverse_()
    result

my_c; 5
only_temporary_(c: my_c)        # COMPILE ERROR, no readonly overload
only_temporary_(c; my_c)        # COMPILE ERROR, no temporary overload

print_(only_temporary_(c. 3))       # OK, prints "10"
print_(only_temporary_(c. my_c!))   # OK, prints "12"
```

Note there is an important distinction between variables defined as writable inside a block
versus inside a function argument list.  Mutable block variables are never reference types.
E.g., `b; a` is always a copy of `a`, so `b` is never a reference to the variable at `a`.
For a full example:

```
reference_this_(a; int_): int_
    b; a    # `b` is a mutable copy of `a`.
            # if you want a reference, use `(b;) = a` or `(b); a`.
    a *= 2
    b *= 3
    b

my_a; 10
print_(reference_this_(a; my_a))    # prints 30, not 60.
print_(my_a)                        # `my_a` is now 20, not 60.
```

You are allowed to have default parameters for reference arguments, and a suitable
block-scoped (but hidden) variable will be created for each function call so that
a reference type is allowed.

```
fn_(b; int_(3)): int_
    b += 3
    b

# This definition would have the same return value as the previous function:
fn_(b?; int_): int_
    if b is non_null;
        non_null += 3
        non_null    # note that this will make a copy.
    else
        6

# and can be called with or without an argument:
print_(fn_())           # returns 6
my_b; 10
print_(fn_(b; my_b))    # returns 13
print_(my_b)            # `my_b` is now 13 as well.
print_(fn_(b; 17))      # `my_b` is unchanged, prints 20
```

Note that a mooted variable will automatically be considered a temporary argument
unless otherwise specified.

```
over_(load. int_): str_
    str_(++load)

load; 100
print_(over_(load!))    # calls `over_(load.)` with a temporary, prints 101
print_(load)            # `load = 0` because it was mooted
```

The implementation in C++ might look something like this:

```
void fn_(string &String);        // reference overload for `fn_(str;)`
void fn_(string &&String);       // temporary overload for `fn_(str.)`
void fn_(const string &String);  // constant reference for `fn_(str:)`
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
use `respectively_[a, b, c]` to get type `a`  for `:`, `b` for `;`, and `c` for `.`.
Similarly for the const template `:;`, `respectively_[a, b]` will give `a` for `:` and
`b` when `;`.

```
my_class_[of_]: [x; of_]
{   ;;take_(of.):
        m x = of!
    ;;take_(of:):
        m x = of

    # maybe something like this?
    ;;take_(of;:):
        m x = @moot_or_copy(of)
        # `@moot_or_copy(z)` can expand to `@if @readonly(z) {z o_()} @else {z!}`
}
```

Alternatively, we can rely on some boilerplate that the language will add for us, e.g.,

```
my_class_[of_]: [x; of_]
{   # these are added automatically by the compiler since `x; of_` is defined.
    ;;x_(of;): { m x<->of }
    ;;x_(of:): { m x = of }
    ;;x_(of.): { m x = of! }

    # so `take_` would become:
    ;;take_(of:;.):
        m x_(of)
}
```

### passing by reference gotchas

For example, this function call passes `array[3]` by reference, even if `array[3]` is a primitive.

```
array[int_]; [0, 1, 2, 3, 4]
my_function_(array[3];) # passed as writable reference
my_function_(array[3]:) # passed as readonly reference
my_function_(array[3])  # passed as writable reference since `array` is mutable.
```

You can switch to passing by value by using `.` or making an explicit copy:

```
array[int_]; [0, 1, 2, 3, 4]
my_function_(int_(array[3]))    # passed by value (e.g., `my_function_(int.)`):
```

Normally this distinction of passing by reference or value does not matter except
for optimization considerations.  The abnormal situations where it does matter in
a more strange way is when the function modifies the outside variable
in a self-referential way.  While this is allowed, it's not recommended!  Here is
an example with an array:

```
array; [0, 1, 2, 3, 4]
saw_off_branch_(int;): null_
    array erase_(int)
    int *= 10

saw_off_branch_(array[3];)
print_(array)   # prints [0, 1, 2, 40]
# walking through the code:
# saw_off_branch_(array[3];):
#     array erase_(array[3])    # `array erase_(3)` --> `array` becomes [0, 1, 2, 4]
#     array[3] *= 10            # [reference to 4] *= 10  --> 40
```

Note that references to elements in a container are internally pointers to the container plus the ID/offset,
so that we don't delete the value at `array[3]` and thus invalidate the reference `array[3];` above.
Containers of containers (and further nesting) require ID arrays for the pointers.
E.g., `my_lot["Ginger"][1]["soup"]` would, roughly speaking, be a struct which contains `&my_lot`,
plus the tuple `("Ginger", 1, "soup")`.  We'll call this a "deep reference", but in cases
where we can determine there is no self-referential logic occurring, we'll be able to pass in a pointer
directly to the element.

Here is an example with a lot.  Note that the argument is readonly, but that doesn't mean
the argument doesn't change, especially when we're doing self-referential logic like this.

```
animals; ["hello": cat_(), "world": snake_(name: "Woodsy")]

do_something_(animal:): string_
    result; animal name
    animals["world"] = cat_()       # overwrites `snake_` with a `cat_`
    result += " ${animal speak_()}"
    result

print_(do_something_(animals["world"])) # returns "Woodsy hisss!" (snake name + cat speak)
```

Here is an example without a container, which is still surprising, because
the argument appears to be readonly.  Again, it's not recommended to write your code like this;
but these are edge cases that might pop up in a complex code base.  Because this will be painful
to reason about, we'll probably want to detect this (if possible) and give an error message.

```
my_int; 100
not_actually_constant_(int:): null_
    print_("int before ${int}")
    my_int += int
    print_("int middle ${int}")
    my_int += int
    print_("int after ${int}")
    # int += 5  # this would be a compiler error since `int` is readonly in this scope.

not_actually_constant_(my_int) # prints "int before 100", then "int middle 200", then "int after 400"
```

Because of this, one should be careful about assuming that a readonly argument is deeply constant;
it may only be not-writable from your scope's reference to the variable.

In cases where we know the function won't do self-referential logic,
we can try to optimize and pass by value automatically.  However, we
do want to support closures like `next_generator_(int; int_): do_(): ++int`,
which returns a function which increments the passed-in, referenced integer,
so we can never pass a temporary argument (e.g., `arg. str_`) into `next_generator_`.

### destructuring

If the return type from a function has multiple fields, we can grab them
using the notation `[field1:, field2;, field3] = do_stuff_()`, where `do_stuff_` has
a function signature of `fn_(): [field1: field1_, field2: field2_, field3: field3_, ...]`,
and `...` are optional ignored return fields.  In the example, we're declaring
`field1` as readonly, `field2` as writable, and `field3` is an existing variable
that we're updating (which should be writeable), but any combination of `;`, `:`,
and `=` are possible.  The standard case, however, is to declare (or reassign) all
variables at the same time, which can be done with `[field1, field2]: do_stuff_()`
(`[field1, field2]; do_stuff_()`) for readonly (writeable) declaration + assignment,
or `[field1, field2] = do_stuff_()` for reassignment.

If the returned fields are references (and we don't want to copy them into local variables),
we can use parentheses in an analogous way:  `(ref1:, ref2;, not_a_ref.) = do_stuff_()`
to define `ref1` as a readonly reference, `ref2` as a writeable reference, and
`not_a_ref` as a unique, writable instance.  Or if we want to do all fields the same,
`(ref1, ref2): do_stuff_()` works to define `ref1` and `ref2` as readonly references, or
`(ref1, ref2); do_stuff_()` works to define them as writable references, and
`(ref1, ref2). do_stuff_()` works to define them as unique, writable instances.
Notice that the only distinction between destructuring references and defining a function
is that the function requires a `function_case_` identifier before the parentheses
(e.g., `fn_(): ...` or `do_stuff_(ref1, ref2); ...`).
TODO: let's support the earlier syntax first `(ref1:, ref2;, not_a_ref.) = ...`,
and wait to add `(ref1, ref2): ...` support until later.  Same for `[...] = ...`.

You can also use destructuring to specify return types explicitly.
The notation is `[field1: type1_, field2; type2_] = do_stuff_()`.  This can be used
to grab even a single field and explicitly type it, e.g., `[x: str_] = whatever_()`,
although via [SFO](#single-field-objects) this is the same as `x: str_ = whatever_()`
or `x: whatever_() str`.

This notation is a bit more flexible than JavaScript, since we're
allowed to reassign existing variables while destructuring.  In JavaScript,
`/*js*/ const {field1, field2} = doStuff();` declares and defines the fields `field1` and `field2`,
but `/*js??*/ {field1, field2} = doStuff();`, i.e., reassignment in oh-lang, is an error in JS.

Some worked examples follow, including field renaming.

```
fraction_(in: string_, io; dbl_): [round_down: int_, round_up: int_]
    print_(in)
    round_down: io round_(down)
    round_up: io round_(up)
    io -= round_down
    [round_down, round_up]

# destructuring
io; 1.234
[round_down]: fraction_(in: "hello", io;)

# === calling the function with variable renaming ===
greeting: "hello!"
input_output; 1.234      # note `;` so it's writable.
# just like when we define an argument for a function,
# the newly scoped variable goes on the left,
# so too for destructuring return arguments.
# this one uses the default type of `round_down`:
[integer_part; round_down_] = fraction_(in: greeting, io; input_output)

# here's an example without destructuring.
io; 1.234
result: fraction_(in: "hello", io;)
# `result` is an object with these fields:
print_(result round_down, result round_up)
```

TODO:
Note that we're not allowed to cast... or are we?  we want to be able to easily convert
an iterator into a list, for example.

```
countdown_(count): all_of_[iterator_[count_], m_: [count]]
{   ::next_()?: count_
        if m count > 0
            --m count
        else
            null
}

my_array: array_[count_] = countdown_(5)
```

Note, you can also have nullable output arguments.  These will be discussed
more in the function overload section, but here are some examples.

```
# standard definition:
wow_(lives: int_)?: cat_
    if lives == 9
        cat_()
    else
        null
```

For nested object return types, there is some syntactic sugar for dealing with them.

```
nest_(x: int_, y: str_): [w: [z: [a: int_], b: str_, c: str_]]
    [w: [z: [a: x], b: y, c: y * x]]

# defines `a`, `b`, and `c` in the outside scope:
# TODO: i'm not sure i like this syntax.  we should define the variable on the left.
# maybe `[a: w z a, b: w b, c: w c] = nest_(x: 5, y: "hi")`??
[w: z: a, w: b, w: c] = nest_(x: 5, y: "hi")
print_(a)    # 5
print_(b)    # "hi"
```

### single field objects

Single field objects (SFOs) are used to make it more concise to return a
type into a variable with a given name; the output variable name can help
determine which overload will be called.  Consider the following overloads.

```
patterns_(): [chaos: f32_, order: i32_]
patterns_(): i32_
# overload when we don't need to calculate `order`:
patterns_(): chaos: f32_    # equivalent to `patterns_(): [chaos: f32_]`

i32: patterns_()            # calls `patterns_(): i32_` overload
my_value: patterns_() i32   # same, but defining `my_value` via the `i32_` return value.
NAMESPACE_i32: patterns_()  # same, defining `NAMESPACE_i32` via the `i32_`.
[q: i32_] = patterns_()     # same, defining `q` via the `i32`.

f32: patterns_()            # COMPILE ERROR: no overload for `patterns_(): f32_`

[chaos]: patterns_()        # calls `patterns_(): [chaos: f32_]` overload via destructuring.
chaos: patterns_()          # same, via SFO concision.
my_value: patterns_() chaos # same, but with renaming `chaos` to `my_value`. 
[wow; chaos_] = patterns_() # same, but with renaming `chaos` to `wow`.

result: patterns_()         # calls `patterns_(): [chaos: f32_, order: i32_]`
                            # because it is the default (first defined).
[chaos, order]: patterns_() # same overload, but because of destructuring.
[order]: patterns_()        # same, but will silently drop the `chaos` return value.
order: patterns_()          # more concise form of `[order]: patterns_()`.
my_value: patterns_() order # same, but with renaming `order` to `my_value`.
[yo; order_] = patterns_()  # same, with renaming `order` to `yo`.
```

The effect of SFO is to make it possible to elide `[]` when asking for a single
named output.  The danger is that your overload may change based on your return
variable name; but this is usually desired.

IMPLEMENTATION NOTE: `x: ... assert_()` will require inferring through
the `[x: x_]` return value through the result `hm_[ok_: [x: x_], ...]`
via `assert_()`.  This may be difficult for more complicated expressions.

SFO effectively makes any `x_` return type into a `[x: x_]` object.  This means
that overloads like `patterns_(): i32_` and `patterns_(): [i32]` would actually
conflict; trying to define both would be a compile error.

### arguments class

Variadic functions are possible in oh-lang using the `arguments_[of_]` class.
We recommend only one class type, e.g., `arguments_[int_]` for a variable number
of integers, but you can allow multiple classes via e.g. `arguments_[one_of_[x_, y_, z_]]`
for classes `x_`, `y_`, and `z_`.  oh-lang disallows mixing `arguments_` with any
other arguments.  The `arguments_` class has methods like `::count_()`, `;:[index]: (of)`,
and `;;[index]!`.  Thus, `arguments_` is effectively a fixed-length array, but you
can modify the contents if you pass as `arguments[type_];`.  It is guaranteed that
there is at least one argument, so `arguments[0]` is always defined.

```
max_(arguments[int_]): int_
    max; arguments[0]
    range_(1, arguments count_()) each index:
        if arguments[index] > max
            max = arguments[index]
    max
```

### dynamically determining arguments for a function

We allow for dynamically setting arguments to a function by using the `call_` type,
which has a few fields: `input` and `output` for the arguments and return values
of the function, as well as optional `prints` and `errors` fields for anything printed
to `stdout` or `stderr` when calling the function.  These fields are named to imply
that the function call can do anything (including fetching data from a remote server).

```
call_:
[   input; lot_[at_: str_, any_]
    output; lot_[at_: str_, any_]
    # things printed to stdout via `print_`:
    prints; array_[str_]
    # things printed to stderr via `error_`:
    errors; array_[str_]
]
{   # adds a named argument to the function call.
    # e.g., `call input_(at. "Cave", "Story")`
    ;;input_(at. str_, any.): null_
        input[at] = any

    ;;input_(any.): null_
        m input_(at. any type_id to_(), any)

    # TODO: this breaks the rule that we don't hold on to references/pointers
    # beyond the scope of the function.
    ;;input_(at. str_, any:;): null_
        input[at] = (at:; any)

    # adds a named field to the return type with a default value.
    # e.g., `call output_(at. "field_name", 123)` will ensure
    # `[field_name]` is defined in the return value, with a
    # default of 123 if `field_name` is not set in the function.
    ;;output_(at. str_, any.): null_
        output_[at] =  any

    # adds a default-named return type, with a default value.
    ;;output_(any.): null_
        ;;output_(at. any type_id to_(), any)
}
```

When passing in a `call_` instance to actually call a function, the `input` field will be treated
as constant/read-only.  The `output` field will be considered "write-only", except for the fact
that we'll read what fields are defined in `output` (if any) to determine which overload to use.
This call structure allows you to define "default values" for the output, which won't get
overwritten if the function doesn't write to them.  Let's try an example:

```
# define some function to call:
some_function_(x: int_): str_
    "hi" * x
# second overload:
some_function_(x: str_): int_
    x count_bytes_()

my_string: str_ = some_function_(x: 100)    # uses the first overload
my_int: int_ = some_function_(x: "cow")     # uses the second overload because the return type is asked for.
NAMED_int: some_function_(x: "wow")         # also the second overload because it's asked for (via `int` name)
check_type1: some_function_(x: 123)         # uses the first overload since it's the default for an `x` arg.
check_type2: some_function_(x: "asdf") int  # uses the second overload because the return type is asked for.
# TODO: we may want to support this in the future without compile errors.
check_type3: some_function_(x: "asdf")      # COMPILE ERROR: "asdf" is not referenceable as `int`
invalid: some_function_(x: 123.4)           # COMPILE ERROR: 123.4 is not referenceable as `int`

# example which will use the default overload:
call; call_
call input_(at. "x", 2)
# use `call` with `;` so that `call;;output` can be updated.
some_function_(;call)
print_(call output) # prints `["str": "hihi"]`

# define a value for the object's output field to get the other overload:
call; call_
call input_(at. "x", "hello")
call output_(-1)    # defines a default-named `int` output, defaulting to -1.
some_function_(;call)
print_(call output) # prints `["int": 5]`

# dynamically determine the function overload:
call; call_
if some_condition_()
    call@ {input_(at. "x", 5), output_("?")}
else
    call@ {input_(at. "x", "hey"), output_(-1)}

some_function_(;call)
print_(call output)  # will print `["str": "hihihihihi"]` or `["int": 3]` depending on `some_condition()`.
```

Note that `call` is so generic that you can put any fields that won't actually
be used in the function call.  In this, oh-lang will return an error at run-time.

```
call; call_() @{ output_(at. "value1", 123), output_(at: "value2", 456) }
some_function_(;call) assert_() # returns error since there are no overloads with [value1, value2]
```

If compile-time checks are desired, one should use the more specific
`my_fn_ call_` type, with `my_fn_` the function you want arguments checked against.

```
# throws a compile-time error:
call; some_function_ call_() @{ output_(at. "value1", 123), output_(at. "value2", 456) }
# the above will throw a compile-time error, since two unexpected fields are defined for output.

# this is ok (calls first overload):
call2; some_function_ call_() @{ input_(at. "x", "4"), output_(0) }
# also ok (calls second overload):
call3; some_function_ call_() @{ input_(at. "x", 4), output_("") }
```

Note that it's also not allowed to define an overload for the `call` type yourself.
This will give a compile error, e.g.:

```
# COMPILE ERROR!!  you cannot define a function overload with a default-named `call` argument!
some_function_(call;): null_
    print_(call input["x"])
```

This is because all overloads need to be representable by `call; call_`, including any
overloads you would create with `call`.  Instead, you can create an overload with a
`call` argument that is not default-named.

```
some_function_(my_call; call_): null_
    print_(my_call input["x"])  # OK
```

### callable

Taking the `call` idea one step further, we can have a pointer to the function
already ready so all we need to do is run `callable call_()`.  To specify the overload,
the input and output variables need to be named inside the `callable`.


## mutable functions

To declare a reassignable function, use `;` after the arguments.

```
greet_(noun: string_); null_
    print_("Hello, ${noun}!")

# you can use the function:
greet_(noun: "World")

# or you can redefine it:
greet_(noun: string_); null_
    print_("Overwriting!")
# it's not ok if we use `greet_(noun: string_): null_` when redefining
# since that looks like we're just defining another overload.
```

It needs to be clear what function overload is being redefined (i.e., having
the same function signature), otherwise you're just creating a new overload
(and not redefining the function).  You can also assign an existing function
to a mutable function using notation like this:
`my_mutable_fn_(); int {original_definition_()}`, followed by
`my_mutable_fn_(); int = some_other_fn_`.  Because the overload on `my_mutable_fn_`
is fully specified (e.g., no input arguments, an `int_` return value), we know
to select that overload on `some_other_fn_`.

## nullable functions

The syntax for declaring a nullable/optional function is to put a `?`
after the function name but before the argument list.  E.g.,
`optional_function_?(...args): return_type_` for a non-reassignable
function and swapping `:` for `;` to create a reassignable function.
When calling a nullable function, unless the function is explicitly
checked for non-null, the return type will be nullable.  E.g.,
`x?: optional_function_(...args)` will have a type of
`one_of_[return_type_, null]_`.  Nullable functions are checked by
the executable, so the programmer doesn't necessarily have to do it.

A nullable function has `?` before the argument list; a `?` *after* the argument list
means that the return type is nullable.  The possible combinations are therefore the following:

* `normal_function_(...args): return_type_` is a non-null function
  returning a non-null `return_type_` instance.

* `nullable_function_?(...args): return_type_` is a nullable function,
  which, if non-null, will return a non-null `return_type_` instance.
  Conversely, if `nullable_function_` is null, trying to call it will return null.

* `nullable_return_function_(...args)?: return_type_` is a non-null function
  which can return a nullable instance of `return_type_`.

* `super_null_function_?(...args)?: return_type_` is a nullable function
  which can return a null `return_type_` instance, even if the function is non-null.
  I.e., if `super_null_function_` is null, trying to call it will return null,
  but even if it's not null `super_null_function_` may still return null.

```
# creating an optional function in a class:
example_: [x: dbl_, optional_fn_?(m:, z: dbl_); int_]

example; example_(x: 5)

# define your own function for `optional_fn_`:
example::optional_fn_(z: dbl_); int_
    floor_(z * m x)

# or set it to null:
example optional_fn_?(z: dbl_); int_ = null

# after setting it to null...
example optional_fn_(z: 3.21)   # returns null
```

## generic/template functions

We can have arguments with generic types, but we can also have arguments
with generic names.

### argument type generics

For functions that accept multiple types as input/output, we define template types
inline, e.g., `copy_(value: ~t_): t_`, using `~` for where the compiler should infer
what the type is.  You can use any unused identifier for the new type, e.g.,
`~q_` or `~sandwich_type_`.

```
copy_(value: ~t_): t_
    print_("got $(value)")
    t_(value)

vector3_: [x: dbl_, y: dbl_, z: dbl_]
vector3: vector3_(y: 5)
result: copy_(value: vector3)   # prints "got vector3_(x: 0.0, y: 5.0, z: 0.0)".
vector3 == result               # equals True
```

You can also add the new types in brackets just after the function name,
e.g., `copy_[t_: my_type_constraints_](value: ~t_): t_`, which allows you to specify any
type constraints (`my_type_constraints_` being optional).  Note that types defined with
`~` are inferred and therefore can never be explicitly given inside of the brackets,
e.g., `copy_[t_: int_](value: 3)` is invalid here, but `copy_(value: 3)` is fine.

If you want to require explicitly providing the type in brackets, don't use `~` when
defining the function.

```
# this generic function does not infer any types because it doesn't use `~`.
copy_[the_type_](value: the_type_): the_type_
    ...
    the_type_(value)

# therefore we need to specify the generics in brackets before calling the function.
copy_[the_type_: int_](value: 1234) # returns 1234
```

For this example, it would probably be better to use `of_` instead of `the_type_`,
since `of_` is the "default name" for a generic type.  E.g., you don't need
to specify `[of_: int_]` to specialize to `int_`, you can just use `[int_]
for an `[of_]`-defined generic.  See also
[default named generic types](#default-named-generic-types).  For example:

```
# this generic function does not infer any types because it doesn't use `~`.
copy_[of_](value: of_): of_
    ...
    of_(value)

# because the type is not inferred, you always need to specify it in brackets.
# you can use `of_: the_type_` but this is not idiomatic:
copy_[of_: int_](value: 3)  # will return the integer `3`

# because it is default named, you can just put in the type without a field name.
copy_[dbl_](value: 3)     # will interpret `3` as a double and return `3.0`
```

### default-named generic arguments

TODO: restrictions here, do we need to only have a single argument, so that
argument names are unique?  it's probably ok if we have an `@order_independent`
or use `~FIRST_t_` and `~SECOND_u_` to indicate order is ok.
or need to use `NAMED_` on some of them.
maybe we see if there's an issue when compiling the generics and then complain at compile time.

Similar to the non-generic case, if the `variable_case` identifier
matches the `type_case_` type of a generic, then it's a default-named argument.
For example, `my_type; ~my_type_` or `t: ~t_`.  There is a shorthand for this
which is more idiomatic: `~my_type;` or `~t:`.  Here is a complete example:

```
logger_(~t): t_
    print_("got ${t}")
    t

vector3_: [x: dbl_, y: dbl_, z: dbl_]
vector3: vector3_(y: 5)
result: logger_(vector3)    # prints "got vector3(X: 0.0, Y: 5.0, Z: 0.0)".
vector3 == result           # equals true

# implicit type request:
int_result: logger_(5)      # prints "got 5" and returns the integer 5.

# explicit type passing:
dbl_result: logger_(dbl_(4))    # prints "got 4.0" and returns 4.0
```

Note that you can use `my_function_(~t;)` for a writable argument.
Default naming also works if we specify the generics ahead of
the function arguments like this:

```
logger_[of_: some_constraint_](of.): of_
    print_("got ${of}")
    of

# need to explicitly add the type since it's never inferred.
logger_[int_](3)  # returns the integer `3`
logger_[dbl_](3)  # will return `3.0`
```

If you want people to pass in the argument with the field name explicit,
you can use the `NAMED_` namespace.  This suppresses the default naming.

```
logger_(~NAMED_of.): of_
    print_("got ${of}")
    of

# need to explicitly add the argument name `of` but
# the type can be inferred due to `~` in the definition.
logger_(of. 3)  # returns the integer `3`
```

If we have a named generic type, just name the `type_case_` type the
same as the `variable_case` variable name (just add a trailing `_`)
so default names can apply.

```
logger_[value_](value.): value_
    print_("got ${value}")
    value

logger_[value_: dbl_](3)    # will return `3.0` and print "got 3.0"
```

If we want to suppress default naming, e.g., require the function argument 
to be `value: XYZ`, then we need to explicitly tell the compiler that we don't
want default names to apply, which we do using the `NAMED_` namespace.

```
logger_(~NAMED_value.): value_
    print_("got ${NAMED_value}")
    NAMED_value

# because of the `~` on the type, it can be called like this,
# which implicitly infers the `value_` type:
logger_(value. 3)   # returns the integer `3`
```

And as in other contexts, you can avoid inferring the type by omitting `~`.

```
# this generic needs to be specified in brackets at the call site: 
logger_[value_](NAMED_value.): value_
    ...
    NAMED_value

# and because it has a `NAMED_` namespace,
# you need to call it like this:
logger_[value: dbl](value. 3)  # will return `3.0`
```

### argument name generics: with different type

You can also define an argument with a known type, but an unknown name.
This is useful if you want to use the inputted variable name at the call site
for logic inside the function, e.g., `this_function_(whats_my_name: 5)`.
You can access the variable name via `@@`.  Internally this creates an overload
for `this_function_(argument_at: str_, argument: known_type_)`, so
`@@argument` will be an alias for `argument_at`.

```
this_function_(~argument: int_): null_
    argument_name: str_(@@argument)
    print("calling this_function with ${argument_name}: ${argument}")

# internally defines this overload:
this_function_(argument_at: str_, argument: int_): null_
    argument_name: str_(argument_at)
    print("calling this_function with ${argument_name}: ${argument}")

# and this overload, but this is only for `call_`ers.
this_function_(argument: int_): null_
    this_function(argument_at: "argument", :argument)

# TODO: there's probably some way to define something like this;
# but we don't actually want to alias this.
@alias this_function_(@match_field(argument_at, argument): int_): null_
    this_function_(:argument_at, argument)
```

Defining such an overload will of course make it impossible to define any
other overloads with one argument, because overloads must be distinguishable
by argument names.  It will also restrict any two argument overloads
from having field names `argument` and `argument_at`.

You can define both the argument name and argument type to be generic,
e.g., `my_function_(~my_name: ~another_type_)`.
TODO: can we use a different syntax for `~my_name` so that new argument
types like `fn_(~t:): t_` expanding to `fn_(t: ~t_): t_`, which should
be the default, will not be inconsistent?  maybe `my_function_(@@my_name: ~new_type_)`.
yeah i think i like `fn_(@@int:)`, `fn_(~@@t:)`, and `fn_(~t:`).


### require

`require` is a special generic field that allows you to include a function,
method, or variable only if it meets some compile-time constraints.  It is
effectively a keyword within a generic specification, so it can't be used
for other purposes, and the boolean value it takes must be known at compile-time.
It also can be used to ensure a generic specification satisfies some constraints,
e.g., `require: n > 0` in the first line below.  `require` can *never* be
specified by hand, e.g., `my_class_[int_, n: 0, require: true]` is a compile error.

```
my_class_[of_, n: count_, require: n > 0]:
[   value: of_
    # this field `second_value` is only present if `n` is 2 or more.
    # TODO: does this conflict with any other usages of generic classes?
    # if so, let's switch to `@if n >= 2 {second_value: of_}`
    second_value[require: n >= 2]: of_
    # this field `third_value` is only present if `n` is 3 or more.
    third_value[require: n >= 3]: of_
    ... # plz help am i coding this right??     (no, prefer `vector_[n, of_]`)
]
{   # `of_ is hashable_` is true iff `of_` extends `hashable_` either explicitly
    # or implicitly by implementing a `hash_` method like this:
    ::hash_[require: of_ is hashable_](~builder):
        builder hash_(value)
        @if n > 1 {builder hash_(second_value)}
        @if n > 2 {builder hash_(third_value)}
        # TODO: maybe add something like `builder hash_(@?second_value)`
        ...
}
```

TODO: should we make this an annotation instead?  `@require(of is orderable)`??
in C++, these sorts of things are templates, but that can be kinda confusing.
but it does allow you to do things like this, where you introduce new types
on the fly and can require them to be a certain way:
`::do_[additional_type_, require: additional_type_ is foo_](~additional_type:): int_`
so if possible, i think i'd prefer to keep it as a template.
the alternative is to do `@if of_ is hashable_ { ::hash_(~builder): ... }`.

# classes

A class is defined with a `type_case_` identifier, an object `[...]`
defining instance variables and instance functions (i.e., variables and
functions that are defined *per-instance* and take up memory), and an
optional indented block (optionally in `{}`) that includes methods and functions that are
shared across all instances: class instance methods (just *methods* for short)
and class functions (i.e., static methods in C++/Java) that don't require an instance.
Class definitions must be constant/non-reassignable, so they are declared using
the `:` symbol.

When defining methods or functions of all kinds, note that you can use `m`/`m_`
to refer to the current class instance/type.  E.g.,

```
# classes can enclose their body in `{}`, which is recommended for long class definitions.
# for short classes, it's ok to leave braces out.
my_class_: [variable_x: int_]
    ::o_(): m_   # OK
        print_("logging a copy")
        m_(m variable_x o_())
```

Inside the class body, you must use `m` to refer to any other instance variables or methods,
e.g., `m x` or `m do_stuff_method_()`.  This isn't as concise as C++/Java, but it is much
more precise and ensures you don't need to worry about too many name collisions with globals.
You don't need to prefix `m_` on any class variables, types, or functions that are defined
inside the class body, which does mean name collisions with globals can be an issue, but
these will be thrown as errors by the compiler.

Note that when returning a newly-declared type from a function (e.g., `my_fn_(int:): [x: int_, y: dbl_]`),
we do not allow building out the class body; any indented block will be assumed
to be a part of the function body/definition:

```
my_fn_(int:): [x: int_, y: dbl_]
{   # this is part of the `my_fn_` definition,
    # and never a part of the `[x: int_, y: dbl_]` class body.
    [x: 5, y: 3.0]
}
```

If you want to specify methods on a return type, make sure to build it out as a separate
class first.

```
x_and_y_: [x: int_, y: dbl_]
{   ::my_method_(): x + round_(y) int
}

my_fn_(int): x_and_y_
    [x: int + 5, y: 3.0]
```

## example class definition

```
parent_class_: [name: str_]

# example class definition
example_class_: all_of_
[   parent_class_
    m_:
    [   # given the inheritance with `parent_class_`,
        # child instance variables should be defined in this `m_: [...]` block.
        # if they are public, a public constructor like `example_class_(x;:. int_)`
        # will be created.
        x; int_

        # instance functions can also be defined here.  they can be set 
        # individually for each class instance, unlike a class function/method
        # which is shared.
        # we define a default for this function but you could change its definition in a constructor.
        # NOTE: instance functions can use `m` as necessary.
        #       even though we could use the notation `::instance_function_()` here,
        #       we prefer to keep that for methods, to make it more clear that
        #       this is different in principle.
        instance_function_(m:): null_
            print_("hello ${m x}!")

        # this class instance function can be changed after the instance has been created
        # (due to being declared with `;`), as long as the instance is mutable.
        some_mutable_function_(); null_
            print_("hello!")
    ]
]
{   # classes must be resettable to a blank state, or to whatever is specified
    # as the starting value based on a `renew_` function.  this is true even
    # if the class instance variables are defined as readonly.
    # NOTE:  defining this method isn't necessary since we already would have had
    # `example_class_(x: int_)` based on the public variable definition of `x`, but
    # we include it as an example in case you want to do extra work in the constructor
    # (although avoid doing work if possible).
    ;;renew_(x. int_): null_
        parent_class renew_(name: "Example")
        m x = x!
    # or short-hand: `;;renew_(m x. int_, parent_class name: "Example"): {}`
    # adding `m` to the arg name will automatically set `m x` to the passed in `x`.

    # create a different constructor.  constructors use the class reference `m` and must
    # return either an `m_` or a `hm_[ok_: m_, er_]` for any error type `er_`.
    # this constructor returns `m_`:
    m_(k: int_): m_(x. k * 1000)

    # some more examples of class methods:
    # prefix `::` (`;;`) is shorthand for adding `m: m_` (`m; m_`) as an argument.
    # this one does not change the underlying instance:
    ::do_something_(int:): int_
        print_("My name is ${m name}")   # `m name` will check child first, then parents.
        # also ok, if we know it's definitely in `parent_class_`:
        print_("My name is ${parent_class name}")
        x + int     # equivalent to `m x + int`

    # this method mutates the class instance, so it uses `;;` instead of `::`:
    ;;add_something_(int:): null_
        x += int    # equivalent to `m x += int`

    # COMPILE ERROR: reassignable methods are currently not supported;
    # they may be in the future but would require hotswapping functions.
    # in case someone is running an old function, we need to let them
    # finish before reclaiming the memory of that function.
    ::reassignable_method_(int:); str_
        str_(x + int)

    # some examples of class functions:
    # this function does not require an instance and cannot use instance variables:
    some_static_function_(y; int_): int_
        y /= 2
        y!

    # this function also does not require an instance and cannot use instance variables:
    some_static_function_(y: int_): null_
        write_(y, file: "y.txt")
}

example; example_class_(x: 5)
print_(example do_something_(7))    # should print 12
example = example_class_(x: 7)      # note: variable can be reassigned.
example x -= 3                      # internal fields can be reassigned as well.

# note that if you define an instance of the class as readonly, you can only operate
# on the class with functions that do not mutate it.
const_var: example_class_(x: 2)
const_var x += 3                    # COMPILER ERROR! `const_var` is readonly.
const_var = example_class_(x: 4)    # COMPILER ERROR! variable is readonly.

# calling class functions doesn't require an instance.
dont_need_an_instance: example_class_ some_static_function_(y; 5)
```

Note that you normall call a static/class function like this `class_name_ class_function_(...)`,
but you can also do it like this: `class_function_(m_: class_name_, ...)`.  This is similar to
how you can get internal class types like `class_name_ internal_type_` in a different way like
`internal_type_[m_: class_name_]`.

## declaring methods and class functions outside of the class

You can also define your own custom methods/functions on a class outside of the class body.
Note that we do not allow adding instance functions or instance variables outside
of the class definition, as that would change the memory footprint of each class instance.
You can also use [sequence building](#sequence-building) outside of the class to define
a few methods, but don't use `:` since we're no longer declaring the class.

```
# static function that constructs a type or errors out
example_class_(z: dbl_): hm_[ok_: example_class_, er_: str_]
    x: z round_() int_() assert_(er: "Need `round_(z)` representable as an `int`.")
    example_class_(x)

# static function that is not a constructor.
# this function does not require an instance, and cannot use instance variables,
# but it can read (but not write) global variables (or other files):
example_class_ some_static_function_(): int_
    y_string: read_(file: "y.txt")
    return int_(?y_string) ?? 7

# a method which can mutate the class instance:
# this could also be defined as `example_class_ another_method_(m;, plus_k: int_): null_`.
example_class_;;another_method_(plus_k: int_): null_
    # outside of a class body, `m` is required to namespace any instance fields,
    # because they are not obviously in scope here like in a class body.
    m x += plus_k * 1000

# Use sequence building.
example_class_@
{   # with sequence building
    # `example_class_ my_added_class_function_(k: int_): example_class_`
    # is exactly how you'd define a class function.
    my_added_class_function_(k: int_): example_class_
        example_class_(x. k * 1000)

    # a method which keeps the instance readonly:
    ::my_added_method_(y: int_): int_
        m x * 1000 + y * 100
}
```

If they are public, you can import these custom methods/functions in other files in two
ways: (1) import the full module via `[*]: \/relative/path/to/file` or `[*]: \\library/module`,
or (2) import the specific method/function via e.g.,
`{example_class_ my_added_class_function_(k: int_): example_class_} \/relative/path/to/file`
or `{example_class_::my_added_method_(y: int_): int_} \\library/module`.

Note that we recommend using named fields for constructors rather than static
class functions to create new instances of the class.  This is because named fields
are self descriptive and don't require named static functions for readability.
E.g., instead of `my_date: date_class_ from_iso_string_("2020-05-04")`, just use
`my_date: date_class_(iso_string: "2020-05-04")` and define the
`;;renew_(iso_string: string_)` method accordingly.

## destructors

The `;;renew_(Args...): null_` (or `: hm_[ok_: m_, er_: ...]`) constructors
are technically resetters.  If you have a custom destructor, i.e., code
that needs to run when your class goes out of scope, you shouldn't define
`;;renew_` but instead `;;descope_(): null_` for the destructor and
`m_(...): m_` for the constructor.  It will be a compile error if you try
to define any of `m_` or `;;renew_` with the same arguments.

```
destructor_class_: [x: int_]
{   # TODO: the `@debug` annotation should do something interesting, like
    #       stop the debugger when the value is `set`ted or `get`ted.
    m_(DEBUG_x. int_): m_
        print_("x ${DEBUG_x}")
        [x. DEBUG_x]
    # `m_(...): m_` will also add methods like this:
    #   ;;renew_(DEBUG_x. int_): null_
    #       # this will call `m descope_()` just before reassignment.
    #       m = m_(.DEBUG_x)

    # you can define the destructor:
    ;;descope_(): null_
        print_("going out of scope, had x ${x}")
        # note that destructors of instance variables (e.g., `x`)
        # will automatically be called, in reverse order of definition.
}
```

Destructors are called before instance variables are descoped.
Child class destructors only need to clean up their own instance variables;
they will be called before the parent class destructor (which will automatically
be called).

## instance functions, class functions, and methods

Class methods can access instance variables and call other class methods,
and require a `m: m_` argument to indicate that it's an instance method.
Mutating methods -- i.e., that modify the class instance, `m`, i.e.,
by modifying its values/variables -- must be defined with `m;` in the
arguments.  Non-mutating methods must be defined with `m:` and can access
variables but not modify them.  Methods defined with `m.` indicate that
the instance is temporary.  We'll use the shorthand notation
`some_class_..temporary_method_()` to refer to a temporary instance method,
`some_class_;;some_mutating_method_()` to refer to a mutable instance method, and
`some_class_::some_method_()` to refer to a readonly instance method.
Calling a class method does not require the `..`, `;;`, or `::` prefix,
but it is allowed, e.g.,

```
some_class; some_class_("hello!")
some_class some_method_()           # ok
some_class::some_method_()          # also ok
some_class some_mutating_method_()  # ok
some_class;;some_mutating_method_() # also ok
# you can get a temporary by using moot (!):
# NOTE: `..` isn't necessary here because of `some_class!`
# already being a temporary:
my_result1: some_class!..temporary_method_()
# or you can get a temporary by creating a new class instance;
# also `..` isn't necessary because `some_class_(...)` is already a temporary.
my_result2: some_class_("temporary")..temporary_metho_d()
```

Note that you can overload a class method with readonly instance `::`,
writable instance `;;`, and temporary instance `..` versions.  If it's
unclear, callers are recommended to be explicit and use `::`, `;;`, or `..`
instead of ` ` (member access).  See the section on member access operators
for how resolution of ` ` works in this case.

You can also call a class method via an inverted syntax, e.g.,
`some_method_(:some_class)`, `some_mutating_method_(;some_class)`,
or `temporary_method_(.some_class)`, with any other arguments to the method added as well.
This is useful to overload e.g., the printing of your class instance, via defining
`print_(m:)` as a method, so that `print_(some_class)` will then call `some_class::print_()`.
Similarly, you can do `count_(some_class)` if `some_class` has a `some_class::count_()`
method, which all container classes have.  This also should work for multiple argument methods,
since `array swap_(FIRST_index., SECOND_index.)` can easily
become `swap_(array;, FIRST_index., SECOND_index.)`.
TODO: we probably can allow `index_1` and `index_2` to resolve type as `index_`.  we don't want
to disallow numbers in class names, e.g., `vector3`, so maybe we require using underscores like
`index_1`, so we can do things like `vector3_2`.  not great, but not awful.

And of course, class methods can also be overridden by child classes (see section on overrides).

Class functions can't depend on the instance, i.e., `m`.  They can
be called from the class name, e.g., `x_ my_class_function_()`.
Because oh-lang makes it easy to refer to a variable `the_variable`'s class
as `the_variable_` (even if `the_variable` is of type `some_other_type_`),
it is easy to call a class function as `the_variable_ the_class_function_()`,
which means that we can distinguish between class functions with the same name
and arguments as shadow class methods, as long as no `m` argument is present.

Instance functions are declared like instance variables, inside the `[...]` block.
Instance functions can be different from instance to instance.
They cannot be overridden by child classes but they can be overwritten.  [Oprah meme]
I.e., if a child class defines the instance function of a parent class,
it overwrites the parent's instance function; calling one calls the other.
(And because of this, you can't call the parent's instance function in a `super` way.)

Class constructors can be defined in two ways, as a method resetter or class function.
Class *method* resetters are defined with the function signature
(a) `;;renew_(args...): null_` or (b) `;;renew_(args...): hm_[ok_: null_, er_: ...]`,
and these methods also allow you to renew an existing class instance as long as
the variable is writable.  Class *function* constructors are defined like
(c) `m_(args...): m_` or (d) `m_(args...): hm_[ok_: m_, er_: ...]`.
In both (a) and (c) cases, you can use them like `my_val: my_class_(args...)`,
and for (b) and (d) you use them like `my_var: my_class_(args...) assert_()`.

The first constructor defined in the class is also the default constructor,
which will be called with default-constructed arguments (if any) if a default
instance of the class is needed.  It is a compiler error if a constructor with
zero arguments is defined after another constructor with arguments.


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
defined as `mutating_method_(m;): return_type_` (AKA `;;mutating_method_(): return_type_`)
or `non_mutating_method_(m:): return_type_` (AKA `::non_mutating_method_(): return_type_`).
Mutating methods follow the "mutate" visibility in the table above, and non-mutating methods
follow the "access" visibility in the table above.

To put into words -- `@public` methods can be called by anyone, regardless
of whether the method modifies the class instance or not.  `@protected`
methods which modify the class instance cannot be called by non-friends,
but readonly `@protected` methods can be called by anyone.  `@private` methods which
modify the class instance can only be called by module functions, and
readonly `@private` methods can be called by friends.

One final note, child classes are considered friends of the parent class,
even if they are defined outside of the parent's directory, and even if they
are defined in the same module as the parent.  What this means
is they can modify public and protected variables defined on the parent instance,
and read (but not modify) private variables.  Overriding a parent class method
counts as modifying the method, which is therefore possible for public and protected
methods, but not private methods.

## getters and setters on class instance variables

TODO: not sure this is actually relevant anymore; we've removed MMR in favor of reference getters.
we probably don't actually want to do this, based on the TODO below things are a bit confusing
on how we'd resolve different situations.  we probably don't want to make `m x = 3` do anything surprising.
but we probably want to keep the "expands to this" example as a way to do idiomatic getters/setters/swappers.

Note that all variables defined on a class are given methods to access/set
them, but this is done with syntactical sugar.  That is,
*all uses of a class instance variable are done through getter/setter methods*,
even when accessed/modified within the class.  The getters/setters are methods
named the `function_case_` version of the `variable_case` variable,
with various arguments to determine the desired action.
TODO: at some point we need to have a "base case" so that we don't infinitely recurse;
should a parent class not call the child class accessors?  or should we only
not recurse when we're in a method like `;;x_(NEW_x.): { m x = NEW_x }`?  or should we
avoid recursing if the variable was defined in the class itself?  (probably the
latter, as it's the least surprising.)

```
# for example, this class:
example_: [@visibility x; str_("hello")]
w; example_()
w x += ", world"
print_(w x) # prints "hello, world"

# expands to this:
example_:
[   @invisible
    x; str_("hello")
]
{   # no-copy readonly reference getter.
    @visibility
    ::x_(): (str:)
        (str: m x)

    # no-copy writable reference getter.
    ;;x_(): (str;)
        (str; m x)

    # copy getter; has lower priority than no-copy getters.
    ::x_(): str_
        m x

    # setter.
    ;;x_(str.):
        m x = str!

    # swapper: swaps the value of `x` with whatever is passed in.
    @visibility
    ;;x_(str;):
        m x <-> str

    # no-copy "take" method.  moves `x` from this temporary.
    @visibility
    ..x_(): m x!
}
w = example_()
w x_() += ", world")
print_(w x_())
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
just_copyable_: [@invisible a_var; int_]
{   ::some_var_(): int_
        m a_var - 1000

    ;;some_var_(int.): null_
        m a_var = int + 1000

    #(#
    # the following references become automatically defined;
    # they are just thin wrappers around the getters/setters.

    # writable reference
    ;;some_var_(): (int;)
        refer_
        (   ;m
            {$o some_var_()}        # getter: `o` is an instance of `just_copyable_`
            {$o some_var_(.$int)}   # setter
        )

    # readonly reference
    ::some_var_(): (int:)
        refer_
        (   :m
            {some_var_(:$o)}
        )

    # TODO: we can use reflection to do this, but probably not in a generic way
    # if the getter is an opaque function.
    # similarly a no-copy take method becomes defined based on the getter.
    ..some_var_(): int
        m a_var! - 1000
    #)#
}

# a class with a swapper method gets a setter and taker method automatically:
just_swappable_: [@invisible some_var; int_]
{   @visibility
    ;;some_var_(int;): null_
        m some_var <-> int
        # you can do some checks/modifications on `some_var` here if you want,
        # though it's best not to surprise developers.  a default-constructed
        # value for `some_var` (e.g., in this case `int: 0`) should be allowed
        # since we use it in the modifier to swap out the real value into a temp.
        # if that's not allowed, you would want to define both the swapper
        # and modifier methods yourself.

    #(#
    # the following setter becomes automatically defined:
    ;;some_var_(int.): null_
        m some_var_(;int)

    # and the following take method becomes automatically defined:
    ..some_var_(): int_
        temporary; int_
        # swap `some_var` into `temporary`:
        m some_var_(;temporary)
        temporary!
    #)#
}

# a class with a readonly reference getter method gets a copy getter automatically:
just_gettable_: [@invisible some_var; int_]
{   ::some_var_(): (int:)
        (int: some_var)

    #(#
    # the following becomes automatically defined:
    ::some_var_(): int_
        (int:) = m some_var()
        int
    #)#
}

# a class with a writable reference method gets a swapper and taker method automatically:
just_referable_: [@invisible some_var; int_]
{   ;;some_var_(): (int;)
        (int; m some_var)

    #(#
    # the following swapper becomes automatically defined:
    ;;some_var_(int;): null_
        m some_var_() <-> int

    # the following setter becomes automatically defined:
    ;;some_var_(int.): null_
        some_var_() = int!

    # and the following taker method becomes automatically defined:
    ..some_var_(): int_
        result; int_
        result <-> some_var_()
        result
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
For one parent, `child_class_: parent_class_name_ {#( child methods )#}`.  Multiple
inheritance is allowed as well, via `all_of_[parent1_, parent2_] {#( child methods )#}`.
If you want to add child instance fields, use e.g., `all_of_[parent_, m_: [field1: int_, ...]]`
to add a big-integer `field1` to the child class.  With a slight overload of notation,
We can access the current class instance using `m`, and `m_` will be the current instance's
type.  Thus, `m_` is the parent class if the instance is a parent type, or a subclass
if the instance is a child class.  E.g., a parent class method can return a `m_` type instance,
and using the method on a subclass instance will return an instance of the subclass.
If your parent class method truncates at all (e.g., removes information from child classes),
make sure to return the same `parent_class_name_` that defines the class.
TODO: is there a better name for truncated classes?  or can we even really promise
that `m_` will be polymorphic here?  maybe `m_` only returns the class itself (i.e., parent),
unless it's a reference, in which case it could be a child class.
or if we see that `m_` is returned in any method, we require it to be overridden
for all child classes that add any instance fields.

We can access member variables or functions that belong to that the parent type,
i.e., without subclass overloads, using the syntax `parent_class_name some_method_(...args)`
oh-lang doesn't have a `super` keyword because we want inheritance to be as clear as composition
for how method calls work.

Some examples:

```
animal_: [name: string_]
{   ;;renew_(m name. string_): {}

    # define two methods on `animal_`: `speak_` and `go_`.
    # these are "abstract" methods, i.e., not implemented by this base class.
    ::speak_(): null_
    ::go_(): string_

    # this method is defined, so it's implemented by the base class.
    # derived classes can still override it, but this default will be defined for them.
    ::escape_(): null
        print_("${m name} ${m go_()} away!!")

    # copy method that returns an instance of whatever the class instance
    # type is known to be.  e.g., an animal returns an animal instance,
    # while a subclass would return a subclass instance:
    ::o_(): m_
        m_(name o_())
}

snake_: animal_
{   # if no `renew_` functions are defined,
    # child classes will inherit their parent `renew_()` methods.

    ::speak_(): null_
        print_("hisss!")
    ::go_(): string_
        "slithers"

    # no need to override `o_`, since we can copy a snake just by name like the parent.
}

snake: snake_(name. "Fred")
snake escape_()  # prints "Fred slithers away!!"
```

To define extra instance variables for a child class, you'll use this notation:

```
cat_: all_of_[animal_, m_: [fur_balls: int_]]
{   # here we define a `renew_` method, so the parent `renew_` methods
    # become hidden to users of this child class:
    ;;renew_(): null
        # can refer to parent methods using the `variable_case`
        # version of the `type_case_` class name:
        animal renew_(Name: "Cat-don't-care-what-you-name-it")
        m fur_balls = 0

    ::speak_(): null_
        print_("hisss!")
    ::go_(): string_
        "saunters"

    ::escape_(): null_
        print_("CAT ESCAPES DARINGLY!")

    # the parent `o_()` method won't work (`cat_` has instance fields)
    # so we need to override:
    ::o_(): m
        # cats are essentially singletons, that cannot have their own name;
        m_()
}

cat: cat_()
cat escape_()   # prints "CAT ESCAPES DARINGLY!"
```

We have some functionality to make it easy to pass `renew_` arguments to
a parent class via the `parent_class_name` namespace in the constructor arguments.
This way you don't need to add the boiler plate logic inside the
constructor like this `;;renew_(parent_argument:): parent renew_(:parent_argument)`,
you can make it simpler like this instead:

```
horse_: all_of_[animal_, m_: [owner: str_]]
{   # this passes `name` to the `animal_` constructor and sets `owner` on self:
    ;;renew_(animal name: str_, m owner: str_, neigh_times: int_ = 0)
        neigh_times each int_:
            m speak_()

    ::speak_(): null_
        print_("neigh!")

    ::go_(): string_
        "gallops"
}

horse: horse_(name: "James", owner: "Fred", neigh_times: 1)
print_(horse owner) # Fred
print_(horse name)  # James
```

All abstract base classes also provide ways to instantiate using lambda functions.
All abstract methods must be defined for the instance to be created, and if a
`renew_` method is defined on the parent, any arguments passed into the first renew
(i.e., which is the default constructor) should be defined for the lambda class.
While these don't look like normal lambda functions, they use the notation `::speak_(): null`
as a shortcut for `speak_(m:): null_`, which works as a lambda.

```
weird_animal: animal_
(   name: "Waberoo"
    ::speak_(): null_
        print_("Meorooo")
    ::go_(): "meanders"
    ::escape_(): null_
        # to call the parent method `escape_()` in here, we can use this:
        animal::escape_()
        print_("${m name} ${m go_()} back...")
        # or we can use this:
        animal escape_(M)
)

weird_animal escape_()  # prints "Waberoo meanders away!!", "Waberoo meanders back...", "Waberoo meanders away!!"
```

## operator overloading

To overload operators for a class, we use the following syntax.

```
# this class checks for overflow/underflow and switches to a "null" (-128) if so.
flow8_: [i8;]
{   ;;renew_(m i8. -128): {}

    # cloning works without errors:
    ::o_(): m_
        [m i8]

    ::!(): bool_    # overload `!m`
        m i8 == 0 || m i8 == -128

    ;;+=(o): null_
        if m i8 == -128
            return
        if o i8 == -128
            m i8 = -128
            return
        i16. m i8 + o i8
        m i8 = i8_(i16) map_({$_er, -128})

    ::+(o): flow8_
        copy; m o_()
        copy += o
        copy
}
```

And similarly for all other operators.

## inheritance and dynamic allocation

In C++, a variable that is typed as a parent instance cannot be a child instance in disguise;
if a child instance is assigned to the variable, the extra derived bits get sliced off.  This
helps avoid dynamic allocation, because the memory for the parent class instance can be allocated
on the stack.  On the other hand, if the variable is a pointer to a parent instance, the variable
can actually point to a child instance in disguise.  This is great in practice for object-oriented
programming, since you can use the child instance in place of the parent instance; as long as
the child class fulfills the same contract as the parent class, it shouldn't matter the exact
implementation.  But this generally requires dynamic memory allocation, which has a cost.

In oh-lang, we want to make it easy for variables of a parent class to be secretly instances
of child classes, but we don't always require paying the cost of dynamically allocating types;
we do this by always padding out parent classes a little bit more (based on known child class
requirements) so they have more space for child-class fields.  Children that have much higher
memory requirements compared to the parent will get dynamically allocated, but this happens behind
the scenes.  This makes it possible to do things like this:

```
some_animal; animal_
some_animal = snake_(name. "Josie")
some_animal go_()   # prints "slithers"
some_animal = cat_()
some_animal go_()   # prints "saunters"
```

This is less surprising than the C++ behavior.  But in cases where users want to gain back
the no-extra-space and no-dynamically-allocated class instances, we have a `@only` annotation
that can be used on the type.  E.g., `some_variable: @only some_type_` will ensure that
`some_variable` is definitely stack-allocated and exactly the size of `some_type_`.
If defined with `;`, the instance can still be modified, but it will be sliced if some child
instance is copied to it.  To make this explicit (and thus prevent confusion), we require
that upcasting (going from child to parent) must be done *explicitly*.  For example:

TODO: classes should probably be allowed to be marked final.  e.g., `i64` and similar
fixed-width integers should be `final` so that we don't need to worry about vtables,
or specifying `@only i64`.  classes that are `final` would not need to be marked `@only`.

```
# extra field which will get sliced off when converting from
# `mythological_cat_` to `@only cat_`:
mythological_cat_: all_of_[cat_, m_: [lives; 9]]

cat; @only cat_
mythological_cat; mythological_cat_(lives; 7)

cat = mythological_cat          # COMPILER ERROR, implicit cast to `@only cat` not allowed.
cat = cat_(mythological_cat)    # OK.  explicit upcast is allowed.

other_cat; @only cat_
cat <-> mythological_cat    # COMPILER ERROR.  swaps not allowed unless both types are the same.
other_cat <-> cat           # OK.  both variables are `@only cat`.
```

One final note: abstract classes cannot be `@only` types for a variable, since they
are not functional without child classes overriding their abstract methods.

## template methods

You can define methods on your class that work for a variety of types.

```
some_example: [value: int_]
{   ;;renew_(int.): null_
        m value = int!

    ::to_(): ~t_
        t_(m value)
}

some_example: some_example_(5)

# you can explicitly ask for a type like this:
to_string: string_ = some_example to_()

# or like this:
[my_value: dbl_] = some_example to_()

# but you can't implicitly ask for the type.
unspecified: some_example to_()     # COMPILER ERROR, specify a type for `unspecified`
```

## generic/template classes

TODO: discuss how `null_` can be used as a type in most places.
But note that if you have a generic function defined like this,
we are already assuming some constraints:
```
my_generic_[of_](y: of_, z: of_): of_
    x: y * z
    x
```
If `of_` was nullable, then `x` would potentially be nullable, and should
be defined via `x?: y * z`.  But because oh-lang does template specialization
only after you supply the specific type you want, this can be caught at
compile time and only if you're requesting an invalid type.

To create a generic class, you put the expression `[types_...]` after the
class identifier, or we recommend `[of_]` for a single template type, where
`of_` is the [default name for a generic type](#default-named-generic-types).
For example, we use `my_single_generic_class_[of_]: [...]` for a single generic
or `my_multi_generic_class_[type1_, type2_]: [...]` for multiple generics.
To actually specify the types for the generic class, we use the syntax
`my_single_generic_class_[int_]` (for an `of_`-defined generic class) or
`my_multi_generic_class_[type1_: int_, type2_: str_]` (for a multi-type generic).
Note that any static/class methods defined on the class can be accessed
like this: `my_single_generic_class_[int_] my_class_function_(...)` or
`my_multi_generic_class_[type1_: int_, type2_: str_] other_class_function_()`.

```
generic_class_[id_, value_]: [id;, value;]
{   # this gives a method to construct the instance and infer types.
    # `g_` is like `m_` but without the template specialization, so
    # `g_` is `generic_class_` in this class body.
    g_(id. ~t_, value. ~u_): g_[id_: t_, value_: u_]
        [id, value]
}

# creating an instance using type inference.
# `id_` will be an `int_` and `value_` will be a `str_`.
class_instance: generic_class_(id. 5, value. "hello")
 
# creating an instance with template/generic types specified:
other_instance: generic_class_[id_: dbl_, value_: string_](id. 3, value. "4")
```

### default-named generics

If you have a generic class like `my_generic_[type1_, type2_]`, you can use them as a
default-named function argument like `my_generic[type1_, type2_]:`, which is short for
`my_generic: my_generic_[type1_, type2_]`.  This works even for generics over values,
e.g., if `fixed_array_[Count]` is a fixed-size array of size `Count`, then `fixed_array[3]:`
can be a declaration for a fixed array of size 3.  We can distinguish between 
`fixed_array[3]` being (1) this declaration or (2) a request to access the fourth
element in an array based on whether `fixed_array` is in scope, but adding a trailing `:`
helps point towards (1).

### generic class type mutability

It may be useful to create a generic class that whose specified type
can have writeable or readonly fields.  This can be done using `variable_name\` some_type_`
inside the generic class definition to define variables, and then specifying
the class with `[type1_: specified_readonly_type_, type2_; specified_writeable_type_]`.
TODO: this is going to be a bit difficult to get right with vim syntax;
can we use `~` instead with a space afterwards?

```
mutable_types_[x_, y_, z_]:
[   # these fields are always readonly:
    r_x: x_
    r_y: y_
    r_z: z_
    # these fields are always writeable:
    w_x; x_
    w_y; y_
    w_z; z_
    # these fields are readonly/writeable based on what is passed in
    # to `mutable_types_` for each of `x_`, `y_`, and `z_`, respectively.
    v_x` x_
    v_y` y_
    v_z` z_
]
{   # you can also use these in method/function definitions:
    ::some_method_(whatever_x` x_, whatever_y` y_): null
}

# the following specification will make `v_x` and `v_z` writeable
# and `v_y` readonly:
my_specification: mutable_types_[x_; int_, y_: string_, z_; dbl_]
```

We use a new syntax here because it would be confusing
to reinterpret a generic class declaration of a variable declared using `:`
as writeable in a specification with a `;`.

Note that if the generic class has no backticks inside, then it is a compile error
if you try to specify the generic class with a `;` type.  E.g., if we have the declaration
`generic_[a_]: [a;]`, then the specification `my_gen: generic_[a_; int_](5)` is a compile error.
If desired, we can switch to `generic_[a_]: [a\`]` to make the specification correct.

### virtual generic methods

You can also have virtual generic methods on generic classes, which is not allowed by C++.

```
generic_[of_]: [value; of_]
{   ::method_(~u): u_
        u + u_(u * m value) ?? panic_()
}

generic; generic_[str_]
generic value = "3"
print_(generic method_(2_i32))  # prints "35" via `2_i32 + i32_(2_i32 * "3")`

specific_[of_: number_]: all_of_[generic_[of_], m_: [scale; of_]]
{   ;;renew_(m scale. of_ = 1, generic value.): {}

    ::method_(~u): u_
        parent_result: generic method_(U)
        scale * parent_result
}

specific(value. 10_i8, scale. 2_i8)
print_(specific method_(0.5))   # should print "11.0" via `2 * (0.5 + dbl(0.5 * 10))`
```

Just like with function arguments, we can elide a generic field value if the
field name is already a type name in the current scope.  For example:

```
NAMESPACE_at_: int_
value_: [x: flt_, y: flt_]
my_lot; lot_[NAMESPACE_at_, value_]
# Equivalent to `my_lot; lot_[at_: NAMESPACE_at_, value_]`.
```

### generic type constraints

To constrain a generic type, use `[type_: constraints_, ...]`.  In this expression,
`constraints_` is simply another type like `non_null_` or `number_`, or even a combination
of classes like `all_of_[container_[id_, value_], number_]`.  It may be recommended for more
complicated type constraints to define the constraints like this:
`my_complicated_constraint_type_: all_of_[t1_, one_of_[t2_, t3_]]` and declaring the class as
`new_generic_[of_: my_complicated_constraint_type_]`, which might be a more readable way to do
things if `my_complicated_constraint_type_` is a helpful name.
TODO: `all_of_` is acting a little bit differently than a child class inheritor here,
do we need to distinguish between the two?  e.g., the child class usage of `all_of_`
will be ordered, but `all_of_` here in a type constraint should not require a certain order.

### generic type defaults

Type defaults follow the same pattern as type constraints but the default types are
not abstract.  So we use `[type_: default_type_, ...]` where `default_type_` is a class
that is non-abstract.

### overloading generic types

Note that we can overload generic types (e.g., `array_[int_]` and `array_[Count: 3, int_]`),
which is especially helpful for creating your own `hm_` result class based on the general
type `hm_[er_, ok_]`, like `MY_er_: one_of_[oops, my_bad], hm_[of_]: hm_[ok_: of_, MY_er_]`.
Here are some examples:

```
# Note that in oh-lang we could define this as `pair_[FIRST_of_, SECOND_of_]`
# so we don't need to specify `first_: int_, second_: dbl_`, but for illustration
# in the following examples we'll make the generic parameters named.
pair_[first_, second_]: [first;, second;]
pair_[of_]: pair_[first_: of_, second_: of_]

# examples using `pair_[of_]`: ======
# an array of pairs:
pair_array: array_[pair_[int_]]([[first. 1, second. 2], [first. 3, second. 4]])
# a pair of arrays:
pair_of_arrays: pair_[array_[int_]]([first. [1, 2], second. [3, 4]])

# examples using `pair_[first_, second_]`: ======
# an array of pairs:
pair_array: array_[pair_[first_: int_, second_: dbl_]]
(   [first. 1, second. 2.3]
    [first. 100, second. 0.5]
)
# a lot of pairs:
pair_lot: lot_[at_: str_, pair_[first_: int_, second_: dbl_]]
(   "hi there". [first. 1, second. 2.3]
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
    `type_case_` type and `Xyz` is the `variable_case` version of it.
    This is mostly useful for inheritance.

## singletons

Defining a singleton class is quite easy, simply by instantiating a class 
by using `variable_case` when defining it.

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

Using `@singleton type_case_` on the LHS defines an abstract singleton.
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
if it were an `variable_case` identifier.  E.g., `\\math` acts like one identifier, `Math`,
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
{   # Returns `null` if `At` is not in this container,
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
    # Returns null if not present.
    ;;[At]!?: of

    # safe setter.
    # returns an error if we ran out of memory trying to add the new value.
    ;;put(At, Of.): hm[null]

    # safe swapper.  replaces the value at `At` with the `Of` passed in,
    # and puts the previous value into `Of`.  the new or old value can
    # be null which means to delete what was there or that nothing was present.
    # returns an error if we ran out of memory trying to add the new value.
    ;;swap(At, Of?;): hm[null]
    
    @alias ::has(At): M[At] != null
    @alias ::contains(At): M[At] != null

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

    # Returns the value in the array if `Index < count()`, otherwise null.
    # If `Index < 0`, take from the end of the array, e.g., `Array[-1]` is the last element
    # and `Array[-2]` is the second-to-last element.  If `Index < -Array count()` then
    # we will also return null.
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
    # option 3: `X` syntax where `X` is a known variable, essentially equal to `@X:_x`
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
er_: one_of_
[   out_of_memory
    # etc...
]
hm_[of]_: hm_[ok_: of_, er_]

# TODO: is there a way we can make container dynamics better here for `unnest_`?
# e.g., can we make it a `container_[at_: of_, of_: of_]` as well?
set_[of_: hashable_]: container_[at_: of_, of_: true_]
{   # Returns `True` iff `Of` is in the set, otherwise null.
    # NOTE: the `true` type is only satisfied by the instance `True`;
    # this is not a boolean return value but can easily be converted to boolean.
    ::[Of]?: true

    # TODO: use `[]` for the unsafe API, `all()` or `put()` for the safe API (returning a `hm`)

    # Adds `Of` to the set and returns `True` if
    # `Of` was already in the set, otherwise `null`.
    # this can be an error in case of running out of memory.
    ;;[Of]: hm[true?]

    # Ejects `Of` if it was present in the set, returning `True` if true
    # and `null` if not.
    # A subsequent, immediate call to `::[Of]?` returns null.
    ;;[Of]!?: true 

    # Modifier for whether `Of` is in the set or not.
    # The current value is passed into the callback and can be modified;
    # if the value was `null` and is converted to `True` inside the function,
    # then the set will get `Of` added to itself.  Example:
    #   `Set[X] = if Condition {True} else {null}` becomes
    #   `Set[X, fn(Maybe True?;): {Maybe True = if Condition {True} else {null}}]`
    # TODO: if we used `True?` as the identifier everywhere we wouldn't need to do `Maybe True`, e.g.,
    #   `Set[X, fn(True?;): {True? = if Condition {True} else {null}}]`
    # TODO: remove these methods and add `refer` methods
    ;;[Of, fn(MAYBE_true?; true): ~t]: t

    # Fancy getter for whether `Of` is in the set or not.
    ::[Of, fn(MAYBE_true?): ~t]: t

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
        return null

    ::peak()?: if Next_value < Less_than
        Next_value 
    else
        null
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
assert(Iterator next(Array) == null)
assert(Iterator peak(Array) == null)
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
        null

    ::peak(Array[of])?: if Next < Array count()
        Array[Next]
    else
        null
    
    # note that this function doesn't technically need to modify this
    # `array_iterator`, but we keep it as `;;` since other container
    # iterators will generally need to update their index/ID.
    ;;remove(Array[of];)?: if Next < Array count()
        Array remove(Next)
    else
        null
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
    if Some_other_condition -> SOME_NAMESPACE_then:
        if Something_else1
            Then exit()
        if Something_else2
            SOME_NAMESPACE_then exit()
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
    #           OLD_value: Value
    #           Value = Value // 2 + 9
    #           # sequence should be: 0, 9, 4+9=13, 6+9=15, 7+9=16, 8+9=17
    #           if OLD_value == Value
    #               Block exit("exited at ${OLD_value}")
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
    Int each COUNT_int:
        sleep(Seconds: COUNT_int)
        Result += str(COUNT_int)
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
The syntax is `type_case_: one_of` followed by a list of named values
(each an `variable_case` identifier), with optional values they take, or subtypes
(each a `type_case_` identifier) with their corresponding type definitions.  Enumerations
are mutually exclusive -- no two values may be held simultaneously.  See
masks for a similar class type that allows multiple options at once.

Enums are by default the smallest standard integral type that holds all values,
but they can be signed types (in contrast to masks which are unsigned).
If desired, you can specify the underlying enum type using `i8_ one_of_[...]` instead
of `one_of_[...]`, but this will be a compile error if the type is not big enough to
handle all options.  It will not be a compile warning if the `one_of_` includes types
inside (e.g., `i8_ one_of_[u32_, f32_]`); we'll assume you want the tag to be an `i8_`.
However, it should be clear that the full type will be at least the size of the
tag plus the largest element in the `one_of_`; possibly more to achieve alignment.

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
the number of values that are enumerated via the class function `count_(): count_arch_`,
and the min and max values `min_(): enum_type_`, `max_(): enum_type_`.  You can also
check if an enum instance `enum` is a specific value `this_value` via
`enum is_this_value_()` which will return true iff so.

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
defaults to `B`, and `one_of[Option_c, null]` defaults to `null`, and
`one_of[A: -1, B: 0, C: 1, null]` also defaults to `null`.
TODO: `[null]` should collapse to `[]` based on how containers work.  Null is
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

TODO: i think we want to converge on `one_of_[dbl, int]`, so that we can do renaming
like this: `one_of_[value1: dbl_, value2: dbl_]`, in case we want to have two different
names that correspond to the same type.
TODO: what's the difference between `one_of[Dbl, Int]` and `one_of[dbl, int]`?
probably nothing??  but `one_of[New_identifier: 0, Other_identifier: 3]` would
be different than `one_of[new_identifier: 0, other_identifier: 3]`? or not??
in both cases, it seems like `0` and `3` are specifying the tag.  but would
`one_of[new_id: [X: dbl], other_id: [Y: str]]` be different than
`one_of[New_id: [X: dbl], Other_id: [Y: str]]`?...  maybe we just force lowercase.
TODO: discuss things like `one_of[1, 2, 5, 7]` in case you want only specific instances.

Nulls are
highly encouraged to come last in a `one_of`, because they will match any input, so
casting like this: `one_of[null, int](1234)` would actually become `null` rather than
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
# arguments short for `A_one_of: one_of_[int_, str_]` and `B_one_of: one_of_[u8_, i32_, dbl_]`.
my_function(A_one_of[int_, str_], B_one_of[u8_, i32_, dbl_]): dbl
    return dbl(A_one_of) * B_one_of
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

TODO: is there a way to make this `any_of` and use 0 as the `null` value?

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

* `Identifier`: starts with a non-numeric character, can have numerical characters after that.

* `function_case_`/`type_case_`: Identifier which ends with a trailing underscore.
    A leading underscore is allowed to indicate an unused function/type.

* `variable_case`: Identifier which does *not* end with a trailing underscore.
    A leading underscore is allowed to indicate an unused variable.

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
        null: {null}
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
