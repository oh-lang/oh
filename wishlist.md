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
In context, functions and types are followed by optional generics (in `{}` braces),
while functions alone have parentheses `()` or brackets `[]` with optional arguments inside.
Because types can act as functions, we don't syntactically distinguish between `type_case_`
and `function_case_` otherwise.

Another change is that oh-lang uses `:`, `;`, and `.` for declarations and `=` for reassignment,
so that declaring a variable and specifying a variable will work the same inside and outside
function arguments.  For example, declaring a function that takes a readonly integer named `x`,
`my_function_(x: int_): null_`, and declaring a constant integer variable named `x` uses the same
syntax:  `x: int_`.  Similarly, calling a function with arguments specified as `my_function_(x: 5)`
and defining a variable works the same outside of a function: `x: 5`.  In function arguments, e.g.
`(x: int_, y; str_, z. dbl_)`, we declare `x` as a readonly reference, `y` a writable reference,
and `z` a temporary, whereas outside of function arguments, `[x: int_, y; str_, z. dbl_]` indicates
that `x` is readonly (though it can be written in constructors or first assignment), that `y` is
writable, and `z` should be passed as a temporary (like most Rust variables).  I.e., it will be
hidden from scope if passed into a function that takes a temporary, unless you pass in a clone
(via `::_o()`).

In some languages, e.g., JavaScript, objects are passed by reference and primitives
are passed by value when calling a function with these arguments.  In oh-lang,
variables are passed by reference by default, for consistency.  I.e., on the left
hand side of an expression like `x = 5`, we know that we're using `x` as a reference,
and we extend that to function calls like `do_something_(x)`.  It is of course possible
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
`my_two_instance_function_(my_class_a:, my_class_b:, width: int_, height: int_)` which
can be called as `(my_class_a, my_class_b) my_two_instance_function_(width: 5, height: 10)`,
or by calling it as a method on either one of the instances, e.g.,
`my_class_a my_two_instance_function_(my_class_b, width: 5, height: 10)`, without needing to
define, e.g., `my_class_b::my_two_instance_function_(my_class_a:, width: 5, height: 10)` as well.

For convenience, `array{3} = 5` will work even if `array` is not already at least size 4;
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
elide the type name; e.g., `my_function_(int:): str_` will declare a function that takes 
a readonly instance of `int_`, i.e., `int:` expands to `int: int_`.  See
[default-named arguments](#default-name-arguments-in-functions).  This is also true if
[namespaces](#namespaces) are used, e.g., `my_function_(MY_NAMESPACE_int:): str_`.
We use `:` to create the readonly reference overload, e.g., `my_function_(int:): str_`
to create a function which takes a readonly integer reference, or `my_function_(int;): str_`
for a function that can mutate the passed-in integer reference or `my_function_(int.): str_`
for a function which takes a temporary integer.
This also works for generic classes like `my_generic_{of_:}` where `of_` is a template type;
`my_function_(my_generic{int_};)` is short for `my_function_(my_generic; my_generic_{int_})`.

When calling a function, we don't need to use `my_function_(x: x)` if we have a local
variable named `x` that shadows the function's argument named `x`.  We can just
call `my_function_(:x)` to specify the readonly reference overload (`x: x`),
`other_function_(;y)` to specify the writable reference overload (`y; y`), or
`tmp_function_(.z)` to specify a temporary overload `z. @hide z!`, which also
stops you from using `z` again in the rest of the block.  Using `tmp_function_(z!)`
also calls the temporary overload and would allow `z` to still be used afterwards,
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
but you'd need to do this in generics as well, in case you're asking for a `\`` generic.
e.g., `my_class_{of_:}: [of\`]` would be `my_class_{;dbl_}` for a class that can
modify the double and `my_class_{:dbl_}` for readonly.

Class methods technically take an argument for `m` everywhere, which is somewhat
equivalent to `this` in C++ or JavaScript or `self` in python, but instead of
declaring `the_method_(m:, x: int_): str_`, we can use `::the_method_(x: int_): str_`.
this parallels `my_class::the_method` in C++, but in oh-lang we can analogously use
`;;a_mutating_method_` for a method that can mutate `m`, i.e.,
`a_mutating_method_(m;, x: int_): str_` becomes `;;a_mutating_method_(x: int_): str_`,
or `..one_temporary_method_()` for a method on a temporary `m`, i.e.,
`one_temporary_method_(m.)`.  Inside an instance method definition, you can use `m`
to refer to the class instance, regardless of how it was defined.  Inside methods,
you must use `m the_variable_name` to refer to a class field `the_variable_name`
and `m my_method_name_()` to call another method; this is to help with overload
resolution and so that static variables/methods can be easily distinguished.
(C++ developers are encouraged to prefix class member variables with
`m_` because C++ is too permissive here with name resolution.)
Here is an example class.

```
vector3_: [x: dbl_, y: dbl_, z: dbl_]
{    ::length_(): dbl_
          sqrt_(m x * m x + m y * m y + m z * m z)
}
```

We support nested types without needing a `m_` prefix, and they are available anywhere
in the class body they are defined.  This is a minor inconsistency with instance fields
and method calls (which always require `m`) but those make overloads much easier to
reason about.  However, to prevent confusion, these nested types cannot shadow any global
types.  Here is an example with a generic type, where it would be convenient
to refer to another generic subtype if we already have the class.

```
# NOTE: we can use type definitions from later in the class body when
# declaring class member variables (e.g., `lot; lot_`):
my_generic_{at_:, of_:}: [lot;]
{    lot_: @only insertion_ordered_lot_{at_, of_}
     ...
}

# ERROR: `lot_` (without a `{}` spec) is shadowed inside of `my_generic_`:
# we should rename this type or the type inside `my_generic_`.
lot_: lot_{at_: int_, of_: str_}
```

After fixing the compile error in the example above, we can use `some_type: my_generic_{at_, of_} lot_`
to refer to the nested type, but can we also use `some_type: lot_{m_: my_generic_{at_, of_}}`.
We don't override `lot_{my_generic_{at_, of_}}` because a single type might be an override of `lot_{of_}`;
this isn't the case for `lot_` specifically but for other types like `array_` there are definitely overloads.
See [type manipulation](#type-manipulation) for more details.

Note this is actually ok, because we can distinguish overloads based on arguments.

```
vector2_: [x: dbl_, y: dbl_]
{    ::atan_(): dbl
          atan_(m x, m y)      # also ok: `\\math atan_(m x, m y)` could avoid the import below.
}
[atan_(x: dbl_, y: dbl_): dbl_]: \\math
```

Also in the spirit of conciseness, `o` can be used for an *o*ther instance of the same type,
and `g_` can be used for the current generic class (without the specification) while
`m_` always refers to the current class type (including the specification if generic).

```
vector3_{of_: number_}: [x; of_, y; of_, z; of_]
{    # `g_` is used for this generic class without the current specification,
     # in this case, `vector3_`.
     g_(value_0. ~value, value_1., value_2.): g_{value_}
          [x: value_0, y: value_1, z: value_2]

     ::dot_(o:): of_
          m x * o x + m y * o y + m z * o z
}

dot_(vector3_(1, 2, 3), vector3_(-6, 5, 4)) == 1 * -6 + 2 * 5 + 3 * 4
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
when errors can occur.  The `hm_{ok_, er_}` class handles this, with `ok_` being the
type of a valid result, and `er_` being the type of an error result.  You can specify
the types via `hm_{ok_: int_, er_: str_}` for `ok_` being `int_` and `er_` being a `str_`.
If the `ok_` and `er_` types are distinct, you don't need to wrap a return value in
`ok_(valid_result)` and `er_(error_result)`; you can just return `valid_result` or `error_result`.
See [the `hm` section](#hm) for more details.  It is a compile error to not handle
errors when they are returned (e.g., something like a `no-unused-result`), although
often there are overloads (without an `hm_` result being returned) which just panic
at runtime in case of an error.  oh-lang does make it easy to chain results using
[`assert_`](#assert), and we'll probably reserve an operator like `?!` to do this.

Another way we aim to be concise is by using the `_` as an [inferred type](#_-type).

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
default value for an error, all use an overloaded `map_` method, so there's no mental
overhead here.  Since overloads are not possible in Rust, there is an abundance of methods, e.g.,
[`Result::map_*` documentation](https://doc.rust-lang.org/std/result/enum.Result.html#method.map),
which can require constantly poring over documentation just to find the right one.  

We also don't use a different concept for interfaces and inheritance.
The equivalent of an interface in oh-lang is simply an abstract class.  This way
we don't need two different keywords to `extend` or `implement` a class or interface.
In fact, we don't use keywords at all; to just add methods (or class functions/variables),
we use this syntax, `wrapper_class_: parent_class_ { ::extra_methods_(): int_, ... }`,
and to add instance variables to the child class we use this notation:
`child_class_: all_of_{parent_class:, m: [child_x: int_, child_y: str_]} { ... methods }`.

oh-lang handles generics/templates in a way more similar to zig or python rather than C++ or Rust.
When compiled without any usage, templates are only tested for syntax/grammar correctness.
When templates are *used* in another piece of code, that's when the specification kicks in
and all expressions within the generic are compiled to see if they are allowed with the
specified types.  Any errors are still compile-time errors, but you get to have the simplicity
of duck typing without needing to specify your type constraints fully.

```
my_generic_{of_}(a: ~of_, b: of_): of_
     # this clearly requires `of_` to implement `*`
     # but we didn't need to specify `[of_: number_]` or similar in the generic template.
     a * b

print_(my_generic_(a: 3, b: 4))                 # OK
print_(my_generic_(a: [1, 2, 3], b: [4, 5]))    # COMPILE ERROR: no definition for `array_{int_} * array_{int_}`
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
custom_container_{of_:}: [@private vector{10, of_};]
{    # make an overload for `m[ordinal]` where `ordinal_` is a 1-based indexing type.
     :;[ordinal.]: hm_{ok_: (of:;), er_: str_}
          if ordinal > 10
               er_("index too high")
          else
               ok_((of:; m vector[ordinal]))

     @can_panic
     :;[ordinal.]: (of:;)
          m[ordinal] hm assert_()

     # for short, you can use this `@hm_or_panic` macro, which will essentially
     # inline the logic into both methods but panic on errors.
     :;[ordinal.]: @hm_or_panic_{ok_: (of:;), er_: str_}
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
`what hm {ok: {print_(ok)}, er: {print_("got error: $(er)")}}`.

For primitive types (like `dbl_` and `i32_`), we don't throw on overflow or underflow.
But we do for wrapper types like `count_`, `index_`, `offset_`, and `ordinal_`.

# general syntax

* `print_(...)` to echo some values (in ...) to stdout, `print_(error: ...)` to echo to stderr
     * use string interpolation for printing dynamic values: `print_("hello, $(variable_1)")`
     * use `print_(no_newline: "keep going ")` to print without a newline
     * default overload is to print to null, but you can request the string that was printed
          if you use the `print_(str.): str_` or `print_(error. str_): str_` overloads.
          e.g., `another_fn_(value: int_): print_("Value is $(value)")` will return `null`,
          whereas `another_fn_(value: int_): str_ {print_("Value is $(value)")}` will
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
     * use `fn_(): x_` or `fn_[]: x_` to declare `fn_` as a function returning an instance of type `x_`,
          see [functions](#functions), with any arguments inside `()` or `[]`, with the distinction being
          useful for [references](#references).  `fn_` can be renamed to anything `function_case_`,
          but `fn_` is one of the defaults.
     * use `new_{}: y_` to declare `new_` as a function returning *a type* `y_`, with any arguments inside `{}`.
          `new_` can be renamed to anything `type_case_`, but `new_` is the default.
          See [returning a type](#returning-a-type).
     * use `a_: y_` to declare `a_` as a constructor that builds instances of type `y_`
          with `a_` any `type_case_` identifier.  This is essentially a `typedef` and useful
          when `y_` is something complicated like a [generic specification](#defining-generic-classes).
     * while declaring *and defining* something, you can avoid the type if you want the compiler to infer it,
          e.g., `a: some_expression_()`
     * thus `:=` is usually equivalent to `:` (and similarly for `;=`), except in the case of defining
          a function via another function, i.e., function aliasing.  E.g.,
          `fn_(x: int_): str_ = other_fn_` will alias `other_fn_(x: int_): str_` to `fn_`, while
          `fn_(x: int_): return_type_` just declares a function that returns an instance of `return_type_`.
          Note that `fn_(x: int_): str_ = generate_fn_()` is the way to define `fn_` based on the
          function returned by calling `generate_fn_()` only once, which in this case should have
          an overload `generate_fn_(): fn_(x: int_): str_`
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
     * `"Interpolate $(not_printed_(), x)"` to create the string "Interpolate 123" when `x` is 123;
          note that `not_printed_()` is run but its results are not added to the string.
     * `f_(a: 3, b: "hi")` to call a function, and `f_(a: int_, b: str_): null_` to declare a function.
     * `a@ (x_(), y)` to call `a x_()` then `a y` with [sequence building](#sequence-building)
          and return them in a reference object with fields `x` and `y`, i.e., `(x: a x_(), y: a y)`.
          This allows `x` and `y` to be references.  This can be useful e.g., when `a` is an expression
          that you don't want to add a local variable for, e.g., `my_long_computation_()@ (x_(), Y)`.
* `[]` are for types and containers (including objects, arrays, and lots)
     * `[x: dbl_, y: dbl_]` to declare a plain-old-data class with two double-precision fields, `x` and `y`
     * `[x: 1.2, y: 3.4]` to instantiate a plain-old-data class with two double-precision fields, `x` and `y`
     * `[greeting: str_, times: int_] = destructure_me_()` to do destructuring of a return value
          see [destructuring](#destructuring).
     * `a@ [x_(), y]` to call `a x_()` then `a y` with [sequence building](#sequence-building)
          and return them in an object with fields `x` and `y`, i.e., `[x: a x_(), y: a y]`.
          You can also consider them as ordered, e.g.,
          `results: a@ [x_(), y], print_("$(results[0]), $(results[1]))`.
* `{}` for blocks and generics
     * `{...}` to effectively indent `...`, e.g., `if condition {do_thing_()} else {do_other_thing_(), 5}`
          * Used for defining a multi-statement function inline, e.g., `fn_(): {do_this_(), do_that_()}`.
               (Note that you can avoid `{}` if the block is one statement, like `fn_(): do_this_()`.)
          * Note that braces `{}` are optional if you actually go to the next line and indent,
               but they are recommended for long blocks.
     * `some_class_{n: number_, of_:}: some_other_class_{count: n, at_: int_, of_}` to define a class type
          `some_class` being related to `some_other_class_`, e.g., `some_class_{n: 3, str_}` would be
          `some_other_class_{count: 3, at_: int, of_: str_}`.
     * For generic/template classes, e.g., classes like `array_{count, of_}` for a fixed array of size
          `count` with elements of type `of_`, or `lot_{int_, at_: str_}` to create a map/dictionary
          of strings mapped to integers.  See [generic/template classes](#generictemplate-classes).
     * For generic/template functions with type constraints, e.g., `my_function_{of_: non_null_}(x: of_, y: int_): of_`
          where `of_` is the generic type.  See [generic/template functions](#generictemplate-functions) for more.
     * `a@ {x_(), y}` with [sequence building](#sequence-building), 
          calling `a x_()` and `a y`, returning `a` if it's a temporary otherwise `a y`
* `~` to infer or generalize a type
     * `my_generic_function_(value: ~u_): u_` to declare a function that takes a generic type `u_`
          and returns an instance of that type.  For more details, see
          [generic/template functions](#generictemplate-functions).
     * `my_result; array_{~} = do_stuff_()` is essentially equivalent to `my_result; do_stuff_() array`, i.e.,
          asking for the first array return-type overload.  This infers an inner type via `{~}` but doesn't name it.
     * `named_inner; array_{~infer_this_} = do_stuff_()` asks for the first array return-type overload,
          and defines the inner type so it can be used later in the same block, e.g.,
          `first_value; infer_this_ = named_inner[0]`.
          Any `type_case_` identifier can be used for `infer_this_`.
* `$` for inline block and lambda arguments
     * [inline blocks](#block-parentheses-and-commas) include:
          * `$(...)` as shorthand for `(fn_(): {...})`, i.e., defining a [lambda function](#lambda-functions)
               with `()` arguments specified by any lambda variables (see `$arg` logic).  E.g.,
               `my_array map_$(2 * $int + 1)` is equivalent to
               `my_array map_(fn_(int:): 2 * int + 1)`.
          * `$[...]` as shorthand for `[fn_[]: {...}]`, i.e., defining a lambda function with
               `[]` arguments specified by any lambda variables (see `$arg` logic).
               TODO: if we go further and do `fn_[]{[...]}` then this would still work:
               `array: if some_condition $[1, 2, 3] else $[4, 5]`
               but i'm not sure we want for consistency.  saving one character isn't that great
               compared to `array: if some_condition {[1, 2, 3]} else {[4, 5]}`.
          * `${...}` is shorthand for a `{new_{}: {...}}`, which can be used to create
               lambda functions for types, or [lambda types](#lambda-types) for short,
               with a similar function for lambda variables becoming arguments.
               TODO: is the wrapping `{}` ok in every situation?  or should we infer it only
               when we see a `type_case_` identifier before `${...}`?
     * `$arg` as shorthand for defining an argument in a [lambda function](#lambda-functions)
          * `my_array map_$($int * 2 + 1)` will iterate over e.g., `my_array: [1, 2, 3, 4]`
               as `[3, 5, 7, 9]`.  The `$` variables attach to the enclosing `$()` as
               function arguments, variables with `$$` would attach to the enclosing `$$()`, etc. 
* all arguments are specified by name so order doesn't matter, although you can have default-named arguments
  for the given type which will grab an argument with that type (e.g., `int` for an `int_` type).
     * `(x: dbl_, int:)` can be called with `(1234, x: 5.67)` or even `(y, x: 5.67)` if `y` is an `int_`
* variables that are already named after the correct argument can be used without `:`
     * `(x: dbl_, y: int_)` can be called with `(x, y)` if `x` and `y` are already defined in the scope,
          i.e., eliding duplicate entries like `(x: x, y: y)`.
* [Horstmann indentation](https://en.wikipedia.org/wiki/Indentation_style#Horstmann) to guide
     the eye when navigating multiline braces/brackets/parentheses
* operators that diverge from some common languages:
     * `**` and `^` for exponentiation
     * `&|` at the start of each text slice to create a multiline string.
     * `+|` at the start of each text slice to create a multiline string with newlines.
     * `<>` for bit flips on integer types (instead of `~`)
     * `><` for bitwise xor on integer types (instead of `^`)
* see [operator overloading](#operator-overloading) for how to overload operators.

TODO: we probably need a `@comptime` annotation for functions that can be called at compile time.
or maybe we make them fully `UPPER_CASE_` to make it clear.  but i'm not as big of a fan of that...

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
greeting: "hello, $(name)!"

# declaring a multiline string with spaces added between lines
long_text:
        &|This is an example of a long sentence which
        &|deserves to be split across "lines".  Spaces will be
        &|added as necessary between lines.
# this is the same as:
# `long_text: "This is an example of a long sentence which deserves to be split across \"lines\".  Spaces will be added as necessary between lines."

# declaring a multiline string with newlines:
important_items:
          +|Fridge
          +|Pancakes and syrup
          +|Cheese
# this is the same as `important_items: "Fridge\nPancakes and syrup\nCheese\n"`

# a single-line ampersanded string can be used to avoid lots of escapes:
avoid_escapes: &|This is not a 'line' "you know"
# this is equivalent to `avoid_escapes: "This is not a 'line' \"you know\""

# a single-line plussed string will include a newline at the end.
just_one_line: +|This is a 'line' "you know"
# this is equivalent to `just_one_line: "This is a 'line' \"you know\"\n"

# declaring a multiline string with interpolation
multiline_interpolation:
          +|Special delivery for $(name):
          +|You will receive $(important_items) and more.
# becomes "Special delivery for Barnabus\nYou will receive Fridge\nPancakes and syrup\nCheese\n and more."

# if you want to avoid string interpolation, e.g., because you need to include a literal
# "${", "$[", or "$(" in your string, you only need to escape one of the two characters.
# e.g., `$a` is never interpolated as the value of `a`.
print_("ok \${we want this literally as} $\[whatever] $ok")
# literally prints "ok ${we want this literally as} $[whatever] $ok"

# interpolation over multiple file lines.
# WARNING: this does not comply with Horstmann indenting,
# and it's hard to know what the indent should be on the second line.
evil_long_line: "this is going to be a long discussion, $(
          name), can you confirm your availability?"
# INSTEAD, use string concatenation, which automatically adds a space if necessary:
good_long_line: "this is going to be a long discussion,"
     &   "$(name), can you confirm your availability?"
best_long_line:
        &|this is going to be a long discussion,
        &|$(name), can you confirm your availability?

# you can also nest interpolation logic, although this isn't recommended:
nested_interpolation: "hello, $(if condition {name} else {'World$("!" * 5)'})!"
```

Notice that the `&` operator works on strings to add a space (if necessary)
between the two operands.  E.g., `'123' & '456'` becomes `'123 456'`.  It also
strips any trailing whitespace on the left operand and any leading whitespace
on the right operand to ensure things like `'123\n \n' & '\n456'` are still just `'123 456'`.
This makes it the perfect operator for string concatenation across lines where we want
to ensure a space between words on one line and the next.

## defining arrays

See [arrays](#arrays) for more information.

```
# declaring a readonly array
my_array: array_{element_type_}

# defining a writable array:
array_var; array_{int_}(1, 2, 3, 4)
# We can also infer types implicitly via one of the following:
#   * `array_var; array_(1, 2, 3, 4)`
#   * `array_var; [1, 2, 3, 4]`
array_var[5] = 5    # array_var == [1, 2, 3, 4, 0, 5]
++array_var[6]      # array_var == [1, 2, 3, 4, 0, 5, 1]
array_var[0] += 100 # array_var == [101, 2, 3, 4, 0, 5, 1]
array_var[1]!       # returns 2, defaults array_var[1]:
                    # array_var == [101, 0, 3, 4, 0, 5, 1]

# declaring a long array (note the Horstmann indent):
long_implicitly_typed:
[    4   # commas aren't needed here.
     5
     6
]

# declaring a long array with an explicit type:
long_explicitly_typed: array_{i32_}
(    5   # commas aren't needed here.
     6
     7
)

# because {} is equivalent to an indented block, these are also valid definitions
# equivalent definitions:
array; array_{int_} = [1, 2, 3]
array{int_}; [1, 2, 3]
array
     int_
;    [1, 2, 3]
# more equivalence:
array; array_{int_}(1, 2, 3)
array{int_}(1, 2, 3);
# TODO: does this work with lexer??
array
     int_
(    1
     2
     3
);
```

Note there are some special rules that allow line continuations for parentheses
as shown above.  See [line continuations](#line-continuations) for more details.

## defining lots

Lots are oh-lang's version of dictionaries or maps.
See [lots](#lots) for more information.

```
# declaring a readonly lot
my_lot: lot_{at: id_type_, value_type_}

# defining a writable lot:
votes_lot; lot_{at_: str_, int_}("Cake": 5, "Donuts": 10, "Cupcakes": 3)
# We can also infer types implicitly via one of the following:
#   * `votes_lot; lot_(["Cake": 5, ...])`
#   * `votes_lot; ["Cake": 5, ...]`
votes_lot["Cake"]        # 5
++votes_lot["Donuts"]    # 11
++votes_lot["Ice Cream"] # inserts "Ice Cream" with default value, then increments
votes_lot["Cupcakes"]!   # resets "Cupcakes" to 0 and returns 3
votes_lot::["Cupcakes"]  # null
# now `votes_lot == ["Cake": 5, "Donuts": 11, "Cupcakes": 0, "Ice Cream": 1]`
```

## defining sets

See [sets](#sets) for more details.
TODO: we may not want to create defaults for sets here using `set["asdf"]`

```
# declaring a readonly set
my_set: set_{element_type_}

# defining a writable set:
some_set; set_{str_}("friends", "family", "fatigue")
# We can also infer types implicitly via the following:
#   * `some_set; set_("friends", ...)`
some_set::["friends"]    # `true`, without changing the set.
some_set::["enemies"]    # null (falsey), without changing the set.
some_set["fatigue"]!     # removes "fatigue", returns `true` since it was present.
                         # `some_set == set_("friends", "family")`
some_set["spools"]       # adds "spools", returns null (wasn't in the set), but now is.
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
{    # NOTE! return fields `w` and `z` are in scope and can be assigned
     # directly in option A:
     # TODO: i don't know if this is consistent with our `vector1_: [x: dbl_]` logic
     #    we may want to allow not using `m` if the class is not defined with it
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
(    you_(): str_ = my_name_
     greet_(name: str_): str_
          "Hello, $(name)"
)

# case (C): defining a few functions inline without `{}`
hello_world_(): print_(do_something_(you_(): "world", greet_(name: str_): "Hello, $(name)"))
```

### defining generic functions

There are two ways to define a generic function: (1) via type inference `~x`
and (2) with an explicit generic specification `{types_...}` after the function name.
You can combine the two methods if you want to infer a type and specify a
condition that the type should satisfy, e.g., `fn_{x_: number_}(~x.): x_`,
where `~x.` expands to `x. ~x_`, meaning that `x_` is inferred, and the
braces `{x_: number_}` require `x_` to be a number type.  Any types that are
not inferred but are given in braces must be added at the callsite, e.g.,
`fn_{x_: number_, y_:}(~x., after: y_): y_` should be called like
`fn_{y_: int_}(123.4, after: 5)`.

Note that default names apply to either case; `~x:` is shorthand for `x: ~x_`
which would not need an argument name, and `fn_{value_:}(value.): null_` would
require `value_` specified in the braces but not in the argument list,
e.g., `fn_{value_: int_}(123)`.  In braces, the "default name" for a type is
`of_`, so you can call a function like `fn_{of_:}(of.): null_` as `fn_{int_}(123)`.

Some examples:

```
# this argument type is inferred, with a default name
fn_(~x.): x_
# call it like this:
fn_(512)

# this argument type is inferred but need to name it as `x: ...`
fn_(~NAMED_x:): null_
# call it like this:
fn_(x: 512)

# another way to infer an argument but require naming it as `x; ...`
fn_(x; ~t_): t_
# we call it like this:
fn_(x; 512)

# explicit generic with condition, not inferred:
fn_{x_: condition_or_parent_type_}(x.): x_
# call it like this, where `int_` should satisfy `condition_or_parent_type_`
fn_{x_: int_}(5)

# explicit generic with condition, inferred:
fn_{x_: condition_}(~x.): x_
# call it like this, where `dbl_` should satisfy `condition_`
fn_(3.14)

# explicit generic without a default name:
fn_{x_:}(value: x_): null_
# call it like this:
fn_{x_: str_}(value: "asdf")

# explicit default-named generic, but argument is not default named:
fn_{of_:}(value: of_): of_
# call it like this; you can omit `of_: ...` in braces:
fn_{int_}(value: 123)

# because braces are equivalent to indented blocks, these are also equivalent:
generic_fn_{of_: number_}(of.): of_
     print_(of)
     of *= 2
     of
# with blocks:
generic_fn_
     of_: number_
(    of.
): of_
     print_(of)
     of *= 2
     of

# calling with braces
generic_fn_{int_}(123)
# calling without braces, might be nice for a few specified generics but probably not one
generic_fn_
     int_
(    123
)
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
{    # here's a class function that's a constructor
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
          m x * y

     # methods which mutate the class use a `;;` prefix
     ;;update_(y: int_): null_
          m x = m do_something_(y)
}
```

Inside a class body, we use `m` to scope instance variables/functions
but we don't need to use `m_` to scope class variables/functions.

Inheritance of a concrete parent class and implementing an abstract class
work the same way, by specifying the parent class/interface in an `all_of_`
expression alongside any child instance variables, which should be tucked
inside an `m` field.

```
parent1_: [p1: str_]
{    ::do_p1_(): null_
          print_("doing p1 $(m p1)")
}

parent2_: [p2: str_]
{    ::do_p2_(): null_
          print_("doing p2 $(m p2)")
}

# TODO: we probably can allow for immutable parent types by doing `parent1:`
#    and mutable via `parent1;`; is there any point to doing that?
child3_: all_of_{parent1:, parent2:, m: [c3: int_]}
{    # this passes p1 to parent1 and c3 to child3 implicitly,
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
the struct will be started with fields in `a` for `all_of_{a:, b:, c:}`
and finish with fields in `c`; the child fields do not need to be first
(or last); they can be added as `a`, `b`, or `c`, of course as `m: [...]`.
Generally it's recommended to add child fields last.

### defining generic classes

With classes, generic types must be explicitly declared in braces.
Any conditions on the types can be specified via `{the_type: the_condition, ...}`.

```
# default-named generic
generic_{of_:}: [@private of;]
{    # you can use inference in functions, so you can use `generic_(12)`
     # to create an instance of `generic_` with `of_: int_` inferred.
     # You don't need this definition if `m of` is public.
     # NOTE: `g_` is like `m_` for generic classes but without the specification.
     g_(~t.): g_{t_}
          [of. t] 
}

generic{int_}(1):           # shorthand for `generic: generic_{int_}(1)`.
my_generic: generic_(1.23)  # infers `generic_{dbl_}` for this type.
WOW_generic("hi");          # shorthand for `WOW_generic; generic_("hi")`, infers `generic_{str_}`

# not default named:
entry_{at_: hashable_, of_: number_}: [at:, value; of_]
{    ;;add_(of): null_
          m value += of
}

# shorthand for `entry: entry_{at_: str_, of_: int_}(...)`:
entry{at_: str_, int_}(at: "cookies", value: 123):
my_entry; entry_(at: 123, value: 4.56)              # infers `at_: int_` and `of_: dbl_`.
my_entry add_(1.23)
my_entry value == 5.79

# because braces are equivalent to indented blocks, these are also equivalent:
generic_class_
{    a_: some_constraint_
     b_: another_constraint_
}:   [a;, b;]
{    ::do_something_(): one_of_{a:, b:}
          m a || m b
}
# defining the generics without braces:
generic_class_
     a_: some_constraint_
     b_: another_constraint_
:    [a;, b;]
{    ::do_something_(): one_of_{a:, b:}
          m a || m b
}
```

See [generic/template classes](#generictemplate-classes) for more information.

## identifiers

Identifiers in oh-lang are very important.  The trailing underscore (or lack thereof)
indicates whether the identifier is a function/type (or a variable), which gives some
space to guide the eye with function calls like `my_function_(x: 3)`.  Similarly for
type (or class) names, since types can work like functions (e.g., `int_(number_string)`).
Variable names like `x` and `max_array_count` do not include a trailing underscore.
Any capitalized letters belong to a [namespace](#namespaces).

There are a few reserved keywords, like `also`, `if`, `elif`, `else`, `with`, `return`,
`break`, `continue`, `what`, `in`, `each`, `for`, `while`, `pass`, `where`, `when`,
`is`, `has`,
which are function-like but may consume the rest of the statement.
`return` is a bit special in that it is used like a variable but will return
from the enclosing function;  e.g., `return: x + 5` will return the value `x + 5`.
The `type_case_` versions of these keywords are also all reserved;
for example, `return_` can be used for the return type of the current function
or it can be used as a function to actually return a value, e.g., `return_(x + 5)`
(i.e., but only if the value is not captured; `y: return_(x + 5)` would be a type cast).
There are some reserved namespaces with side effects like `NAMED_`, `AS_`,
which should be used for their side effects.  Variables that end in numbers like
`x_0` or `asdf_123` will also have the number considered as a namespace, which
can be used for binary operations like `+` and `*`.  See [namespaces](#namespaces).

There are some reserved variable names: `m` and `o`, along with their `type_case_`
variants, and `_` which is reserved as an [inferred type](#_-type).  `m` can only
be used as a reference to the current class instance, and `o` which
can only be used as a reference to an *o*ther instance of the same type;
`o` must be explicitly added as an argument, though, in contrast to `m` which can be implicit.
The corresponding types `m_`, and `o_` are reserved, `m_` for class bodies
(to indicate the current type) and `o_` as a method to *clone* (or copy)
the current instance, with function signature `::o_(): m_` or
`::o_(): hm_{ok_: m_, er: ...}` if cloning can fail (e.g., due to OOM).

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
     print_("do something eventually with the arg")

# when calling, omit the leading underscore:
my_function_(argument_which_we_will_need_later: 3)
```

### `_` type

By itself, `_` means to infer the type, usually from the left-hand-side of
an equation.  This is useful for [class imports](#modules), [enums](#enumerations),
and [masks](#masks), among other things.  Some examples:

For imports/modules we have, e.g., `\/my/implementation/vector2 _` being equivalent
to `vector2_` (`_` is combined with the base file name `vector2` to get `vector2_`).
For enums like `my_enum_: one_of_{cool:, neat:, sweet:}`, we can define them like
this: `my_enum: _ cool`, where `_` infers the type of `my_enum` on the LHS.
This works similarly for masks like `my_mask_: any_of_{world:, npc:, player:}`;
`my_mask: _ world | _ npc` will set `my_mask` to `my_mask_ world | my_mask_ npc`.

We also can use `_` when defining default-named variables to refer to the variable's
type, e.g., for static/class functions like `oh_info: _ caller_()` which is equivalent
to `oh_info: oh_info_ caller_()` or `my_type: _(abc. 123)` to initialize `my_type`
to `my_type(abc. 123)`.  

## blocks

### tabs vs. spaces

Blocks of code are made out of lines at the same indent level; an indent is 5 spaces.
No more than 7 levels of indent are allowed, e.g., lines at 0, 5, 10, 15, 20, 25, 30 spaces.
If you need more indents, use a helper method and refactor.

Note that five spaces are used so that we can determine line continuation intent with operators
that are three characters long, e.g., `&&=`; we'll assume something like this is a line continuation:

```
my_long_named_condition_we_will_need_a_continuation
     &&=  some_other_condition
```

See the next section for more information on line continuation logic.
While a single tab would cover any number of spaces and help determine continuation intent,
we don't use them because they don't copy-paste well from the internet.

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
     [    5
          6
          7
     ]

# note it's unnecessary because we also allow opening brackets
# to get attached to the previous line if the internals are indented.
my_array:   # better!
[    5
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
my_function_(int:): [x: int_, y: int_]
     do_something_(int)
     [x: 5 - int, y: 5 + int]

# this indents `[x, y]` (i.e., to split into a multi-line array),
# but note that we need `return` to avoid parsing as `do_something_(int)[x: ...]`.
my_function_(int): [x: int_, y: int_]
     do_something_(int)
     # TODO: maybe we forbid `fn_(...)[...]` to avoid this issue.
     #    we could require `fn_(...) whatever[...]` if `fn_` returns an indexable `whatever`
     #    alternatively we could require `array at_(3)` instead of `array[3]`.
     return:
     [    x: 5 - int
          y: 5 + int
     ]

# alternatively, you could add a comma between the two statements
# to ensure it doesn't parse as `do_something_(int)[x: ...]`:
my_function_(int): [x: int, y: int]
     do_something_(int),
     [    x: 5 - int
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
[    1    # we insert commas
     2    # between each newline
     3    # as long as the indent is the same.
     other_array    # here we don't insert a comma after `Other_array`
     [    3         # because the indent changes
     ]              # so we parse this as `other_array[3],`
     5    # and this gets a comma before it.
]

# this is inferred to be a `lot` with a string ID and a `one_of_{int:, str:}` value.
lot_variable;
[    "Some_value": 100
     "Other_value": "hi"
]
lot_variable["Some_other_value"] = if condition {543} else {"hello"}

# This is different than the `lot_variable` because it is an instance
# of a `[some_value: int, other_value: str]` plain-old-data type,
# which cannot have new fields added, even if it was mutable.
object_variable:
[    some_value: 100
     other_value: "hi"
]
```

Note that the close parenthesis must be at the same indent as the line of the open parenthesis.
The starting indent of the line is what matters, so a close parenthesis can be on the same
line as an open parenthesis.

```
some_value:
(         (20 + 45)
     *    continuing + the + line + at_plus_2_indent -
          (         nested * parentheses / are + ok
               -    too
          )
)

another_line_continuation_variable: can_optionally_start_up_here
     +    ok_to_not_have_a_previous_line_starting_at_plus_two_indent * 
          (         keep_going_if_you_like
               -    however_long
          ) + (70 - 30) * 3

# note that the formatter will take care of converting indents like this:
non_horstmann_indent: (
     20 + some_function_(45)
)
# into this:
non_horstmann_indent:   # FIXME: update name :)
(    20 + some_function_(45)
)
```

Note that line continuations must be at least +2 indent, but can be more if desired.
Unless there are parentheses involved, all indents for subsequent line continuations
should be the same.

```
example_plus_three_indent; some_type_
...
example_plus_three_indent
     =         hello
          +    world
          -    continuing
```

Arguments supplied to functions are similar to arrays/lots and only require +1 indent
if they are multiline.

```
if some_function_call_
(    x
     y: 3 + sin_(x)      # default given for y, can be given in terms of other arguments.
     available_digits:
     [    1
          3
          5
          7
          9
     ]
)
     do_something_()

defining_a_function_with_multiline_arguments_
(    times: int_
     greeting: string_
     name: string_("World")  # argument with a default
):      string_             # indent here is optional/aesthetic
     # "return" is optional for the last line of the block,
     # unless you're returning a multiline array/object.
     "$(greeting), $(name)! " * times

defining_a_function_with_multiline_return_values_
(    argument0: int_
):
[    value0: int_   # you may need to add comments because
     value1: str_   # the formatter may 1-line these otherwise
]
     do_something_(argument0)
     # here we can avoid the `return` since the internal
     # part of this object is not indented.
     [value0: argument0 + 3, value1: str_(argument0)]

# ALTERNATIVE: multiline return statement
defining_a_function_with_multiline_return_values_
(    argument0: int_
     argument1: str_
): [value0: int_, value1: str_]
     # this needs to `return` or `pass` since it looks like an indented block
     # otherwise, which would attach to the previous line like
     # `do_something_(argument0)[value0: ...]`
     # or you can add an end-of-line comment between the two lines.
     do_something_(argument0)
     return:
     [    value0: argument0 + 3
          value1: argument1 + str_(argument0)
     ]
     # if you are in a situation where you can't return -- e.g., inside
     # an if-block where you want to pass a value back without returning --
     # use `pass`.

defining_another_function_that_returns_a_generic_
(    argument0: str_
     argument1: int_
): some_generic_type_
{    type0_: int_
     type1_: str_
}
     do_something_(argument0)
     print_("got arguments $(argument0), $(argument1)")
     return: ...
```

Putting it all together in one big example:

```
some_line_continuation_example_variable:
          optional_expression_explicitly_at_plus_two_indent
     +    5 - some_function_
          (    some_expression
                    +    next_variable
                    -    can_keep_going
                    /    indefinitely
               r: 123.4
          )
```

### block parentheses and commas

You can use `{` ... `}` to define a block inline.  The braces block is grammatically
the same as a standard block, i.e., going to a new line and indenting to +1.
Braces are useful for short `if` statements that you want to inline, e.g.,
`if some_condition {do_something_()}`, or long blocks that you want to be able to
navigate quickly in text editors.

Similarly, note that commas are grammatically equivalent to a new line and tabbing to the
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
Multiline block braces are useful if you want to clearly delineate where your blocks
begin and end, which helps some editors navigate more quickly to the beginning/end of the block.

```
# multiline block braces via an optional `{`
if some_condition
{    print_("toggling shutdown")
     print_("waiting one more tick")
     print_("almost..."), print_("it's a bit weird to use comma statements here")
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
* `bool_`: can hold a true or false value
* `rune_`: a utf8 character, presumably held within an `i32`
* `u8_`: unsigned byte (can hold values from 0 to 255, inclusive)
* `u16_` : unsigned integer which can hold values from 0 to 65535, inclusive
* `u32_` : unsigned integer which can hold values from 0 to `2^32 - 1`, inclusive
* `u64_` : unsigned integer which can hold values from 0 to `2^64 - 1`, inclusive
* `uXYZ_` : unsigned integer which can hold values from 0 to `2^XYZ - 1`, inclusive,
     where `XYZ` is 128 to 512 in steps of 64, and generically we can use
     `unsigned_{bits: count_}: what bits {8 {u8_}, 16 {u16_}, 32 {u32_}, ..., else {disallowed_}}`
* `count_` : `u64_` under the hood, intended to be <= `i64_ max_() + 1` to indicate the amount of something.
* `index_` : signed integer, `i64_` under the hood.  for indexing arrays starting at 0, can be negative
     to indicate we're counting from the back of the array.
* `ordinal_` : `u64_` under the hood.  for indexing arrays starting at 1.

and similarly for `i8_` to `i512_`, using two's complement.  For example,
`i8_` runs from -128 to 127, inclusive, and `u8_(wrap: i8_(-1))` equals `255`.
The corresponding generic is `signed_{bits: count_}`.  We also define the
symmetric integers `s8_` to `s512_` using two's complement, but disallowing
the lowest negative value of the corresponding `i8_` to `i512_`, e.g.,
-128 for `s8_`.  This allows you to fit in a null type with no extra storage,
e.g., `s8_?` is exactly 8 bits, since it uses -128 for null.
(See [nullable classes](#nullable-classes) for more information.)
Symmetric integers are useful when you want to ensure that `-symmetric`
is actually the opposite sign of `symmetric`; `-i8_(-128)` is still `i8_(-128)`.
The corresponding generic for symmetric integers is `symmetric_{bits: count_}`.

Note that the `ordinal_` type behaves exactly like a number but can be used
to index arrays starting at 1.  E.g., `array[ordinal_(1)]` corresponds to `array[index_(0)]`
(which is equivalent to other numeric but non-index types, e.g., `array[0]`).
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
safe_cast: x int_()                 # Safe_cast is a result type (`hm_{ok_: int_, number_ er_}`)
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

Casting to a complex type, e.g., `one_of_{int:, str:}(some_value)` will pass through `some_value`
if it is an `int_` or a `str_`, otherwise try `int_(some_value)` if that is allowed, and finally
`str_(some_value)` if that is allowed.  If none of the above are allowed, the compiler will
throw an error.  Note that nullable types absorb errors in this way (and become null), so
`result?: int_(some_safe_cast)` will be null if the cast was invalid, or an `int_` if the
cast was successful.

To define a conversion from one class to another, you can define a global function
or a class method, like this:

```
scaled8_:
[    # the actual value held by a `scaled8` is `Scaled_value / Scale`.
     @private
     scaled_value: u8
]
{    # static/class-level variable:
     @private
     scale: 32_u8

     m_(flt): hm_{ok_: m_, er_: one_of_{negative:, too_big:}}
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
     ::to_(): hm_{ok_: int_, number_ er_}
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
u16_(scaled8): hm_{ok_: u16_, number_ er_}
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
x; one_of_{int:, dbl:} = 4
y; x_ = 4.56    # use the type of `x` to define a variable `y`.
```

Note that the `type_case_` version of the `variable_case` name does not have
any information about the instance, so `x` is `one_of_{int:, dbl:}` in the above
example and `y` is an instance of the same `one_of_{int:, dbl:}` type.  For other
ways to handle different types within a `one_of_`, [go here](#one_of_-with-data).

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
For example, the generic result class in oh-lang is `hm_{ok_, er_}`, which
encapsulates an ok value (`ok_`) or a non-nullable error (`er_`).  For your custom class you
may not want to specify `hm_{ok_: my_ok_type_, er_: my_class_er_}` all the time for your custom
error type `my_class_er_`, so you can define `hm_{of_:}: hm_{ok_: of_, er_: my_class_er_}` and
use e.g. `hm_{int_}` to return an integer or an error of type `my_class_er_`.  Shadowing variables is
invalid in oh-lang, but overloads are valid.  Note however that we disallow redefining
an overload, as that would be the equivalent of shadowing.

## type manipulation

Plain-old-data objects can be thought of as merging all fields in this way:
TODO: i think this needs to be separate from `tag`s, but see if we can combine
`field` and `tag` ideas.  or maybe use `TAG_` to indicate this is a built-in.

Each type has reflection properties like `fields_()` which returns an array
of all the `field_` types that are in an object.  E.g., `[a: int_, b: str_]`
has fields `field_{name: "a", of_: int_}` and `field_{name: "b", of_: str_}`.
TODO: we probably need a type-ID field.

Using [lambda types](#lambda-types), we can practice type manipulation:

```
# tautologies
object_
    ==  merge_{object_ fields_(), ${$field_}}
    ==  merge_{object_ fields_(), ${field_($field_ name, $field_ of_)}}
```

There are some nice ways to manipulate object types, like converting all
field values of an object into some other type:

```
object_ valued_{new_{of_:}: ~new_of_}
    ==  merge_{object_ fields_(), ${field_{$field_ name, new_{$field_ of_}}}}
```

For example, `object_ valued_${um_{$of_}}` is a way to convert `object_`
data into futures.

Here are some examples of changing the nested fields on an object
or a container, e.g., to convert an array or object to one containing futures.

```
# base case, needs specialization.
nest_{m_, new_{of_:}: ~n_}: disallowed_

# container specialization.
# e.g., `array_{int_} nest_${um_{$of_}} == array_{um_{int_}}`,
# or you can do `nest_{m_: array_{int_}, ${um_{$of_}}}` for the same effect.
nest_
{    c_: container_
     m_: ~c_{of_: ~nested_, ~at_:}
     new_{of_:}: ~n_
}: c_{of_: new_{nested_}, at_}
# TODO: would this work as well?
nest_
{    c_: container_
     m_: ~c_{of_: ~nesting_, ~at_:}
     new_{of_: nesting_}: ~nested_
}: c_{of_: nested_, at_}

# object specialization.
# e.g., `[x: int_, y: str_] nest_${hm_{ok_: $of_, er_: some_er_}}`
# to make `[x: hm_{ok_: int_, er_: some_er_}, y: hm_{ok_: str_, er_: some_er_}]`,
# or you can do `nest_{{hm_{ok_: $of_, er_: some_er_}}, m_: [x: int_, y: str_]}` for the same effect.
nest_{m_: object_, new_{of_:}: ~_n_}: merge_
{    m_ fields_()
     ${field_{$field_ name, new_{$field_ value_}}}
}
```

Here are some examples of unnesting fields on an object/future/result.

```
# base case, needs specialization
unnest_{of_:}: disallowed_

# container specialization
# e.g., `unnest_{array_{int_}} == int_`
unnest_{container_{of_: ~nested_, ~_at_:}}: nested_

# `set` needs its own specialization because it has interesting
# `container_` dynamics.  e.g., `unnest_{set_{str_}} == str_`.
unnest_{set_{~nested_:}}: nested_

# future specialization
# e.g., `unnest_{um_{str_}} == str_`.
unnest_{um_{~nested_:}}: nested_

# result specialization
# e.g., `unnest_{hm_{ok_: str_, er_: int_}} == str`.
unnest_{hm_{ok_: ~nested_, ~_er_:}}: _nested

# null specialization
# e.g., `unnest_{int_?:} == int`.
unnest_{~nested_?:}: nested_
```

Note that if we have a function that returns a type, we must use braces, e.g.,
`type_function_{...}: the_return_type_`, but we can use instances like booleans
or numbers inside of the braces (e.g., `array_{3, int_}` for a fixed size array type).
Conversely, if we have a function that returns an instance, we must use parentheses,
e.g., `the_function_(...): instance_type_`.  In either case, we can use a type as
an argument, e.g., `nullable_(of_): bool_` or `array3_{of_:}: array_{3, of_}`.
Type functions can be specialized in the manner shown above, but instance functions
cannot be.  TODO: would we want to support that at some point??

Here is some nullable type manipulation:

```
# the `null` type should not be considered nullable because there's
# nothing that can be unnulled, so ensure there's something not-null in a nullable.
#   nullable_(one_of_{dbl:, int:, str:}) == false
#   nullable_(one_of_{dbl:, int:}?) == true
#   nullable_(null_) == false
nullable_(of_:): of_ contains_(not_{null_}, null_)
# TODO: i think i like `!null_` for `not_{null_}`.
nullable_(of_:): of_ contains_(!null_, null_)

# examples
#   unnull_{int_} == int_
#   unnull_{int_?} == int_
#   unnull_{one_of_{array{int_}:, set{dbl_}:}?} == one_of_{array{int_}:, set{dbl_}:}
unnull_{of_:}: if nullable_(of_) {unnest_{of_}} else {of_}

# a definition without nullable, using template specialization:
unnull_{of_:}: of_
unnull_{~nested_?:}: nested_
```

# operators and precedence

TODO: add : , ; ?? postfix/prefix ?
TODO: add ... for dereferencing.  maybe we also allow it for spreading out an object into function arguments,
e.g., `my_function_(a: 3, b: 2, ...my_object)` will call `my_function_(a: 3, b: 4, c: 5)` if `my_object == [b: 4, c: 5]`.
TODO: i think i would like a `|>` "inline-tab" operator for indenting the next line, e.g.,
```
["a", "bc", "def"] each str. |> what str
     "a"
          print_("got a")
     "bc"
          print_("got bc")
     else
          print_("got something else: $(str)")

# equivalent to 
["a", "bc", "def"] each str.
     what str
          "a"
               print_("got a")
          "bc"
               print_("got bc")
          else
               print_("got something else: $(str)")
```
TODO: or we could make it just a "pipe" like operator, so `each str. |> what` would not need `str` after it.

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
|           |   `?`     | postfix nullable          | unary:  `a?`      |               |
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
     print_("something cool with $(x), $(y), and $(z)")

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
* `@what`, `@when`, `@also`
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
{    # this is a situation where you might like to use namespaces.
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

* `OH_` - used internally by oh-lang, do not use
* `_0` - for the first operand in a binary operation (where order matters)
* `_1` - for the second operand in a binary operation (where order matters)
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
{    ;;renew_(m x: int_, m y: dbl_): null_
          print_("x $(x) y $(y)")

     # this `::` prefix is shorthand for `multiply_(m:, ...): dbl_`:
     ::multiply_(z: dbl_): dbl_
          m x * m y * z
}
```


```
some_class_: [x: dbl_, y: dbl_, a; array_{str_}]
Some_class; some_class_(x: 1, y: 2.3, a: ["hello", "world"])
print_(some_class::a)       # prints ["hello", "world"] with a readonly reference overload
print_(some_class::a[1])    # prints "world"
print_(some_class a[1])     # also prints "world", using ` ` (member access)
some_class;;a[4] = "love"   # the fifth element is love.
some_class::a[7] = "oops"   # COMPILE ERROR, `::` means the array should be readonly.
some_class;;a[7] = "no problem"

nested_class; array_{some_class_}
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
get_median_slow_(array{int_}:): hm_{ok_: int_, er_: string_}
     if array count_() == 0
          return: er_("no elements in array, can't get median.")
     # make a copy of the array, but no longer allow access to it (via `@hide`):
     SORTED_array: @hide array sort_()   # same as `array::sort_()` since `array` is readonly.
     ok(SORTED_array[SORTED_array count_() // 2])

# sorts the array and returns the median.
get_median_slow_(array{int_};): hm_{ok_: int_, er_: string_}
     if array count_() == 0
          return: er_("no elements in array, can't get median.")
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
binds less strongly than function calls and member access.  So something like
`a[b]^2` is equivalent to `(a[b])^2`, `fn_(x)^b` is equivalent to `(fn_(x))^b`,
and `a b^3` is equivalent to `(a::b)^3`.

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

The `or` operation `x or y` has type `one_of_{x:, y:}` (for `x: x_` and `y: y_`).
If `x` evaluates to truthy (i.e., `!!x == true`), then the return value of `x or y` will be `x`.
Otherwise, the return value will be `y`.  Note in a conditional, e.g., `if x or y`, we'll always
cast to boolean implicitly (i.e., `if bool_(x or y)` explicitly).

Similarly, the `and` operation `x and y` also has type `one_of_{x:, y:}`.  If `x` is falsey,
then the return value will be `x`.  If `x` is truthy, the return value will be `y`.
Again, in a conditional, we'll cast `x and y` to a boolean.

If the LHS of the expression can take a nullable, then there is a slight modification.
`x or y` will be `one_of_{x:, y:}?` and `x and y` will be `y_?`.
The result will be `null` if both (either) operands are falsey for `or` (`and`).

```
non_null_or: x or y         # non_null_or: if x {x} else {y}
non_null_and: x and y       # non_null_and: if !x {x} else {y}
nullable_or?: x or y        # nullable_or?: if x {x} elif y {y} else {null}
nullable_and?: x and y      # nullable_and?: if !!x and !!y {null} else {y}
```

This makes things similar to the `xor` operator, but `xor` always requires a nullable LHS.
The exclusive-or operation `x xor y` has type `one_of_{x:, y:}?`, and will return `null`
if both `x` and `y` are truthy or if they are both falsey.  If just one of the operands
is truthy, the result will be the truthy operand.  An example implementation:

```
xor_(~x, ~y)?: one_of_{x:, y:}
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
symbol, `?`, placed just after the variable name like `x?: int_`, and `?` is required
in order to give **null visibility** to any variable that could be nullable.  Even when
defining a variable without a type, the `?` symbol is required if the variable could be
nullable, e.g., `x?: nullable_result_(...)`, again for null visibility.  The default
value for a nullable type is `null`.  For generics where `null_` might be a valid value
for a template type, make sure to only include your nullable variable if necessary, e.g.,
`x{require: template_type_ is not_{null_}}: of_`.

One of the cool features of oh-lang is that we don't require the programmer to check for
null on a nullable type before using it.  The executable will automatically check for null
on variables that can be null.  This is also helpful for method chaining on classes (see 
more on those below).  If your code calls a method on an instance that is null, a null
will be returned instead (and the method will not be called).  But because of null visibility,
this will be clear to the programmer.

```
# define a class with a method called `some_method_`:
some_class_: []{ ::some_method_(): int }

nullable?; some_class_ = null

value?: nullable some_method_()    # `value` has type `int_?` now,
                                   # so it needs to be defined with `?`

# use `is` coercion to determine if a class is not null.
if nullable is some_class:
     non_null_value: some_class some_method_()   # `non_null_value` here must be `int_`.
```

See the [`is` operator](#is-operator) for more details.

It is not allowed to implicitly cast from a nullable type to a non-nullable type,
e.g., `value: nullable some_method_()`.  The compiler will require that we define
`value` with `?:`, or that we explicitly cast via whatever ending type we desire,
e.g., `value: int_(nullable some_method_()?)`.  Note that `whatever_type_(null)` is
the same as `whatever_type_()`, i.e., the default constructor, and number types
(e.g., `int_()` or `flt_()`)  default to 0, but we still need to acknowledge as
the programmer that the argument could be null (for null visibility), so we either
need `value: int_(from?: nullable some_method_()?)` or perhaps more idiomatically,
`value: nullable some_method_() ?? 0`.

[Nullable functions](#nullable-functions)) do not exist per se, but can be constructed
via a data wrapper.

## nullable classes

We will allow defining a nullable type by taking a type and specifying what value
is null on it.  For example, the symmetric type `s8_` defines null as `-128` like this:

```
s8_?: s8_
{    null: -128_i8
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
{    ;;renew_(m some_state: int_): {}

     ::normal_method_(): int_
          m some_state + 3

     # the nullable definition, inside a class:
     ?: m_
     {    null: [some_state: -1]
          ::is_null_(): m some_state < 0

          ::additional_null_method_(): int_
               if m is_null_() {0}
               else {m some_state * 5}
     }
}

# nullable definition, outside a class (but same file).
# both internal/external definitions aren't required of course.
my_class_?: my_class_
{    null: [some_state: -1]
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
nullish_or_(~a_0?., a_1.): a_
     what a_0
          non_null: {non_null}
          null {a_1}

# boolean or.
# `nullable || x` to return `x` if `nullable` is null or falsey,
# otherwise the non-null truthy value in `nullable`.
or_(~a_0?., a_1.): a_
     what a_0
          non_null:
               if non_null
                    non_null
               else
                    a_1
          null {a_1}
```

We'll support more complicated pattern matching (like in Rust) using
the `where` operator.  The shorter version of the above `what` statement is:

```
or_(~a_0?., a_1.): a_
     what a_0
          NON_NULL_a: where !!NON_NULL_a
               NON_NULL_a
          null
               a_1
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
vector2: [x: dbl_, y: dbl_] = [x: 3.75, y: 3.25]
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
     print_("x = $(x), y = $(y), atan_(y, x) = $(\\math atan_(x, y))")

# Note that it is also ok to use parentheses around a function definition,
# but you should use braces `{}`.
excite_(times: int_): str_
{    "hi!" * times
}

# You can define a multi-statement function in one line like this,
# but this is not normally recommended.
oh_(really; dbl_): dbl_ { really *= 2.5, return: 50 + really }
```

Note that we disallow the inverted syntax of `function_name_: return_type_(...args)`
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
(my_ref; int_) = my_value
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
(my_ref) = my_value2
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
(    from: 
     (    SOURCE_pixels
          size + vector2_(x: 3, y: 4)
     )
     to:
     (    ;DESTINATION_pixels
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
{    nested_referent: int_ = 5
     # COMPILE ERROR: `nested_referent` doesn't live as long as `my_reference`
     (my_reference) = nested_referent
}
```

However, since function arguments can be references (e.g., if they are defined with
`:` or `;`), references that use these function arguments can escape the function block.

```
fifth_element_(array{int_};): (int;)
     # this is OK because `array` is a mutable reference
     # to an array that already exists outside of this scope.
     # NOTE: this actually returns a pointer to the array with an offset (i.e., 4)
     #       in case the array reallocates, etc.
     (;array[4])

my_array; array_{int_}(1, 2, 3, 4, 5, 6)
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

# here we can elide the `refer_` that is inside the method
# `array_{int_};;[index]: (int;)`
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
there are actually a few options: `a_`, `an_`, `fn_`, and `do_`.
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
(    name_it_what_you_want_(): true
)   # should print "function returned true!"

# or you can create a default-named function yourself:
q_
(    fn_(): bool_
          random_() > 0.5
)   # will print one of the above due to randomness.
# equivalent to `q_(fn_(): random_() > 0.5)` or `q_({random_() > 0.5})`

# defining a lambda usually requires a name, feel free to use a default:
q_(do_(): true)
# or you can use this notation, without the name:
q_({true})

# or you can do multiline:
x; bool_
q_
(    fn_():
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
     return: 1234 + 5

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
     return: "!" * not_value

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
               return: CHECK_int
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
     return: "hello, world" + "!" * int
detect_(greet_(int): string_ = say_hi_) # returns 1

# you can also create a function named correctly inline -- the function
# will not be available outside, after this call (it's scoped to the function arguments).
detect_
(    greet_(int): string_
          "hello, world!!!!" substring_(length: int)
)   # returns 13

detect_(greet_(int): {["hi", "hey", hello"][int % 3] + ", world!"}) # returns 2
```

### lambda functions

Lambda functions are good candidates for [functions as arguments](#functions-as-arguments),
since they are very concise ways to define a function.  They utilize a parenthetical with a
number of `$` like `$(...function-body...)` with function arguments defined inside using
`$the_argument_name` (using the corresponding number of `$` as the brace).  There is no way
to specify the type of a lambda function argument, so the compiler must be able to infer it
(e.g., via using the lambda function as an argument, or by using a default name like `$int`
to define an integer).  Some examples:

```
run_asdf_(do_(j: int_, k: str_, l: dbl_): null_): null_
     print_(do_(j: 5, k: "hay", l: 3.14))

# Note that `$k`, `$j`, and `$l` attach to the same lambda based on looking
# for the matching `$()`.  `$()` also automatically gets a wrapping `()`.
run_asdf_$($k * $j + str_($l))     # prints "hayhayhayhayhay3.14"
```

If you need a lambda function inside a lambda function, use more `$` to escape
the arguments into the brace with the same number of `$`, e.g.,

```
# with function signatures
# `run_(fn_(x: any_): any_): any_` and
# `run_nested_(fn_(y: any_): any_): any_`
run_$($x + run_nested_$$($$y + $x))

# which is equivalent to the arguably the more readable:
run_
(    OUTER_fn_(x: any_):
          x + run_nested_
          (    INNER_fn_(y: any_):
                    y + x
          )
)
```

Again, it is likely more readable to just define the functions normally in this instance
rather than nest `$$()`.

There is currently no good way to define the name of a lambda function; we may use
`@named(whatever_name_) {$x + $y}`, but it's probably more readable to just define
the function inline as `whatever_name_(x:, y:): x + y`.

### types as arguments

Generally speaking you can use generic/template programming for this case,
which infers the types based on instances of the type.

```
# generic function taking an instance of `x_` and returning one.
do_something_(~x): x_
     return: x * 2

do_something_(123)    # returns 246
do_something_(0.75)   # returns 1.5
```
See [generic/template functions](#generictemplate-functions) for more details
on the syntax.

However, there are use cases where we might actually want to pass in
the type of something.  We can use `of_` as a type name to get default naming.
```
# `whatever_constraints_` can be something like `number_`,
# or you can elide it if you want no constraints.
do_something_(of_: whatever_constraints_): of_
     return: of_(123)

print_(do_something_(dbl_)) # returns 123.0
print_(do_something_(u8_))  # returns u8_(123)
```

Or we could do this as a a generic type, like this:
```
do_something_(~x_): x_
     return: x_(123)

print_(do_something_(dbl_)) # returns 123.0
print_(do_something_(u8_))  # returns u8(123)
```

### returning a type

We use a different syntax for functions that return types; namely `()` becomes `{}`,
e.g., `type_fn_{args...}: the_type_`.  This is because we do not need to support
functions that return instances *or* constructors, and it becomes clearer that we're
dealing with a type if we use a different enclosure.  We can also pass in comptime
constants into generics like `{count: count_}`.  Because braces are the gramatically
equivalent to indented blocks, the following are equivalent:

```
my_optional_{of_:}: one_of_{none:, some: of_}

# TODO: not 100% sure the lexer will like this, but let's try to figure it out:
my_optional_
     of_:
:    one_of_{none:, some: of_}

my_optional_{of_:}: one_of_
     none:
     some: of_
```

The brace syntax is related to [template classes](#generictemplate-classes) and
[overloading generic types](#overloading-generic-types).

To return multiple types, you can use the [type tuple syntax](#type-tuples).

TODO: 
it seems like we should be able to overload `fn_` for a `[]` argument list or a `()` argument list,
and that we should be able to return a type or a non-type either way.  but we should have a way
of indicating when a function is compile-time constant, so we can do `object_ fields_()`.
we're already doing this implicitly with enums, e.g., `abc_: one_of_{a:, b:, c:}`, then
`abc_ count_()`.  but again we'll need to do some built-in like `COUNT_(abc_)` in case
we have something like `w_: one_of_{count:, ...}`.  we should also investigate how to
overload `[]` for array indexing, because theoretically we can have `array[1, 2, 3]`.

### lambda types

TODO: discuss lambda functions for types, i.e., `${the_type_}`.

### unique argument names

Arguments must have unique names; e.g., you must not declare a function with two arguments
that have the same name.  This is because we wouldn't be able to distinguish between
the two arguments inside the function body.

```
# COMPILER ERROR.  duplicate identifiers
my_fun_(x: int_, x: dbl_): one_of_{int:, dbl:}
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
{    ;;renew_(m x. dbl_, m y. dbl_): {}

     # this is required to create vectors like this: `vector2_(1.0, 2.0)`
     # since we are explicit about `_0` and `_1` we don't need the
     # `@order_dependent` annotation.
     m_(dbl_0., dbl_1.): m_
          m_(x. dbl_0, y. dbl_1)

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
namespaces like `_0` and `_1`.  `m` is assumed to be `vector2_0` in the
above example, but if you use `o` it will be assumed to be `vector2_1`.

As mentioned earlier, we can have order dependence in certain established cases, but these
should be avoided in oh-lang as much as possible, where we prefer unique names.
One example is the cross product of two vectors, where order matters but the
names of the vectors don't.  (The relationship between the two orders is also
somewhat trivial, `a cross_(b) == -b cross_(a)`, and this simplicity should be aspired to.)

```
vector3_: [x; dbl_, y; dbl_, z; dbl_]
{    ;;renew_(m x. dbl_, m y. dbl_, m z. dbl_): {}

     # defined in the class body, we do it like this:
     ::cross_(o:): m_
     (    x. m y * o z - m z * o y
          y. m z * o x - m x * o z
          z. m x * o y - m y * o x
     )
}

# defined outside the class body, we do it like this:
# NOTE: both definitions are *not* required, only one.
cross_(vector3_0:, vector3_1:): vector3_
(    x: vector3_0 y * vector3_1 z - vector3_0 z * vector3_1 y
     y: vector3_0 z * vector3_1 x - vector3_0 x * vector3_1 z
     z: vector3_0 x * vector3_1 y - vector3_0 y * vector3_1 x
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
     print_("Hello, $(string)!")

greet_(say: string_, to: string_): null_
     print_("$(say), $(to)!")

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
     print_("$(say), world...")

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
     "hi $(y)"
```

The behavior that we get when we call `overloaded_` will depend on whether we
pass in a `y` or not.  But if we pass in a null `y`, then we also will end up
calling the overload that defined the missing argument case.  I.e.:

```
y?; int = ... # `y` is maybe null, maybe non-null

# the following calls `overloaded_()` if `y` is null, otherwise `overloaded_(y)`:
z: overloaded_(y?) # also OK, but not idiomatic: `z: overloaded_(y?: y)`
# `z` has type `one_of_{dbl:, string:}` due to the different return types of the overloads.
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
some_function_(x?: some_nullish_function_()) # good
some_function_(x: some_nullish_function_())  # ERROR! `some_nullish_function_` is nullable

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

# either way, `x` has type `string_?`.
```

You can use prefix `?` with multiple arguments; if any argument with prefix `?` is null,
then the function will not be called.

This can also be used with the `return` keyword to only return if the value is not null.

```
do_something_(x?: int_): int_
     y?: ?x * 3     # `y` is null or `x*3` if `x` is not null.
     return_(?y)    # only returns if `y` is not null
     # `return: ?y` is equivalent.
     #( do some other stuff )#
     ...
     return_(3)
```

### nullable output arguments

We also support function overloads for *outputs* that are nullable.  Just like with overloads
for nullable input arguments, there are some restrictions on defining overloads with (1) a
missing output, (2) a present output, and (3) a nullable output.  The restriction is a bit
different here, in that we cannot define (1) and (3) simultaneously for nullable outputs.
This enables us to distinguish between, e.g., `x?: my_overload_(y)` and `x: my_overload_(y)`,
which defines a nullable `x` or a non-null `x`.

If cases (1) and (2) are defined, then we should never ask for `x?: my_overload_(y)`; we
should only ask for `x: my_overload_(y)` or `my_overload_(y)` (which requests the null output
because it is not moved into a variable).

TODO: discussion on `fn_(): [x?: int_]` differences from `fn_()?: [x: int_]`.

```
# case 1, missing output (not compatible with case 3):
my_overload_(y: str_): null_
     print_(y)

# case 2, present output:
my_overload_(y: str_): [x: int_]
     [x: int_(y) ?? panic_("should be an integer")]

# case 3, nullable output (not compatible with case 1):
my_overload_(y: str_)?: [x: int_]
     what int_(y)
          ok: {[x: ok]}
          er: {null}

my_overload_(y: "9999999")    # calls (1) if it's defined, otherwise it's a compile error
x: my_overload_(y: "1234")    # calls (2) if it's defined, otherwise it's a compiler error.
x?: my_overload_(y: "abc")    # calls (3) if it's defined, otherwise it's a compiler error.
```

TODO: can we make `assert_` shorter?  Rust is nice with `?`, but we use that for
nullable stuff.  we should consider being nice.  Maybe `?!`.

Note that if only Case 3 is defined, we can use `assert_`s to ensure that the return
value is not null, e.g., `x: my_overload_() assert_()`.  This will throw a run-time
error if the return value for `x` is null.  Note that this syntax is invalid if Case 2
is defined, since there is no need to assert a non-null return value in that case.
This will also work for an overload which returns a result `hm`.

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
`;;renew_(o:): hm_{ok_: null_, er_: ...}` or `::o_(): hm_{ok_: m_, er_: ...}`
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
use `respectively_{a:, b:, c:}` to get type `a_`  for `:`, `b_` for `;`, and `c_` for `.`.
Similarly for the const template `:;`, `respectively_{a:, b:}` will give `a_` for `:` and
`b_` when `;`.

```
my_class_{of_:}: [x; of_]
{    ;;take_(of.):
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
my_class_{of_:}: [x; of_]
{    # these are added automatically by the compiler since `x; of_` is defined.
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
array{int_}; [0, 1, 2, 3, 4]
my_function_(;array[3]) # passed as writable reference
my_function_(:array[3]) # passed as readonly reference
my_function_(array[3])  # passed as writable reference since `array` is mutable.
```

You can switch to passing by value by using `.` or making an explicit copy:

```
array{int_}; [0, 1, 2, 3, 4]
my_function_(array[3] o_())    # passed by value (e.g., `my_function_(int.)`):
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
# saw_off_branch_(;array[3]):
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
     result += " $(animal speak_())"
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
     print_("int before $(int)")
     my_int += int
     print_("int middle $(int)")
     my_int += int
     print_("int after $(int)")
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
# TODO: i think i prefer `[round_down:] = fraction(...)`
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
countdown_(count:): all_of_{iterator{count_}}:, m: [count;]}
{    ::next_()?: count_
          if m count > 0
               --m count
          else
               null
}

my_array: array_{count_} = countdown_(5)
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

# TODO: i think i prefer `[chaos:] = patterns_()` unless `chaos` is pre-defined, then `[chaos] = patterns_()` is ok.
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

Variadic functions are possible in oh-lang using the `arguments_{of_}` class.
We recommend only one class type, e.g., `arguments_{int_}` for a variable number
of integers, but you can allow multiple classes via e.g. `arguments_{one_of_{x:, y:, z:}}`
for classes `x_`, `y_`, and `z_`.  oh-lang disallows mixing `arguments_` with any
other arguments.  The `arguments_` class has methods like `::count_()`, `;:[index.]: (of;:)`,
and `;;[index]!`.  Thus, `arguments_` is effectively a fixed-length array, but you
can modify the contents if you pass as `arguments{type_};`.  It is guaranteed that
there is at least one argument, so `arguments[0]` is always defined.

```
max_(arguments{int_}:): int_
     max; arguments[0] o_()
     range_(1, arguments count_()) each index:
          if arguments[index] > max
               max = arguments[index] o_()
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
[    input; lot_{at_: str_, any_}
     output; lot_{at_: str_, any_}
     # things printed to stdout via `print_`:
     prints; array_{str_}
     # things printed to stderr via `print_(error)`:
     errors; array_{str_}
]
{    # adds a named argument to the function call.
     # e.g., `call input_(at. "Cave", "Story")`
     ;;input_(at. str_, any.): null_
          m input[at] = any

     ;;input_(any.): null_
          m input_(at. any type_id to_(), any)

     ;;input_(at. str_, any:;): null_
          m input[at] = (:;any)

     # adds a named field to the return type with a default value.
     # e.g., `call output_(at. "field_name", 123)` will ensure
     # `[field_name]` is defined in the return value, with a
     # default of 123 if `field_name` is not set in the function.
     ;;output_(at. str_, any.): null_
          m output[at] = any

     # adds a default-named return type, with a default value.
     ;;output_(any.): null_
          m output_(at. any type_id to_(), any)
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
some_function_(;call) assert_()    # returns error since there are
                                   # no overloads with [value1, value2]
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
     print_("Hello, $(noun)!")

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

oh-lang is not a fan of nullable functions; due to other language choices 
(specifically function overloading) they look super messy and difficult to
(re)set to null.  If you need a nullable function we recommend that yoou
wrap it in a nullable class, e.g.,

```
# a wrapper type that has a non-null function inside it
escape_handler_: [do_something_(my_class;): null_]

# a class that now effectively has a nullable function
my_class_: [escape_handler?;, ...]
{    ;;escape_(): null_
          # will automatically check for null:
          m escape_handler do_something_(m)

          # or you can manually check:
          if m escape_handler is not_null:
               not_null do_something_(m)
          else
               print_("was null")

     ;;handle_escape(): null_
          m escape_handler =
          [    do_something_(m;):
                    print_("escaping!")

                    # setting to null is easy
                    m escape_handler = null
          ]
}
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

You can also add the new types in braces just after the function name,
e.g., `copy_{t_: my_type_constraints_}(value: ~t_): t_`, which allows you to
specify any type constraints (`my_type_constraints_` being optional).  Note
that types prefixed with `~` anywhere in the expression are inferred and
therefore can never be explicitly given inside of the braces, e.g.,
`copy_{t_: int_}(value: 3)` is invalid here, but `copy_(value: 3)` is fine.

If you want to require explicitly providing the type in braces, don't use `~` when
defining the function.

```
# this generic function does not infer any types because it doesn't use `~`.
copy_{the_type_:}(value: the_type_): the_type_
     ...
     the_type_(value)

# therefore we need to specify the generics in braces before calling the function.
copy_{the_type_: int_}(value: 1234) # returns 1234
```

For this example, it would probably be better to use `of_` instead of `the_type_`,
since `of_` is the "default name" for a generic type.  E.g., you don't need
to specify `{of_: int_}` to specialize to `int_`, you can just use `{int_}`
for an `{of_:}`-defined generic.  See also
[default named generic types](#default-named-generic-types).  For example:

```
# this generic function does not infer any types because it doesn't use `~`.
copy_{of_:}(value: of_): of_
     ...
     of_(value)

# because the type is not inferred, you always need to specify it in braces.
# you can use `of_: the_type_` but this is not idiomatic:
copy_{of_: int_}(value: 3)    # will return the integer `3`

# because it is default named, you can just put in the type without a field name.
copy_{dbl_}(value: 3)         # will interpret `3` as a double and return `3.0`
```

### default-named generic arguments

TODO: restrictions here, do we need to only have a single argument, so that
argument names are unique?  it's probably ok if we have an `@order_independent`
or use `~t_0` and `~t_1` to indicate order is ok.
or need to use `NAMED_` on some of them.
maybe we see if there's an issue when compiling the generics and then complain at compile time.

Similar to the non-generic case, if the `variable_case` identifier
matches the `type_case_` type of a generic, then it's a default-named argument.
For example, `my_type; ~my_type_` or `t: ~t_`.  There is a shorthand for this
which is more idiomatic: `~my_type;` or `~t:`.  Here is a complete example:

```
logger_(~t): t_
     print_("got $(t)")
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
logger_{of_: some_constraint_}(of.): of_
     print_("got $(of)")
     of

# need to explicitly add the type since it's never inferred.
logger_{int_}(3)    # returns the integer `3`
logger_{dbl_}(3)    # will return `3.0`
```

If you want people to pass in the argument with the field name explicit,
you can use the `NAMED_` namespace.  This suppresses the default naming.

```
logger_(~NAMED_of.): of_
     print_("got $(of)))
     of

# need to explicitly add the argument name `of` but
# the type can be inferred due to `~` in the definition.
logger_(of. 3)  # returns the integer `3`
```

If we have a named generic type, just name the `type_case_` type the
same as the `variable_case` variable name (just add a trailing `_`)
so default names can apply.

```
logger_{value_:}(value.): value_
     print_("got $(value)")
     value

logger_{value_: dbl_}(3)    # will return `3.0` and print "got 3.0"
```

If we want to suppress default naming, e.g., require the function argument 
to be `value: XYZ`, then we need to explicitly tell the compiler that we don't
want default names to apply, which we do using the `NAMED_` namespace.

```
logger_(~NAMED_value.): value_
     print_("got $(NAMED_value)")
     NAMED_value

# because of the `~` on the type, it can be called like this,
# which implicitly infers the `value_` type:
logger_(value. 3)   # returns the integer `3`
```

And as in other contexts, you can avoid inferring the type by omitting `~`.

```
# this generic needs to be specified in braces at the call site: 
logger_{value_:}(NAMED_value.): value_
     ...
     NAMED_value

# and because it has a `NAMED_` namespace,
# you need to call it like this:
logger_{value: dbl}(value. 3)  # will return `3.0`
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
     print("calling this_function with $(argument_name): $(argument)")

# internally defines this overload:
this_function_(argument_at: str_, argument: int_): null_
     argument_name: str_(argument_at)
     print("calling this_function with $(argument_name): $(argument)")

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
specified by hand, e.g., `my_class_{int_, n: 0, require: true}` is a compile error.

TODO: do we even need a `require` keyword or can we just do
`second_value{n >= 2}: of_` and assume all booleans are "require"-like?
that would force people who have a lot of things, e.g., `lot_{bool_, at_: str_}`
would look like a `require` statement possibly.
i also don't want to make it look like we're redefining the generic type,
e.g., `;;some_method_{template_type_: some_constraint_}`, so having
`;;some_method_{require: template_type_ is some_constraint_}` is better.

```
my_class_{of_:, n: count_, require: n > 0}:
[    value: of_
     # this field `second_value` is only present if `n` is 2 or more.
     second_value{require: n >= 2}: of_
     # this field `third_value` is only present if `n` is 3 or more.
     third_value{require: n >= 3}: of_
     ... # plz help am i coding this right??     (no, prefer `vector_[n, of_]`)
]
{    # `of_ is hashable_` is true iff `of_` extends `hashable_` either explicitly
     # or implicitly by implementing a `hash_` method like this:
     # TODO: do we even need `require: of_ is hashable_` or can we just do
     # `::hash_{of_: hashable_}(~builder;): null_`
     ::hash_{require: of_ is hashable_}(~builder;):
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

Note that when returning an inlined data type from a function, we do not allow building out
the class body for the type; any indented block will be assumed to be a part of the function
body/definition, e.g.:

```
my_fn_(int:): [x: int_, y: dbl_]
{    # this is part of the `my_fn_` definition,
     # and never a part of the `[x: int_, y: dbl_]` class body.
     [x: 5 + int, y: 3.0]
}
```

If you want to specify methods on a return type, make sure to build it out as a separate
class first.

```
x_and_y_: [x: int_, y: dbl_]
{    ::my_method_(): x + round_(y) int
}

my_fn_(int:): x_and_y_
     [x: int + 5, y: 3.0]
```

## example class definition

```
parent_class_: [name: str_]

# example class definition
example_class_: all_of_
{    parent_class:
     m:
     [    # given the inheritance with `parent_class_`,
          # child instance variables should be defined in this `m: [...]` block.
          # if they are public, a public constructor like `example_class_(x;:. int_)`
          # will be created.
          x; int_

          # instance functions can also be defined here.  they can be set 
          # individually for each class instance, unlike a class function/method
          # which is shared.  we define a default for this function but you could
          # change its definition in a constructor.
          # NOTE: instance functions can use `m` as necessary.
          #       even though we could use the notation `::instance_function_()` here,
          #       we prefer to keep that for methods, to make it more clear that
          #       this has different storage requirements in practice.
          instance_function_(m:): null_
               print_("hello $(m x)!")

          # this class instance function can be changed after the instance has been created
          # (due to being declared with `;`), as long as the instance is mutable.
          some_mutable_function_(); null_
               print_("hello!")
     ]
}
{    # classes must be resettable to a blank state, or to whatever is specified
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
     # return either an `m_` or a `hm_{ok_: m_, er_}` for any error type `er_`.
     # this constructor returns `m_`:
     m_(k: int_): m_(x. k * 1000)

     # some more examples of class methods:
     # prefix `::` (`;;`) is shorthand for adding `m: m_` (`m; m_`) as an argument.
     # this one does not change the underlying instance:
     ::do_something_(int:): int_
          print_("My name is $(m name)")   # `m name` will check child first, then parents.
          # also ok, if we know it's definitely in `parent_class_`:
          print_("My name is $(parent_class name)")
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

Note that you normally call a static/class function like `class_name_ class_function_(...)`,
but you can also do it via `class_function_(m_: class_name_, ...)`.  This is similar to
how you can get internal class types like `class_name_ internal_type_` in a different way like
`internal_type_{m_: class_name_}`.

## declaring methods and class functions outside of the class

You can also define your own custom methods/functions on a class outside of the class body.
Note that we do not allow adding instance functions or instance variables outside
of the class definition, as that would change the memory footprint of each class instance.
You can also use [sequence building](#sequence-building) outside of the class to define
a few methods, but don't use `:` since we're no longer declaring the class.

```
# static function that constructs a type or errors out
example_class_(z: dbl_): hm_{ok_: example_class_, er_: str_}
     # we can use `map_$($er, ...)` as well.
     x: z round_() int_() map_(er: "Need `round_(z)` representable as an `int`.") assert_()
     example_class_(x)

# static function that is not a constructor.
# this function does not require an instance, and cannot use instance variables,
# but it can read (but not write) global variables (or other files):
example_class_ some_static_function_(): int_
     y_string: read_(file: "y.txt")
     int_(?y_string) ?? 7

# a method which can mutate the class instance:
# this could also be defined as `example_class_ another_method_(m;, plus_k: int_): null_`.
example_class_;;another_method_(plus_k: int_): null_
     # outside of a class body, `m` is required to namespace any instance fields,
     # because they are not obviously in scope here like in a class body.
     m x += plus_k * 1000

# Use sequence building.
example_class_@
{    # with sequence building
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
ways: (1) import the full module via `[*]: \/relative/path/to/file` or
`[*]: \\library/module`, or (2) import the specific method/function via e.g.,
`{example_class_ my_added_class_function_(k: int_): example_class_} \/relative/path/to/file`
or `{example_class_::my_added_method_(y: int_): int_} \\library/module`.

Note that we recommend using named fields for constructors rather than static
class functions to create new instances of the class.  This is because named fields
are self descriptive and don't require named static functions for readability.
E.g., instead of `my_date: date_class_ from_iso_string_("2020-05-04")`, just use
`my_date: date_class_(iso_string: "2020-05-04")` and define the
`;;renew_(iso_string: string_)` method accordingly.

## destructors

The `;;renew_(args...): null_` (or `: hm_{ok_: m_, er_: ...}`) constructors
are technically resetters.  If you have a custom destructor, i.e., code
that needs to run when your class goes out of scope, you shouldn't define
`;;renew_` but instead `;;descope_(): null_` for the destructor and
`m_(...): m_` for the constructor.  It will be a compile error if you try
to define any of `m_` or `;;renew_` with the same arguments.

```
destructor_class_: [x: int_]
{    # TODO: the `@debug` annotation should do something interesting, like
     #       stop the debugger when the value is `set`ted or `get`ted.
     m_(DEBUG_x. int_): m_
          print_("x $(DEBUG_x)")
          [x. DEBUG_x]
     # `m_(...): m_` will also add methods like this:
     #   ;;renew_(DEBUG_x. int_): null_
     #       # this will call `m descope_()` just before reassignment.
     #       m = m_(.DEBUG_x)

     # you can define the destructor:
     ;;descope_(): null_
          print_("going out of scope, had x $(x)")
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
since `array swap_(index_0., index_1.)` can easily
become `swap_(array;, index_0., index_1.)`.
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
(a) `;;renew_(args...): null_` or (b) `;;renew_(args...): hm_{ok_: null_, er_: ...}`,
and these methods also allow you to renew an existing class instance as long as
the variable is writable.  Class *function* constructors are defined like
(c) `m_(args...): m_` or (d) `m_(args...): hm_{ok_: m_, er_: ...}`.
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
example_: [x; str_("hello")]
w; example_()
w x += ", world"
print_(w x) # prints "hello, world"

# expands to this:
example_:
[    x; str_("hello")
]
{    # no-copy readonly reference getter.
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
     ;;x_(str;):
          m x <-> str

     # no-copy "take" method.  moves `x` from this temporary.
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
just_copyable_: [a_var; int_]
{    ::some_var_(): int_
          m a_var - 1000

     ;;some_var_(int.): null_
          m a_var = int + 1000

     #(#
     # the following references become automatically defined;
     # they are just thin wrappers around the getters/setters.

     # writable reference
     ;;some_var_(): (int;)
          refer_
          (    ;m
               {$o some_var_()}        # getter: `o` is an instance of `just_copyable_`
               {$o some_var_(.$int)}   # setter
          )

     # readonly reference
     ::some_var_(): (int:)
          refer_
          (    :m
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
just_swappable_: [internal_some_var; int_]
{    ;;some_var_(int;): null_
          m internal_some_var <-> int
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
just_gettable_: [internal_some_var; int_]
{    ::some_var_(): (int:)
          (int: m internal_some_var)

     #(#
     # the following becomes automatically defined:
     ::some_var_(): int_
          (int:) = m some_var()
          int
     #)#
}

# a class with a writable reference method gets a swapper and taker method automatically:
just_referable_: [internal_some_var; int_]
{    ;;some_var_(): (int;)
          (int; m internal_some_var)

     #(#
     # the following swapper becomes automatically defined:
     ;;some_var_(int;): null_
          m some_var_() <-> int

     # the following setter becomes automatically defined:
     ;;some_var_(int.): null_
          m some_var_() = int!

     # and the following taker method becomes automatically defined:
     ..some_var_(): int_
          result; int_
          result <-> m some_var_()
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
inheritance is allowed as well, via `all_of_{parent1:, parent2:} {#( child methods )#}`.
If you want to add child instance fields, use e.g., `all_of_{parent:, m: [field1: int_, ...]}`
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
{    ;;renew_(m name. string_): {}

     # define two methods on `animal_`: `speak_` and `go_`.
     # these are "abstract" methods, i.e., not implemented by this base class.
     ::speak_(): null_
     ::go_(): string_

     # this method is defined, so it's implemented by the base class.
     # derived classes can still override it, but this default will be defined for them.
     ::escape_(): null
          print_("$(m name) $(m go_()) away!!")

     # copy method that returns an instance of whatever the class instance
     # type is known to be.  e.g., an animal returns an animal instance,
     # while a subclass would return a subclass instance:
     ::o_(): m_
          m_(name o_())
}

snake_: animal_
{    # if no `renew_` functions are defined,
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
cat_: all_of_{animal:, m: [fur_balls: int_]}
{    # here we define a `renew_` method, so the parent `renew_` methods
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
horse_: all_of_{animal:, m: [owner: str_]}
{    # this passes `name` to the `animal_` constructor and sets `owner` on self:
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
(    name: "Waberoo"
     ::speak_(): null_
          print_("Meorooo")
     ::go_(): "meanders"
     ::escape_(): null_
          # to call the parent method `escape_()` in here, we can use this:
          animal::escape_()
          print_("$(m name) $(m go_()) back...")
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
{    ;;renew_(m i8. -128): {}

     # cloning works without errors:
     ::o_(): m_
          [m i8]

     ::!(): bool_    # overload `!m`
          m i8 == 0 || m i8 == -128

     ;;+=(o): null_
          if m i8 == -128
               return_()
          if o i8 == -128
               m i8 = -128
               return_()
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
mythological_cat_: all_of_{cat:, m: [lives; 9]}

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
{    ;;renew_(int.): null_
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
my_generic_{of_:}(y: of_, z: of_): of_
     x: y * z
     x
```
If `of_` was nullable, then `x` would potentially be nullable, and should
be defined via `x?: y * z`.  But because oh-lang does template specialization
only after you supply the specific type you want, this can be caught at
compile time and only if you're requesting an invalid type.

To create a generic class, you put the expression `{type1_:, ...}` after the
class identifier, or we recommend `{of_:}` for a single template type, where
`of_` is the [default name for a generic type](#default-named-generic-types).
For example, we use `my_single_generic_class_{of_:}: [...]` for a single generic
or `my_multi_generic_class_{type1_:, type2_:}: [...]` for multiple generics.
To actually specify the types for the generic class, we use the syntax
`my_single_generic_class_{int_}` (for an `of_`-defined generic class) or
`my_multi_generic_class_{type1_: int_, type2_: str_}` (for a multi-type generic).
Note that any static/class methods defined on the class can be accessed
like this: `my_single_generic_class_{int_} my_class_function_(...)` or
`my_multi_generic_class_{type1_: int_, type2_: str_} other_class_function_()`.

```
generic_class_{id_:, value_:}: [id;, value;]
{    # this gives a method to construct the instance and infer types.
     # `g_` is like `m_` but without the template specialization, so
     # `g_` is `generic_class_` in this class body.
     g_(id. ~t_, value. ~u_): g_{id_: t_, value_: u_}
          [id, value]
}

# creating an instance using type inference.
# `id_` will be an `int_` and `value_` will be a `str_`.
class_instance: generic_class_(id. 5, value. "hello")
 
# creating an instance with template/generic types specified:
other_instance: generic_class_{id_: dbl_, value_: string_}(id. 3, value. "4")
```

### default-named generics

If you have a generic class like `my_generic_{type1_:, type2_:}`, you can use them as a
default-named function argument like `my_generic{type1_, type2_}:`, which is short for
`my_generic: my_generic_{type1_, type2_}`.  This works even for generics over values,
e.g., if `fixed_array_{count}` is a fixed-size array of size `count`, then `fixed_array{3}:`
can be a declaration for a fixed array of size 3.

### generic class type mutability

It may be useful to create a generic class that whose specified type
can have writeable or readonly fields.  This can be done using `variable_name\` some_type_`
inside the generic class definition to define variables, and then specifying
the class with `{type1_: specified_readonly_type_, type2_; specified_writeable_type_}`.
TODO: this is going to be a bit difficult to get right with vim syntax;
can we use `~` instead with a space afterwards?

```
# TODO: should we declare like this? `{x_;:., ...}` to indicate that the
#    type can be mutated differently?
#    we could do `gen_{x_:;}: [whatever:; x_]` and use respectively on `{x_}` whenever `x_` is encountered,
#    but only in the `[]` brackets
mutable_types_{x_:, y_:, z_:}:
[    # these fields are always readonly:
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
{    # you can also use these in method/function definitions:
     ::some_method_(whatever_x` x_, whatever_y` y_): null
}

# the following specification will make `v_x` and `v_z` writeable
# and `v_y` readonly:
my_specification: mutable_types_{x_; int_, y_: string_, z_; dbl_}
```

We use a new syntax here because it would be confusing
to reinterpret a generic class declaration of a variable declared using `:`
as writeable in a specification with a `;`.

Note that if the generic class has no backticks inside, then it is a compile error
if you try to specify the generic class with a `;` type.  E.g., if we have the declaration
`generic_{a_:}: [a;]`, then the specification `my_gen: generic_{a_; int_}(5)`
is a compile error.  If desired, we can switch to `generic_{a_:}: [a\`]`
to make the specification correct.

### virtual generic methods

You can also have virtual generic methods on generic classes, which is not allowed by C++.

```
generic_{of_:}: [value; of_]
{    ::method_(~u): u_
          u + u_(u * m value) ?? panic_()
}

generic; generic_{str_}
generic value = "3"
print_(generic method_(2_i32))  # prints "35" via `2_i32 + i32_(2_i32 * "3")`

specific_{of_: number_}: all_of_{generic{of_}:, m: [scale; of_]}
{    ;;renew_(m scale. of_ = 1, generic value.): {}

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
my_lot; lot_{NAMESPACE_at_, value_}
# Equivalent to `my_lot; lot_{at_: NAMESPACE_at_, value_: value_}`.
```

### generic type constraints

To constrain a generic type, use `{type_: constraints_, ...}`.  In this expression,
`constraints_` is simply another type like `non_null_` or `number_`, or even a combination
of classes like `all_of_{container_{id_, value_}, number_}`.  It may be recommended for more
complicated type constraints to define the constraints like this:
`my_complicated_constraint_type_: all_of_{t1:, one_of{t2:, t3:}:}` and declaring the class as
`new_generic_{of_: my_complicated_constraint_type_}`, which might be a more readable way to do
things if `my_complicated_constraint_type_` is a helpful name.
TODO: `all_of_` is acting a little bit differently than a child class inheritor here,
do we need to distinguish between the two?  e.g., the child class usage of `all_of_`
will be ordered, but `all_of_` here in a type constraint should not require a certain order.
or maybe we need to do `{of_: constraint1_, of_: constraint2_}`.

### generic type defaults

Type defaults follow the same pattern as type constraints but the default types are
not abstract.  So we use `{type_: default_type_, ...}` where `default_type_` is a class
that is non-abstract.

### overloading generic types

Note that we can overload generic types (e.g., `array_{int_}` and `array_{count: 3, int_}`),
which is especially helpful for creating your own `hm_` result class based on the general
type `hm_{er_, ok_}`, like `MY_er_: one_of_{oops:, my_bad:}, hm_{of_:}: hm_{ok_: of_, MY_er_}`.
Here are some examples:

```
# Note that in oh-lang we could define this as `pair_{of_1_, of_2_}`
# so we don't need to specify `first_: int_, second_: dbl_`, but for illustration
# in the following examples we'll make the generic parameters named.
pair_{first_:, second_:}: [first;, second;]
pair_{of_:}: pair_{first_: of_, second_: of_}

# examples using `pair_{of_}`: ======
# an array of pairs:
pair_array: array_{pair_{int_}}([[first. 1, second. 2], [first. 3, second. 4]])
# a pair of arrays:
pair_of_arrays: pair_{array_{int_}}([first. [1, 2], second. [3, 4]])

# examples using `pair_{first_, second_}`: ======
# an array of pairs:
pair_array: array_{pair_{first_: int_, second_: dbl_}}
(    [first. 1, second. 2.3]
     [first. 100, second. 0.5]
)
# a lot of pairs:
pair_lot: lot_{at_: str_, pair_{first_: int_, second_: dbl_}}
(    "hi there". [first. 1, second. 2.3]
)
```

### default named generic types

The default name for a type is `of_`, mostly to avoid conflicts with
`type_` which is a valid verb (e.g., to type in characters), but also
to reduce the letter count for generic class types.  Default names
are useful for generics with a single type requirement, and can be
used for overloads, e.g.:

```
a_class_{x_:, y_:, n: count_}: array_{[x:, y:], count: n}

a_class_{of_:}: a_class_{x_: of_, y_: of_, n: 100}
```

TODO: use `vector_{count:}` for a fixed-length array (all elements default-initialized)
and `array_{max_count: count_}` for a max-length array.  unless we want
to just overload; `vector_{count:}` will have a different API than `array_`, though.
e.g., `vector_` is also not allocated dynamically.

Similar to default-named arguments in functions, default-named generics
allow you to specify the generic without directly using the type name.
For example:

```
# use the default-name `type_` here:
a_class_{of_:, n: count_}: a_class_{x_: of_, y_: of_, n}

# so that we can do this:
an_instance: a_class_{dbl_, n: 3}
# equivalent but not idiomatic: `an_instance: a_class_{of_: dbl_, n: 3}`.
```

Similar to default-named arguments in functions, there are restrictions.
You are not able to create multiple default-named types in your generic
signature, e.g., `my_generic_{A_of_, B_of_}`, unless we use `_0` and
`_1` namespaces, e.g., `my_generic_{of_0_, of_1_}`.  These
should only be used in cases where order intuitively matters.

### generic overloads must use the original class or a descendant

To avoid potential confusion, overloading a generic type must use
the original class or a descendant of the original class for any
overloads.  Some examples:

```
some_class_{x_:, y_:, n: count_}: [ ... ]

# this is OK:
some_class_{of_:, n: count_}: some_class_{x_: of_, y_: of_, n}

# this is also OK:
child_class_{of_:}: some_class_{x_: of_, y_: of_, n: 256}
{    # additional child methods
     ...
}
some_class_{of_:}: child_class_{of_}

# this is NOT OK:
some_class_{t_:, u_:, v_:}: [ ...some totally different class... ]
```

Note that we can support a completely specified generic class, e.g.,
`some_class_: some_class_{my_default_type_}`; we can still distinguish
between the two usages of `some_class_{specified_type_}` and `some_class_`,
as long as there's no default concrete specification all other types, e.g.,
`some_class_{of_: some_concrete_specified_type_}` would already specify the default.

### type tuples

TODO: not sure this works with {} for generics.  may need to modify.

One can conceive of a tuple type like `{x_, y_, z_}` for nested types `x_`, `y_`, `z_`.
They are grammatically equivalent to a `lot` of types (where usually order doesn't matter),
and their use is make it easy to specify types for a generic class.  This must be done
using the spread operator `...` in the following manner.

```
tuple_type_: {x_, y_, z_}

# with some other definition `my_generic_{w_:, x_:, y_:, z_:}: [...]`:
some_specification_: my_generic_{...tuple_type_, w_: int_}

# you can even override one of your supplied `tuple_type_` values with your own.
# make sure the override comes last.
another_spec_{OVERRIDE_of_:}: my_generic_{...tuple_type_, w_: str_, x_: OVERRIDE_of_}

# Note that even if `tuple_type_` completely specifies a generic class
# `some_generic_{x_:, y_:, z_:}: [...]`, we still need to use the spread operator
# because `some_generic_ tuple_type_` would be syntax for something else,
# i.e., the nested class `tuple_type_` within `some_generic_`, which will
# be a compiler error if not present.  Instead:
a_specification_: some_generic_{...tuple_type_}
```

Here is an example of returning a tuple type.

```
# TODO: this is probably a bad example because we shouldn't have randomness here
tuple_{dbl:}: {precision_, vector2_: any_}
     if abs_(dbl) < 128.0
          {precision_: flt_, vector2_: [x: flt_, y: flt_]}
     else
          {precision_: dbl_, vector2_: [x: dbl_, y: dbl_]}

my_tuples_: tuple_{random_() dbl * 256.0}
my_number; my_tuples_ precision_(5.0)
my_vector; my_tuples_ vector2_(x: 3.0, y: 4.0)
```

See also [`new_{...}: ...` syntax](#returning-a-type).

### default field names with generics

Note that generic classes like `generic_{of_:}: [of;]` do not work exactly the same way as
generic arguments in functions like `fn_{of_:}(of.)`.  The latter uses a default-named
argument in a reference object and the former creates an object whose field is always
named `of`, regardless of what the generic type `of_` is.  Thus `fn_{of_:}(of.)` can be
called like `fn_{int_}(5)` (without `int. 5` or `of. 5` specified), while creating a generic
class `generic_{of_:}: [of;]` will always declare the field with name `of`, e.g.,
`generic_{int_}` is `[of: int_]` instead of `{int: int_}`, and which should be instanced
as `generic{int_}: [of: 3]`.
TODO: should we actually use `[int: int_]`?  we should be able to accept input like this:
`gi_: generic_{int_}, gi: [5]`.  if we have two (or more arguments) that conflict,
e.g., `element_{at_:, of_:}: [at;, of;]` and use `element_{at_: str_, of_: str_}`,
then we can use namespaces like this: `[AT_str;, OF_str;]`.  i'm not sure i like this,
however, and we probably want something like `generic_{x_:, y_:}: [x;, y;]` to look
like `[x: dbl_, y: str_]`, etc.

There's a slight bit of inconsistency here, but it makes defining generic classes
much simpler, especially core classes like `hm_{ok_:, er_:}: one_of_{ok:, er:} {...}`,
so we always refer to a good value as `ok` and an error result as `er`, rather
than whatever the internal values are.
TODO: actually `one_of_` probably follows different rules to make sure things
are named correctly.

## common class methods

All classes have a few compiler-provided methods which have special constraints;
some cannot be overridden, and some must have certain function signatures.

* `(m;)!: m_` creates a temporary with the current instance's values, while
     resetting the current instance to a default instance -- i.e., calling `;;renew_()`.
     Internally, this swaps pointers, but not actual data, so this method
     should be faster than copy for dynamically-allocated types.  This method
     cannot be overridden.  Similarly for `(m.)!: m_`, although this may
     elide the `renew_` call on the temporary.
* `..map_(an_(m.): ~t_): t_` to easily convert types or otherwise transform
     the data held in `m`.  This method consumes `m`.  You can also overload
     `map_` to define other useful transformations on your class, but not override.
* `::map_(an_(m:): ~t_): t_` is similar to `..map_(an_(m.): ~t_): t_`,
     but this method keeps `m` constant (readonly).  You can overload but not override.
* you can define one of `m_(...): m_` (class constructors) or `;;renew_(...): null` 
     (renew methods), and the other will be automatically defined with the same arguments.
     `;;renew_` can be called on any writable variable even if some fields are constant.
* you can define one of `m_(...): hm_{ok_: m_, er_: ...}` (class-or-error constructors)
     or `;;renew_(...): hm_{ok_: m_, er_: ...}` (renew-or-error methods), and the other
     will be automatically defined with the same arguments.
* besides `m_(...): m_` and `m_(...): hm_{ok_: m_, er_: ...}`, you are not allowed to define
     any other methods named `m_` that return anything else.  Defining both is ok,
     and defining overloads for different input arguments is ok.
* you can define `::o_(): m_` (copy constructor) or `::o_(): hm_{ok_: m_, er_: ...}`
     (copy-or-error constructor), or both.
* besides `::o_(): m_` and `::o_(): hm_{ok_: m_, er_: ...}`, you are not allowed to define
     any other methods named `o_`.  Defining both is ok; whichever one comes first
     is the default copy method, so we recommend the copy-or-error constructor coming
     first.

## singletons

Defining a singleton class is quite easy, simply by instantiating a class 
by using `variable_case` when defining it.

```
awesome_service: all_of_
{    parent_class1:, parent_class2:, #(etc.)#,
     m: [url_base: "http://my/website/address.bazinga"]
}
{    ::get_(id: string_): awesome_data_
          json: http get_("$(m url_base)/awesome/$(id)")
          awesome_data_(json)
}
```

Using `@singleton type_case_` on the LHS defines an abstract singleton.
These are useful when you want to be able to grab an instance of the concrete
child-class but only through the parent class reference.  Errors will be
thrown if multiple children implement the parent and instantiate.

```
### screen.oh ###
@singleton
screen_: []
{    ;;draw_(image;, vector2.): null_
     ;;clear_(color. color_ black)
}
### implementation/sdl-screen.oh ###
sdl_screen_: \/../screen screen_
{    ;;draw_(image;, vector2.): null_
          # actual implementation code:
          m sdl_surface draw_(image, vector2)

     ;;clear_(color. color_ black)
          m sdl_surface clear_(color)
}
### some-other-file.oh ###
# this is an error if we haven't imported the sdl-screen file somewhere:
screen; screen_
screen clear_(color_(r. 50, g. 0, b. 100))
```

You get a run-time error if multiple child-class singletons are imported/instantiated
at the same time.

## sequence building

Sequence building is using syntax like `a@ [b, c_()]` to create `[b: a b, c: a c_()]`,
and similarly for `()` which creates a reference object, and `{}` which doesn't create
an object but just sequentially evaluates methods/fields.  If you need the LHS of a
sequence builder to come in at a different spot, use `@` inside the parentheses, e.g.,
`a@ [b + @ x_(), if @ y_() { c } else { @ z }, w]`, which corresponds to
`[b: b + a x_(), y: if a y_() { c } else { a z }, w: a w]`.  Note that if you use `@`
anywhere in a parenthetical statement, you need to use it everywhere you want the LHS
to appear.  (A parenthetical statement is considered just one of statements here:
`[statement1, statement2, ...]`.)

Why would you need sequence building?
Some languages use a builder pattern, e.g., Java, where you add fields to an object
using setters.  E.g., `/* Java */ myBuilder.setX(123).setY(456).setZ("great").build()`.
In oh-lang, this is mostly obviated by named arguments:
`my_class_(x. 123, y. 456, z. "great")` could do the same thing.  However, there are
still situations where it's useful to chain methods on the same class instance, and
oh-lang does not recommend returning a reference to the class via the return type `(m;)`.
More idiomatically, we use sequence building with all the method calls inside a block.
For example, if we were to implement a builder pattern with setters, we could combine
a bunch of mutations like this:

```
# class definition:
my_builder_: [...]
{    ;;set_(string., int.): null_    # no need to return `(m;)`
}

# Note, inside the `{}` we allow mutating methods because `my_builder_()` is a temporary.
# The resulting variable will be readonly after this definition + mutation chain,
# due to `my_builder` being defined with `:`.
my_builder: my_builder_()@
{    set_("abc", 123)
     set_("lmn", 456)
     set_("xyz", 789)
     # etc.
}

# You can also do inline, but you should use commas here.
# Note that this variable can be mutated after this line due to being defined with `;`.
my_builder2; my_builder_()@ {set_("def", 987), set_("uvw", 321)}
```

By default, if the left-hand side of the sequence builder is writable (readonly),
the methods being called on the right will be the writable (readonly) versions
when using implicit member access (e.g., not explicitly using `::` or `;;`).
E.g., if `my_builder_()` is the left-hand side for the sequence builder, it is a
temporary which defaults to writable.  You can explicitly ask for the readonly
(or writable) version of a method using `::` (or `;;`), although it will be a
compile-error if you are trying to write a readonly variable.

The return value of the sequence builder also depends on the LHS.
If the LHS is a temporary, the return value will be the temporary after it has been called
with all the methods in the RHS of the sequence builder.  E.g., from the above example,
a `my_builder_` instance with all the `set_` methods called.  Otherwise, if the LHS
is a reference (either readonly or writable), the return value of the sequence
builder will depend on the type of parentheses used:

* `{}` returns the value of the last statement in `{}`,
* `[]` creates an object with all the fields built out of the RHS methods, and
* `()` creates a reference object with all the fields built out of the RHS methods.

Some examples of the LHS being a reference follow:

```
readonly_array: [0, 100, 20, 30000, 4000]
results: readonly_array@
[    [2]            # returns 20
     ::sort_()      # returns a sorted copy of the array; `::` is unnecessary
     ::print_()     # prints unsorted array; `::` is unnecessary
     # this will throw a compile-error, but we'll discuss results
     # as if this wasn't here.
     ++@;;[3]       # compile error, `readonly_array` is readonly
]
# should print [0, 100, 20, 30000, 4000] without the last statement
# results == [int: 20, sort: [0, 20, 100, 4000, 30000]]

writeable_array; [-1, 100, 20, 30000, 4000]
result: writeable_array@
[    [2]             # returns 20
     sort_()         # in-place sort, i.e., `;;sort_()`
     ++@;;[3]        # OK, a bit verbose since `;;` is unnecessary
     # prints the array after all the above modifications:
     ::print_()      # OK, we probably don't have a `;;print_()` but you never know
     min_()
]
# should print [-1, 20, 100, 4001, 30000]
# note that `sort_()` returns null and thus collapses.
# result == [int_0: 20, int_1: 4001, min: -1]
```

### field renaming in sequence builders

You can use field names in sequence builders in order to better organize things.
This is only useful if the LHS is not a temporary, since a temporary LHS is returned
as sequence builder's value, or if you are using the variable for something else inside
the sequence builder.

```
my_class: [...]
results: my_class@
[    field1: @ my_method_()
     field2: @ next_method_()
]
# The above is equivalent to the following:
results:
[    field1: my_class my_method_()
     field2: my_class next_method_()
]

# This is a compile error because the LHS of the sequence builder `my_class get_value_()`
# is a temporary, so the fields are not used in the return value.
# this also would be a compile error for `()` sequence builders.
results: my_class get_value_()@
[    field1: @ do_something_()
     field2: @ do_something_else_()
]   # COMPILE ERROR: `field1` is an unused variable

# this would be ok:
results: my_class get_value_()@
{    field1: @ do_something_()
     print_(@ do_something_else_() * field1)
}
```

### nested sequence builders

There is one exception in oh-lang for shadowing identifiers, and it is for `@`
inside nested sequence builders.  We don't expect this to be a common practice.

```
# Example method sequence builder:
my_class@
[    my_method_()@ [next_method_(), next_method2_(), nested_field]
     other_method_()
     some_field
]

# Is equivalent to this sequence:
HIDDEN_result; my_class my_method_()
next_method: HIDDEN_result next_method_()
next_method2: HIDDEN_result next_method2_()
nested_field: HIDDEN_result nested_field
other_method: my_class other_method_()
some_field: my_class some_field
# This is constructed (since it's defined with `[]`):
[my_method: [next_method, next_method2, nested_field], other_method, some_field]
```

# aliases

Aliases enable writing out logic with semantically similar descriptions, and 
are useful for gently adjusting programmer expectations.  The oh-lang formatter will
substitute the preferred name/logic for any aliases found.

Aliases can be used for simple naming conventions, e.g.:

```
options_: one_of
{    align_inherit_x: 0
     align_center_x:
     align_left:
     align_right:
}
{    @alias inherit_align_x: align_inherit_x
}

options: _ inherit_align_x    # converts to `_ align_inherit_x` on next format.
```

Aliases can also be used for more complicated logic and even deprecating code.

```
my_class_: [x; int_]
{    # implicit constructor:
     ;;renew_(m x. int_): {}

     # This was here before...
     # ;;my_deprecated_method_(delta_x: int_): null_
     #     m x += delta_x

     # But we're preferring direct access now:
     @alias ;;my_deprecated_method_(delta_x: int_): null_
          m x += delta_x
}

my_class; my_class_(x: 4)
my_class my_deprecated_method_(delta_x: 3)  # converts to `my_class x += 3` on next format.
```

While it is possible, it is not recommended to use aliases to inline code.
This is because the aliased code will be immediately and permanently inlined,
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
paths.  You can also use `oh_("./relative/path/to/file.oh")`, which does require
the final `.oh` extension.

For example, suppose we have two files, `vector2.oh` and `main.oh` in the same
directory.  Each of these is considered a module, and we can use backslashes
to invoke logic from these external files.

```
# vector2.oh
vector2_: [x: dbl_, y: dbl_]
{    ;;renew_(m x. dbl_, m y. dbl_): {}

     ::dot_(o): dbl_
          m x * o x + m y * o y
}

# main.oh
vector2_oh: \/vector2   # .oh extension can be used but will be formatted off.
# alternatively: `vector2_oh: oh_("./vector2.oh")`
# alternatively: `[vector2_]: \/vector2` or `[vector2_]: oh_("./vector2.oh")`
vector2: vector2_oh vector2_(x. 3, y. 4)
print_(vector2)
# you can also destructure imports like this:
```

For concision, we can use `\/other_file/some_class _` to reference the class
`some_class_` within `./other_file/some_class.oh`.  So the above example
would be more idiomatically written like this:

```
# main.oh
vector2: \/vector2 _(x. 3, y. 4)
print_(vector2)
```

Note that we cannot import a function like this: `[my_function_]: \/other_file`;
to oh-lang this looks like a type.  You either need to specify the overload
that you're pulling in, e.g., `[my_function_(int): str_]: \/other_file`,
or request all overloads via `[my_function_(call;): null_]: \/other_file`.
Or you can just import the file and use the function as needed:
`other_file: \/other_file, other_file my_function_(123)`.
TODO: i think we can relax this requirement; if you request `[my_function_]` it can just
be the function with all overloads; otherwise we should technically require specifying
type "overloads" for generic types like `hm_{of_:}: hm_[ok_: of_, er_: ...]` that come
from other files.  there's not a huge difference between types and functions, they both can
take arguments to return something else.  but we can't use `my_function_` as a type, so it
would be nice to distinguish, maybe `[my_function_(*): *]: \/other_file`?

You can use this `\/` notation inline as well, which is recommended
for avoiding unnecessary imports.  It will be a language feature to
parse all imports when compiling a file, regardless of whether they're used,
rather than to dynamically load modules.  This ensures that if another imported file
has compile-time errors they will be known at compile time, not run time.

```
# importing a function from a file in a relative path:
print_(\/path/to/relative/file function_from_file_("hello, world!"))

# importing a function from the math library:
angle: \\math atan2_(x: 5, y: -3)
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
`\/(relative/path/with a space/to/a/great file)` for a relative path; you can use
any type of parenthetical `{}`, `[]`, or `()` for either absolute or relative.  Or
you can use a backslash to escape the space, e.g., `\\library/path/with\ spaces` or
`\/relative/path/with\ a\ space/to/a/great\ file`.  Other standard escape sequences
(using backslashes) will probably be supported.

Note that we take the entire import as
if it were an `variable_case` identifier.  E.g., `\\math` acts like one identifier, `math`,
so `\\math atan_(x, y)` resolves like `math atan_(x, y)`, i.e., member access or drilling down
from `math: \\math`.  Similarly for any relative import; `\/relative/import/file some_function_(q)`
correctly becomes like `file some_function_(q)` for `file: \/relative/import/file`.

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
`script: oh_("../my_script/doom.ohs") map_({$_er, "should compile"})`.

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
private_function_(x: int_, y: int_): [z: str_]
     z: "$(x):$(y)"

@protected
protected_function_(x: int_, y: int_): [z: str_]
     [z;] = private_function_(x, y)
     z += "!"
     [z]

public_function_(x1: int_, y1: int_, x2: int_, y2: int_): null_
     print_(protected_function_(x: x1, y: y1) z, private_function_(x: x2, y: y2))

@test "foundation works fine":
     assert_(private_function_(x: 5, y: 3)) == [z: "5:3"]
     assert_(private_function_(x: -2, y: -7)) == [z: "-2:-7"]

@test "building blocks work fine":
     assert_(protected_function_(x: 5, y: -3)) == [z: "5:-3!"]
     assert_(protected_function_(x: -2, y: 7)) == [z: "-2:7!"]

@test "public function works correctly":
     public_function_(x1: -5, y1: 3, x2: 2, y2: 7)
     assert_(test printed_()) == ["-5:3!2:7"]

     @test "nested tests also work":
          public_function_(x1: 2, y1: -7, x2: -5, y2: -3)
          assert_(test printed_()) == ["2:-7!-5:-3"]
```

See [the test definition](https://github.com/oh-lang/oh/blob/main/core/test.oh) for
how the `test` function and `@test` macro work.

Nested tests will freshly execute any parent logic before executing themselves.
This ensures a clean state.  If you want multiple tests to start with the same
logic, just move that common logic to a parent test.

Inside of a `@test` block, you have access to a `test` variable which includes
things like what has been printed (`test printed_()`).  `test printed_()`
will pull everything that would have been printed in the test, putting it into
a string array (one string per newline), for comparisons and matching.
It then clears its internal state so that new calls to `test printed_()`
will only see new things since the last time `test printed_()` was called.

Parametric tests are also possible; just make sure to use `@each` (or another control flow macro)
in order to expand the loops at compile time.

```
@test_only
test_case_: [argument: str_, result: int_]

@test_only
test_cases: lot_{at_: str_, test_case_}
(    "hello": [argument: "hello world", result: 11]
     "wow": [argument: "wowee", result: 5]
)

@test "do_something":
     # this common setup executes before each parametric test;
     # each nested test starts with the common setup from fresh
     # and doesn't continue to use the environment for the next nested test.
     get_environment_set_up_()

     test_cases @each (name: at_, test_case:)
          @test "testing $(name)":
               assert_(do_something_(test_case argument)) == test_case result
```

Integration tests can be written in files that end with `.test.oh` or `.test.ohs` (i.e., as a script).
These can pull in any dependencies via standard file/module imports, including other test files.
E.g., if you create some test helper functions in `helper.test.oh`, you can import these
into other test files (but not non-test files) for usage.

Unit and integration tests are run via `oh test .` in the directory you want,
or `oh test subdirectory/`; only tests in that directory (and recursive subdirectories)
will be run.  `oh test` will run all tests by default.

## file access / file system

Files can be opened via the `file_` class, which is a handle to a system file.
See [the `file_` definition](https://github.com/oh-lang/oh/blob/main/core/file.oh).

TODO: make it possible to mock out file system access in unit tests.

# errors and asserts

## hm

TODO: should we be using `hm_{ok:, er:}` since this boils down to a `one_of_{ok:, er:}`?
probably not since we need to pass it into the `one_of_`.

oh-lang borrows from Rust the idea that errors shouldn't be thrown, they should be
returned and handled explicitly.  We use the notation `hm_{ok_, er_}` to indicate
a generic return type that might be `ok_` or it might be an error (`er_`).
In practice, you'll often specify the generic arguments like this:
`hm_{ok_: int_, er_: string_}` for a result that might be ok (as an integer) or it might
be an error string.  If your function never fails, but the interface requires using
`hm_`, you can use `hm_{ok_, er_: never_}` to indicate the result will never be an error.

To make it easy to handle errors being returned from other functions, oh-lang uses
the `assert_` method on a result class.  E.g., `ok: my_hm assert_()` which will convert
the `my_hm` result into the `ok` value or it will return the `er_` error in `my_hm` from
the current function block, e.g., `ok: what my_hm { ok: {ok}, er: {return: er} }`.
It is something of a macro like `?` in Rust.  Note that `assert_` doesn't panic,
and it *always runs*, not just in debug mode.  See [its section](#assert) for more details.

Note that we can automatically convert a result type into a nullable version
of the `ok_` type, e.g., `hm_{ok_: string_, er_: error_code_}` can be converted into
`string_?` without issue, although as usual nulls must be made explicit with `?`.
E.g., `my_function_(string_argument?: my_hm)` to pass in `my_hm` if it's ok or null if not,
and `string?: my_hm` to grab it as a local variable.  This of course only works
if `ok` is not already nullable, otherwise it is a compile error.

See [the `hm_` definition](https://github.com/oh-lang/oh/blob/main/core/hm.oh)
for methods built on top of the `one_of_{ok:, er:}` type.

```
result: if x { ok_(3) } else { er_("oh no") }
if result is_ok_()
     print_("ok")

# but it'd be nice to transform `result` into the `ok` (or `er`) value along the way.
result is_(an_(ok): print_("ok: $(ok)"))
result is_(an_(er): print_("er: $(er)"))

# or if you're sure it's not an error, or want the program to terminate if not:
ok: result ?? panic_("expected `result` to be ok!!")
```

A few keywords, such as `is`, are actually [operators](#is-operator), so we can
overload them and use them in this slightly more idiomatic way.  Notice that
we declare an `ok` variable here so we need to use a colon (e.g., `ok:`).

```
if result is ok:
     print_("ok: ", ok)
elif result is er:
     print_("er: ", er)
```

Or use `what` if you want to ensure via the compiler that you get all cases:

```
what result
     ok:
          print_("ok: ", ok)
     er:
          print_("er: ", er)
```

If you want to be more strict in matching after defining a variable,
use the [`where` operator](#where-operator).

## assert

The built-in `assert_` statement will shortcircuit the block if the rest of the statement
does not evolve to truthy.  As a bonus, when returning, all values will be logged to stderr
as well for debugging purposes for debug-compiled code.  For more technical details, see
[the definition](https://github.com/oh-lang/core/blob/main/core/assert.oh).

```
assert_(some_variable) == expected_value    # "throws" if `some_variable != expected_value`,
                                                       # printing to stderr the values of both `some_variable`
                                                       # and `expected_value` if so.

# both of the next statements throw if `some_class method_(100)` is not truthy,
# but the first also logs `some_class`, the text `some_class method_(100)`, and its value.
assert_(some_class) method_(100)
assert_(some_class method_(100))

assert_(some_class) other_method_("hi") > 10    # throws if `some_class other_method_("hi") <= 10`,
                                                            # printing value of `some_class` as well as
                                                            # `some_class other_method_("hi")`.
```

Note that `assert_` logic is always run, even in non-debug code.  To only check statements in the
debug binary, use `assert_(debug_only, ...)`, which otherwise has the same signature as `assert_(...)`.
Using debug asserts is not recommended, except to enforce the caller contract of private/protected
methods.  For public methods, `assert_` should always be used to check arguments.

Note that for functions that return results, i.e., `hm_{ok_:, er_:}`, `assert_` will automatically
return early with an `er_` based on the error the `assert_` encountered.  If a function does
*not* return a result, then using `assert_` will be a run-time panic; to make sure that's
what you want, annotate the function with `@can_panic`, otherwise it's a compile warning/error in
debug/release mode (respectively).

## automatically converting errors to null

If a function returns a `hm_` type, e.g., `my_function_(...): hm_{ok_: ..., er_: ...}`,
then we can automatically convert its return value into a `ok_?`, i.e.,
a nullable version of the `ok_` type.  This is helpful for things like type casting;
instead of `my_int: what int_(my_dbl) {ok. {ok}, er: {-1}}` you can do
`my_int: int_(my_dbl) ?? -1`.  Although, there is another option that
doesn't use nulls:  `int_(my_dbl) map_(fn_(_er): -1)`, or via
[lambda functions](#lambda-functions): `int_(my_dbl) map_$($_er, -1)`.

TODO: should this be valid if `ok` is already a nullable type?  e.g.,
`my_function_(): hm_{ok_: int_?, er_: str_}`.
we probably should compile-error-out on casting to `int?: my_function_()` since
it's not clear whether `int` is null due to an error or due to the return value.
maybe we allow flattening here anyway.

# standard container classes (and helpers)

Brackets are used to create containers, e.g., `y: "Y-Naught", z: 10, [x: 3, {y}: 4, z]`
to create a lot with keys "x", the value of `y` ("Y-Naught"), and "z", with
corresponding values 3, 4, and the value of `z` (10).  Thus any bracketed values, as
long as they are named, e.g., `a: 1, b: 2, c: 3, [a, b, c]`, can be cast into a lot.
Because containers are by default insertion-ordered, they can be implicitly cast to an
array depending on the type of the receiving variable.  This cast happens only
conceptually; constructing an array doesn't construct a `lot` first to convert.
See [the container definition](https://github.com/oh-lang/oh/blob/main/core/container.oh)
for more details.

TODO: discuss the expected behavior of what happens if you delete an element
out of a container when iterating over it (not using the iterator itself, if
it supports delete).  for the default implementation, iterators will only
hold an index to an element in the container, and if that index no longer
indexes an element, we can stop iteration.

## arrays

TODO: switch to `hazable`

An array contains a list of elements in contiguous memory.  You can define
an array explicitly using the notation `array_name: array_{element_type_}`
for the type `element_type`.  The default-name of an array does not depend
on the element type; it is always `array`, e.g., `array{element_type_}:`
to define `array: array_{element_type_}`.  For example, to declare an
argument which is a default-named array of strings, you'd use a function
signature like `my_function_(array{string_};:.): null_`.  To define an array
quickly (i.e., without a type annotation), use the notation `["hi", "hey"]`.
Example usage and declarations:

```
# this is a readonly array:
my_array: array_{dbl_}(1.2, 3, 4.5)     # converts all to `dbl_`
my_array append_(5) # COMPILE ERROR: `my_array` is readonly
my_array[1] += 5    # COMPILE ERROR: `my_array` is readonly

# writable integer array:
array{int_};        # declaring a writable, default-named integer array
array append_(5)    # now `array == [5]`
array[3] += 30      # now `array == [5, 0, 0, 30]`
array[4] = 300      # now `array == [5, 0, 0, 30, 300]`
array[2] -= 5       # now `array == [5, 0, -5, 30, 300]`

# writable string array:
string_array; array_{string_}("hi", "there")
print_(string_array pop_())     # prints "there".  now `string_array == ["hi"]`
```

The default implementation of `array_` might be internally a contiguous deque,
so that we can pop or insert into the beginning at O(1).  We'll reserve
`stack_` for a contiguous list that grows in one direction only.
See [the definition](https://github.com/oh-lang/oh/blob/main/core/array.oh) for
more details.

TODO: for transpiling to javascript, do we want to use the standard javascript `Array`
as an internal field or do we want to `@hide` it as the base class for the oh-lang `array_`?
i.e., we create oh-lang methods that are aliases for operations on the JS `Array` class?
it depends on if we want other JS libraries to take advantage of oh-lang features or if
we want to make it look as native as possible -- for less indirection.

### vectors

oh-lang has the concept of a fixed-length array (or fixed-size array),
but we call it a vector because it has different semantics than a resizable array.
Vectors have all their elements initialized to the default value (e.g., 0 for
number types) and will always have the same size/count.

A vector has this declaration:
`vector_{of_: element_type_, count:, count_: select_count_ = arch_ count_}`,
and so can be instantiated like this: `vector_{5, int_}`.  Example usage:

```
# this function requires being called with comptime-known argument
# because the return type depends on it.
count_up_(@comptime ~count.): vector_{int_, count}
     result; return_
     count each index: count_
          result[index] = int_(index)
     result

print_(count_up_(10))    # prints [0,1,2,3,4,5,6,7,8,9]
```

TODO: we probably need the concept of a slice like in Rust; vectors and arrays
can be converted to readonly slice references without being copied.  

## lots 

A `lot_` is oh-lang's version of a map (or `dict` in python).  Instead of
"mapping" from a `key_` to a `value_` type, lots locate an `of_` at an `at_`.
This change from convention is mostly to avoid overloading the term `map`
which is used when transforming values such as `hm_`, but also because
`map_`, `key_`, and `value_` have little to do with each other; we don't "unlock"
anything with a C++ `map`'s key, we locate where an instance is at.  Thus
we use `at` for a locator and `of` because it's the default-named type for
a generic.  The class definition for a `lot_` is
[here](https://github.com/oh-lang/oh/blob/main/core/lot.oh).

A lot can look up, insert, and delete elements by key quickly (ideally amortized
at `O(1)` or at worst `O(lg(N)`).  You can use this way to define a lot, e.g.,
`variable_name: lot_{at_: id_type_, of_: value_type_}`, e.g.,
`my_var: lot_{at_: str_, int_}` which declares a lot of integers located at strings.
A default-named lot can be defined via `lot{at_: id_type_, value_type_};`, e.g.,
`lot{dbl, at: int};`.  Note that while an array can be thought of as a lot with the
`at_` type as `index_`, the array type `array_{element_type_}` is most useful for
densely packed data (i.e., instances of `element_type_` for most indices), while
the lot  type `lot_{element_type_, at_: index_}` would be useful for sparse data.

To define a lot (and its contents) inline, use this notation:

```
jim1: "Jim C"
jim2: "Jim D"
jim: 456
# lot linking string to ints:
employee_ids_: lot_{at_: int_, str_}
(    # option 1.A: `x: y` syntax
     "Jane": 123
     # option 1.B: `[at: x, of: y]` syntax
     [at: "Jane", of: 123]
     # option 1.C: `[x, y]` syntax
     ["jane", 123]
     # if you have some variables to define your `at`, you need to take care.
     # option 2.A, wrap in braces to indicate it's a variable not an ID
     {jim1}: 203
     # option 2.B
     [at: jim1, of: 203]
     # option 2.C
     [jim1, 203]
)
# note that commas are optional if elements are separated by newlines,
# but required if elements are placed on the same line.
```

To define a lot quickly (i.e., without a type annotation), use the notation
`["Jane": 123, "Jim": 456]`.

Lots require an `at_` type whose instances can hash to an integer or string-like value.
E.g., `dbl_` and `flt_` cannot be used, nor can container types which include those
(e.g., `array_{dbl_}`).  However, standard container types with hashable elements are
likewise hashable and can thus be used as `at_` types.

Note: when used as a lot `at_`, types with nested fields become deeply constant,
regardless of whether the internal fields were defined with `;` or `:`.
I.e., the object is defined as if with a `:`.  This is because we need `at`
stability inside a container; we're not allowed to change the `at` or it could
change places inside the lot and/or collide with an existing `at`.  Such
manipulations are possible but should be done explicitly and only intentionally.

The default lot type is `insertion_ordered_lot_`, which means that the order of elements
is preserved based on insertion; i.e., new elements come after old elements when iterating.
(Equality checking doesn't care about insertion order, however.)
Other notable lots include `at_ordered_lot_`, which will iterate over elements in order
of their sorted locations, and `unordered_lot_`, which has an unpredictable iteration order.
Note that `at_ordered_lot_` has `O(lg(N))` complexity for look up, insert, and delete,
while `insertion_ordered_lot_` has some extra overhead but is `O(1)` for these operations,
like `unordered_lot_`.

## sets

A set contains some elements, and makes checking for the existence of an element within
fast, i.e., O(1).  Like with container `at`s, the set's element type must satisfy certain
properties (i.e., hashable, e.g., integer/string-like).  The syntax to define a set is
`variable_name: set_{element_type_}`.  You can elide `set_` for default named arguments
like this: `set{element_type_};` (or `:` or `.`).  See
[the set definition here](https://github.com/oh-lang/oh/blob/main/core/set.oh).

Like the `at`s in lots, items added to a set become deeply constant, even if the set
variable is writable.  The default type of a set is an `insertion_ordered_set_`,
because oh-lang makes insertion-ordered containers the default.  For equality checking,
insertion order doesn't matter, but for iteration it does.

TODO: discuss how `in` sounds like just one key from the set of IDs (e.g., `k_ in ats_(o_)`)
and `from` selects multiple (or no) IDs from the set (`k_ from ats_(o_)`).

## iterator

For reference, see [the definition here](https://github.com/oh-lang/oh/blob/main/core/iterator.oh).
For example, here is a way to create an iterator over some incrementing values:

```
my_range_{of_: number_}: all_of_
{    @private m:
     [    next_value; of_ = 0
          less_than: of_
     ]
     iterator{of_};
}
{    ;;renew_(start_at. of_ = 0, m less_than. of_ = 0): null_
          m next_value = start_at

     ;;next_()?: of_
          if m next_value < m less_than
               m next_value++
          else
               null

     ::peak_()?: of_
          if m next_value < m less_than
               m next_value 
          else
               null
}

my_range_(less_than: index_(10)) each index:
     print_(index)
# prints "0" to "9"
```

We want to avoid pointers where possible, so iterators should just be indices
into the container that work with the container to advance, peak, etc.
Thus, we need to call `iterator next_` with the container to retrieve
the element and advance the iterator, e.g.:

```
array: [1, 2, 3]
iterator; iterator_{int_}
assert(iterator next_(array)) == 1
assert_(next_(array, iterator)) == 2    # you can use global `next_`
assert_(iterator::peak_(array)) == 3
assert_(peak(iterator, array)) == 3     # you can use global `peak_`
assert_(iterator next_(array)) == 3
assert_(iterator next_(array)) == null
assert_(iterator peak_(array)) == null
# etc.
```

The way we achieve that is through using an array iterator:

```
# by requesting the `next_()` value of an array with this generic iterator,
# the iterator will become an array iterator.  this allows us to check for
# `@only` annotations (e.g., if `iterator` was not allowed to change) and
# throw a compile error.
next_{t_}(iterator{t_}; @becomes array_iterator_{t_}, array{~t_}:)?: t_
     iterator = array_iterator_{t_}()
     iterator;;next_(array)
```

Where [the array iterator is defined here](https://github.com/oh-lang/oh/blob/main/core/array/iterator.oh).

We can also directly define iterators on the container itself.
We don't need to define both the `iterator_` version and the `each_` version;
the compiler can infer one from the other.  We write the `each_` option
as a method called `each_`.
TODO: is this true, can we really infer?

```
array_{of_}: []
{    # TODO: decide between blockable and function approach:
     # function approach doesn't contain any block information (e.g., what was `break`ed)
     ;:each_(fn_(of;:): loop_): bool_
          m count_() each index:
               if fn_(m[index];:) is_break_()
                    return: true
          false

     .;:each_(each_blockable{declaring_: (of.;:), ~t_}): t_
          m count_() each index:
               each_blockable then_(.;:m[index])
          each_blockable else_()
}

x: [1, 2, 3, 4] each int.
     if int > 3
          print_("choosing $(int)")
          break: "$(int)"
     else
          print_("ignoring $(int)")
else
     print_("found no good matches")
     ""

# becomes something like
each_blockable{declaring_: (int.), int_}: _
(    each_(int.): bc_{str_}
          if int > 3
               print_("choosing $(int)")
               bc_ break_("$(int)")
          else
               print_("ignoring $(int)")
          bc_ continue
     else_(): str_
          print_("found no good matches")
          ""
)
```

# standard flow constructs / flow control

We have a few standard control statements or flow control keywords in oh-lang.

TODO -- `return`
TODO: more discussion about how `return` works vs. putting a RHS statement on a line.
TODO -- description, plus `if/else/elif` section

Conditional statements including `if`, `elif`, `else`, as well as `what`,
can act as expressions and return values to the wider scope.  This obviates the need
for ternary operators (like `x = do_something_() if condition else default_value` in python
which inverts the control flow, or `int x = condition ? do_something() : default_value;`
in C/C++ which takes up two symbols `?` and `:`).  In oh-lang, we borrow from Kotlin that
[`if` is an expression](https://kotlinlang.org/docs/control-flow.html#if-expression),
and similarly for `what` statements (similar to
[`when` in kotlin](https://kotlinlang.org/docs/control-flow.html#when-expressions-and-statements)).

## with statements

`with x_()` is just a fancy way to say that we want to descope the value of `x_()`
after the statement.  The statement usually includes an indented block after
the `with`.  You can also declare a variable (that will get descoped).

```
with file; open_some_file_()
     print_(file read_())
     print_("ok")
```

## then statements

We can rewrite conditionals to accept an additional `then` "argument".  For `if`/`elif`
statements, the syntax is `if expression -> then:` to have the compiler infer the `then`'s
return type, or `elif expression -> whatever_name: then_{whatever_type_}` to explicitly
provide it and also use `whatever_name` for the `then_`'s name.  Similarly for `what
statements, e.g., `what expression -> whatever_name: then_{whatever_}` or
`what expression -> then:`.  `else` statements also use the `->` expression, e.g.,
`else -> then:` or `else -> whatever: then_{else_type_}`.  Note that we use a `:` here
because we're declaring an instance of `then_`; if we don't use `then` logic we don't use
`:` for conditionals.  Also note that `then_` is a thin wrapper around the
[`block_` class](#blocks) (i.e., a reference that removes the `::loop_()` method that
doesn't make sense for a `then_`).  If you want to just give the type without renaming,
you can do `if whatever -> then{my_if_block_type_}:`.

```
if some_condition -> then:
     # do stuff
     if some_other_condition -> SOME_NAMESPACE_then:
          if something_else1
               then exit_()
          if something_else2
               SOME_NAMESPACE_then exit_()
     # do other stuff

result: what some_value -> then{str_}:
     5
          ...
          if other_condition
               then exit_("Early return for `what`")
          ...
     ...

# if you are running out of space, try using parentheses.
if
(       some long condition
     &&  some other_fact
     &&  need_this too_()
) -> then:
     print_("good")
     ...

# of you can just use double indents:
if some long condition
     &&   some other_fact
     &&   need_this too_()
->        then:
     print_("good")
     ...
```

## if statements

TODO: we can get some nice alignment with 5-tab spaces using `when` instead of `if`,
and maybe we even just keep using `when` instead of an `elif`.  e.g.,
```
when condition
     do_something_()
when other_condition
     do_other_thing_()
else
     do_something_else_()
```
HOWEVER we do need to handle the case if something like `if x {print_("asdf")}`
followed by another `if y {print_("asdf2")}`, which with `when`-`when` would only
execute the second statement if `!x`, whereas the first one ignores the value of `x`.
maybe something like `also`.

```
if x
     print_("x was truthy")
if y
     print_("y was truthy")

# would become
when x
     print_("x was truthy")
also
when y
     print_("y was truthy")
```

not sure i love this solution as `when` doesn't have a `elif` feel to me.
`when` could be a replacement for `if` but probably not `elif`.  but i do
like `also` being explicit...

```
x: if condition
     do_something_()
elif other_condition
     do_something_else_()
else
     calculate_side_effects_(...)    # ignored for setting x
     default_value

# now `x` is either the result of `do_something_()`, `do_something_else_()`,
# or `default_value`.  note, we can also do this with braces to indicate
# blocks, and can fit comfortably in one line if we have fewer conditions, e.g.,

y: if condition {do_something_()} else {calculate_side_effects_(...), default_value}
```

Note that ternary logic short-circuits operations, so that calling the function
`do_something_()` only occurs if `condition` is true.  Also, only the last line
of a block can become the RHS value for a statement like this.

Of course you can get two values out of a conditional expression, e.g., via destructuring:

```
[x, y]: if condition
     [x: 3, y: do_something_()]
else
     [x: 1, y: default_value]
```

Note that indent matters quite a bit here.  Conditional blocks are supposed to indent
at +1 from the initial condition (e.g., `if` or `else`), but the +1 is measured from
the line which starts the conditional (e.g., `[x, y]` in the previous example).  Indenting
more than this would trigger line continuation logic.  I.e., at +2 or more indent,
the next line is considered part of the original statement and not a block.  For example:

```
# WARNING, PROBABLY NOT WHAT YOU WANT:
q?: if condition
          what + indent_twice
# actually looks to the compiler like:
q?: if condition what + indent_twice
```

Which will give a compiler error since there is no internal block for the `if` statement.

### if without else

You can use the result of an `if` expression without an `else`, but the resulting
variable becomes nullable, and therefore must be defined with `?:` (or `?;`).

```
greet_(): str_
     "hello, world!"

result?: if condition { greet_() }
```

This also happens with `elif`, as long as there is no final `else` statement.


### is operator

You can use the `is` operator to convert statements like `x is_(an_(another_type: ...): ...)`
into more idiomatic things like `if x is another_type: ...`.

```
# not idiomatic:
my_decider_(x: one_of_{type1:, type2:}):
     x is_
     (    a_(type1:):
               print_("x was type1: ", type1)
     )
     # or using lambda functions:
     x is_({print_("x was type2: ", $type2)})

# idiomatic:
my_decider_(x: one_of_{type1:, type2:}):
     if x is type1:
          print("x was type1: ", type1)
     elif x is type2:
          print("x was type2: ", type2)
```

This is how you might declare similar functionality for your own class,
by overloading the `is` operator.

```
example_class_: [value: int_]
{    #[# the standard way to use this method uses syntax sugar:
          ```
          if example_class is large:
               print_("was large: $(large)")
          ```
     #]#
     :;.is_(if_block{declaring_: (large:;. int_), ~t_}): t_
          if m value > 999
               if_block then_(declaring: (large:;. m value))
          else
               if_block else_()
}
```

### has operator

TODO: include a `has` operator (like `is`) operator.  discuss for arrays
and lots.  maybe instead of `container` do `hasable`.

Similar to the `is` operator, we can define a `has_` method on a class
and then automatically get a `has` operation like this.

```
pair_class: [value1: int_, value2: int_]
{    #[# the standard way to use this method uses syntax sugar:
          ```
          if pair_class has 123
               print_("had a 123 internally")
          ```
     #]#
     :;.has_(int:, if_block{~t_}): t_
          if m value1 == int || m value2 == int
               if_block then_()
          else
               if_block else_()
}
```

## what statements

`what` statements are comparable to `switch-case` statements in C/C++,
but in oh-lang the `case` keyword is not required.  You can use the keyword
`else` for a case that is not matched by any others, i.e., the default case.
You can also use `any:;.` to match any other case, if you want access to the
remaining values.  (`else` is therefore like an `_any:` case.)
We switch from the standard terminology for two reasons: (1) even though
`switch x` does describe that the later logic will branch between the different
cases of what `x` could be, `what x` is more descriptive as to checking what `x` is,
and (2) `switch` is something that a class instance might potentially want to do,
e.g., `my_instance switch_(background1)`, and having `switch` as a keyword negates
that possibility.

TODO: explain how `case` values are cast to the same type as the value being `what`-ed.

You can use RHS expressions for the last line of each block to return a value
to the original scope.  In this example, `x` can become 5, 7, 8, or 100, with various
side effects (i.e., printing).  Note that we don't use `:` or `;` to define the cases
here because we are not declaring any new variables.

```
x: what string
     "hello"
          print_("hello to you, too!")
          5
     # you can do multiple matches over multiple lines:
     "world"
     "earth"
          # `string == "world"` or "earth" here.
          print_("it's a big place")
          7
     # or you can do multiple matches in a single line with commas:
     "hi", "hey", "howdy"
          # `string == "hi"`, "hey", or "howdy" here.
          print_("err, hi.")
          8
     else
          100

# Note again that you can use braces to make these inline.
# Try to keep usage to code that can fit legibly on one line:
y: what string { "hello" {5}, "world" {7}, else {100} }
```

You don't need to explicitly "break" a `case` statement like in C/C++.
Because of that, a `break` inside a `what` statement will break out of
any enclosing `for` or `while` loop.  This makes `what` statements more
like `if` statements in oh-lang.

```
air_quality_forecast: ["good", "bad", "really bad", "bad", "ok"]
meh_days; 0
air_quality_forecast each quality: str_
     what quality
          "really bad"
               print_("it's going to be really bad!")
               break    # stops `for` loop, might not be what you want!
          "good"
               print_("good, that's good!")
          "bad"
               print_("oh no")
          "ok"
               ++meh_days
```

The `what` operation is also useful for narrowing in on `one_of_` variable types.
E.g., suppose we have the following:

```
status_: one_of_{unknown:, alive:, dead:}
vector3_: [x; dbl_, y; dbl_, z; dbl_]
{    ::length_(): sqrt_(x^2 + y^2 + z^2)
}

update_: one_of_
{    status:
     position: vector3_
     velocity: vector3_
}
# example usage of creating various `update`s:
update0: update_ status_ alive
update1: update_ position_(x: 5.0, y: 7.0, z: 3.0)
update2: update_ velocity_(x: -3.0, y: 4.0, z: -1.0)
```

We can determine what the instance is internally by using `what` with
variable declarations that match the `one_of_` type and field name.
We can do this alongside standard constant values, like so,
with earlier `what` cases taking precedence.  

```
...
# checking what `update` is:
what update
     # no trailing `:` because we're not declaring anything here:
     status_ unknown
          print_("unknown update")
     status:
          # match all other statuses:
          print_("got status update: $(status)")
     position: vector3_
          print_("got position update: $(position)")
     velocity: vector3_
          print_("got velocity update: $(velocity)")
```

You can use a `then` on the `what` itself or on a specific case that's complicated,
or `where` to further restrict the scope of a matching case.

```
speed_: one_of_
{    none:
     slow:
     going_up:
     going_down:
     going_sideways:
     dead:
}
# here's an example with a `then` that works for all cases.
# note we are declaring a `then` so we need a `:`
speed1: speed_ = what update -> then{speed_}:
     velocity: vector3_
          if Velocity length() < 5.0
               then exit_(slow)
          print_("going slow, checking up/down")
          then exit_
          (    if velocity y abs_() < 0.2
                         going_sideways
               elif velocity y > 0.0
                         going_up
               else
                         going_down
          )
     else
          then exit_(none)

# here's an example with a `then` that applies to only one case.
speed2: speed_ = what update
     status dead
          dead
     velocity: vector3_ -> then{speed_}:
          if Velocity length() < 5.0
               then exit_(slow)
          print_("going slow, checking up/down")
          # TODO: i think we can assume that this exits the `then` here:
          if velocity y abs_() < 0.2
               going_sideways
          elif velocity y > 0.0
               going_up
          else
               going_down
     position: where position length_() is_nan_()
          dead
     else
          none
```

Note that variable declarations can be argument style, i.e., including
temporary declarations (`.`), readonly references (`:`), and writable
references (`;`), since we can avoid copies if we wish.  This is only
really useful for allocated values like `str`, `int`, etc.  However, note
that temporary declarations via `.` can only be used if the argument to
`what` is a temporary, e.g., `what my_value!` or `what some_class value_()`.
There is no need to pass a value as a mutable reference, e.g., `what my_value;`;
since we can infer this if any internal matching block uses `;`.

```
whatever_: one_of_
{    str:
     card: [name: str_, id: u64_]
}

whatever; whatever_ str_("this could be a very long string, don't copy if you don't need to")

what whatever!      # ensure passing as a temporary by mooting here.
     str.
          print_("can do something with temporary here: $(str)")
          do_something_(str!)
     card.
          print_("can do something with temporary here: $(card)")
          do_something_else_(card!)
```

TODO: do we even really want a `fall_through` keyword?  it makes it complicated that it
will essentially be a `goto` because fall through won't work due to the check for string
equality.  probably not, because it might not play well with declared values (like `str.`
and `card.` above).

### restrictions for what

Any class that supports a compile-time fast hash with a salt can be
put into a `what` statement.  Floating point classes or containers thereof
(e.g., `dbl_` or `array_{flt_}`) are not considered *exact* enough to be hashable, but
oh-lang will support fast hashes for classes like `int_`, `i32_`, and `array_{u64_}`,
and other containers of precise types, as well as recursive containers thereof.

```
# note it's not strictly necessary to mention you implement `hashable_`
# if you have the correct `hash` method signature.
my_hashable_class_: all_of_{hashable;, m: [id: u64_, name; str_]}
{    # we allow a generic hash builder so we can do cryptographically secure hashes
     # or fast hashes in one definition, depending on what is required.
     # This should automatically be defined for classes with precise fields
     # (e.g., int, u32, string, etc.)!
     ::hash_(~builder;): null_
          builder hash_(m id)      # you can use `hash_` via the builder or...
          m name hash_(;builder)   # you can use `hash_` via the field.

     # equivalent definition via sequence building:
     ::hash_(~builder;): builder@
     {    hash_(m id)
          hash_(m name)
     }
}

# note that defining `::hash_(~builder;)` automatically defines a `fast_hash_` like this:
# fast_hash_(my_hashable_class:, salt. unsigned_ arch_): unsigned_ arch_
#    builder; \\hash fast_(salt)
#    builder hash_(my_hashable_class)
#    builder build_()

my_hashable_class: my_hashable_class_(id. 123, name. "Whatever")

what my_hashable_class
     my_hashable_class_(id. 5, name. "Ok")
          print_("5 OK")
     my_hashable_class_(id. 123, name. "Whatever")
          print_("great!")
     my_hashable_class:
          print_("it was something else: $(my_hashable_class)")
```

Note that if your `fast_hash_` implementation is terrible (e.g., `fast_hash_(salt): salt`),
then the compiler will error out after a certain number of attempts with different salts.

For sets and lots, we use a hash method that is order-independent (even if the container
is insertion-ordered).  E.g., we can sum the hash codes of each element, or `xor` them.
Arrays have order-dependent hashes, since `[1, 2]` should be considered different than 
`[2, 1]`, but the lot `["hi": 1, "hey": 2]` should be the same as `["hey": 2, "hi": 1]`
(different insertion order, but same contents).

### where operator

TODO: could we just use `if` or `when` here?  i like how `where` reads, though,
very mathematically.  also it might be required to have a different keyword
for the specialized versions of methods, like `::do_this_(): hm_{i32_}`
and `::do_this_(): i32_ where !!m`, which will never throw.

The `where` operator can be used to further narrow a conditional.  It
is typically used in a `what` statement like this:

```
cows_: one_of_
{    one:
     two:
     many: i32_
}
cows: some_function_returning_cows_()
what cows
     one
          print_("got one cow")
     two
          print_("got two cows")
     many: where many <= 5       # optionally `many: i32_ where many <= 5`
          print_("got a handful of cows")
     many:                       # optionally `many: i32`
          print_("got $(many) cows")
```

It can also be used in a conditional alongside the `is` operator.
Using the same `cows` definition from above for an example:

```
cows: some_function_returning_cows_()
if cows is many: where many > 5
     # executes if `cows` is `cows_ many` and `many` is 6 or more.
     print_("got $(many) cows")
else
     # executes if `cows` is something else.
     print_("not a lot of cows")
```

`where` is similar to the [`require`](#require) field, but
`require` needs to be computable at compile-time, and `where`
can be computed at run-time.

TODO: do we really need `where`?  what's wrong with `and`?
`if cows is many: and many > 5`.  is the order of operations bad?
or do we want to make sure people don't do weird things like
`if cows is many: or other_condition_()`?  but i suppose
we could do something like `if other_condition_() or cows is many:`;
we probably can fix by using `or cows is many?:` to indicate `many`
might not be defined.

### what operator overloading

TODO:
Can we overload the `what` operator?  oh-lang would like to avoid walling off
parts of the code that you can't touch.  We'd need to figure out a good syntax
here.

```
my_vec2_: [x; dbl_, y; dbl_]
{    ::what_
     (    do_(quadrant_i. dbl_): ~a_
          do_(quadrant_ii. dbl_): ~b_
          do_(quadrant_iii. dbl_): ~c_
          do_(quadrant_iv. dbl_): ~d_
          else_(): ~e_
     ): flatten_{a_, b_, c_, d_, e_}
          if x == 0.0 or y == 0.0
               else_()
          elif x > 0.0
               if y > 0.0
                    do_(quadrant_i. +x + y)
               else
                    do_(quadrant_iv. +x - y)
          else
               if y > 0.0
                    do_(quadrant_ii. -x + y)
               else
                    do_(quadrant_iii. -x - y)
}
```

## for-each loops

TODO: Can we write other conditionals/loops/etc. in terms of `indent/block` to make it easier to compile
from fewer primitives?  E.g., `while condition -> do: {... do exit_(3) ...}`, where
`do` is a thin wrapper over `block`?  or maybe `do -> loop: {... loop exit_(3) ...}`

oh-lang doesn't have `for` loops but instead uses the syntax `each` on an iterator.
The usual syntax is `iterator each iterand;:. {do_something_(iterand)}`.  If your
iterand variable is already defined, you should use `iterator each iterand {...}`.
Note that all container classes have an `each` method defined, and some
"primitive" classes like the `count` class do as well.

```
# iterating from 0 to `count - 1`:
count: 5
count each int:
     print_(int)    # prints 0 to 4 on successive lines

# iterating over a range:
range_(1, 10) each int:
     print_(int)    # prints 1 to 9 on successive lines.

# iterating over non-number elements:
vector2_: [x: dbl_, y: dbl_]
array{vector2_}: [[x: 5, y: 3], [x: 10, y: 17]]

# should print `[x: 5, y: 3]` then `[x: 10, y: 17]`.
array each vector2:
     print_(vector2)

# if the variable is already declared, you avoid the declaration `:` or `;`:
# NOTE the variable should be writable!
iterating_vector; vector2_
array each iterating_vector
     print_(iterating_vector)
# this is useful if you want to keep the result of the last element outside the loop.
```

You can get the result of an `each` operation but this only really
makes sense if the `each` block has a `break` command in it.
Like `return`, `break` can pass a value back.

```
# result needs to be nullable in case the iteration doesn't break anything.
result?: range_(123) each int:
     if int == 120
          break: int

# you can use an `else` which will fire if the iterator doesn't have
# any values *or* if the iteration never hit a `break` command.
# in this case, `result` can be non-null.
result: range_(123) each int:
     if int == 137
          break_(int)    # also equivalent: `break: int`
else
     44
```

Of course, you can use the `else` block even if you don't capture a result.

```
count_(123) each int:
     print_(int)
     if int == 500
          break
else
     print_("only fires if `break` never occurs")
```

Here are some examples of iterating over a container while mutating some values.

```
a_array; array_{int_} = [1, 2, 3, 4]
# this is clearly a reference since we have `int` in parentheses, `(int;)`:
a_array each (index, int;)
     int += index
a_array == [1, 3, 5, 7] # should be true

b_array; array_{int_} = [10, 20, 30]
b_array each (int;)
     int += 1
b_array == [11, 21, 31] # should be true

c_array; array_{int_} = [88, 99, 110]
start_referent; int_ = 77
(iterand_value;) = start_referent
c_array each iterand_value
     iterand_value -= 40
c_array == [48, 59, 70] # should be true
iterand_value += 100
c_array == [48, 59, 170] # should be true
```

You should be careful not to assume that `;` (or `:`) means a reference
unless the RHS of the `each` is wrapped in parentheses.
TODO: revisit this logic.  we probably want to do references to avoid
copies in most places, unless we're explicitly copying.
but it does kinda make sense for the value that escapes the loop.
let's try to figure out how to make the copy explicit.

```
b_array; array_{int_} = [10, 20, 30]
# WARNING! this is not the same as the previous `b_array` logic.
b_array each int;
     # NOTE: `int` is a mutable copy of each value of `b_array` element.
     int += 1
     # TODO: we probably should have a compile error here since
     #       `int` is not used to affect anything else and
     #       `int;` here is *NOT* a reference to the `c_array` elements.
     #       e.g., "use (int;) if you want to modify the elements"
b_array == [11, 21, 31] # FALSE

c_array; array_{int_} = [88, 99, 110]
iterand_value; 77 
c_array each iterand_value
     # NOTE: `iterand_value` is a mutable copy of each `c_array` element.
     iterand_value -= 40
c_array == [48, 59, 70] # FALSE
c_array == [88, 99, 110] # true, unchanged.
iterand_value == 70 # true
```

TODO: there may be some internal inconsistency here.  let's make sure
the way we define `;;each_(...)` makes sense for the non-parentheses
case and the parentheses case.

TODO: if we require parentheses for references here,
we need to discuss this for `if result is (ok;)`, etc.,
vs. `if result is ok;`.  The latter is a copy, the former is a no-copy reference.

# printing and echoing output

TODO: allow tabbed print output.  instead of searching through each string,
maybe we just look at `print_` and add the newlines at the start.  Each thread should
have its own tab stop.  E.g.,

```
array_{of_:}: []
{    ...
     ::print_(): null_
          if m count_() == 0
               return: print_("[]")
          print_("[")
          with print_ indent_()
               m each of:
                    print_(of)
          print("]")
}
```

TODO: defining `print_` on a class will also define the `string_()` method.
essentially any `print_`s called inside of the class `print_` will be redirectable to
a string-stream, etc.  `print_ indent_` should maybe do something different for `string_()`;
maybe just add commas *after* the line instead of spaces before the line to be printed.

TODO: we should also have a print macro here in case we want to stop printing,
e.g., in case the string buffer is full (e.g., limited output).  if the print
command triggers a stop at any point, then abort (and stop calling the method)

## blocks

You can write your own `assert` or `return`-like statements using `block` logic.  The `block_`
class has a method to return early if desired.  Calling `block exit_(...)` shortcircuits the
rest of the block (and possibly other nested blocks).  This is annotated by using the `jump_`
return value.  You can also call `block loop_()` to return to the start of the block.
You don't usually create a `block` instance; you'll use it in combination with the global
`indent_` function.

```
# indent function which returns whatever value the `block` exits the loop with.
indent_(do_(block{~t_};): never_): t_
# indent function which populates `block declaring` with the value passed in.
indent_(~declaring;:., do_(block{~t_, (declaring;:.)};): never_): t_

@referenceable_as(then_)
block_{of_:, declaring_: any_ = null_}:
[    # variables defined only for the lifetime of this block's scope.
     # TODO: give examples, or maybe remove, if this breaks cleanup with the `jump` ability
     # TODO: if `declaring_` is nullable, then we also have a problem here, would need
     declaring{require: declaring_ is !null_}`
     declaring{require: declaring_ is nullable_}?`
]
{    # exits the `indent_` with the corresponding `of_` value.  example:
     #    value; 0
     #    what indent_
     #    (    do_(block{str_};): never_
     #              OLD_value: value
     #              value = value // 2 + 9
     #              # sequence should be: 0, 9, 4+9=13, 6+9=15, 7+9=16, 8+9=17
     #              if OLD_value == value
     #                   block exit_("exited at $(OLD_value)")
     #              # note we need to `loop_` otherwise we won't satisfy the `never_`
     #              # part of the indent function.
     #              # TODO: i don't know if `loop_` makes sense for `block.
     #              # how would `block` know to restart this `do_` function??
     #              block loop_()
     #    )
     #         str.
     #              print_(str)    # should print "exited at 17"
     # TODO: should this be `never_` instead of `jump_`??
     ::exit_(of.): jump_

     # like a `continue` statement; will bring control flow back to
     # the start of the `indent` block.  example:
     #    value; 0
     #    indent
     #    (    do_(block{str_};): never_
     #              if ++value >= 10 {block exit_("done")}
     #              if value % 2
     #                   block loop_()
     #              print_(value)
     #              block loop()
     #   )
     #   # should print "2", "4", "6", "8"
     @hide_from(then_)
     ::loop_(): jump_
}
```

### blocks to define a variable

```
my_int: indent_
(    block{int_}:
          if some_condition_()
               block exit_(3)
          block loop_()
)
```

### then with blocks

When using `then_`, it's recommended to always exit explicitly, but like with the
non-`then_` version, the conditional block will exit with the value of the last
executed line.  There is a rawer version of this syntax that does require an
explicit exit, but also doesn't allow any `return` functions since we are literally
defining a `(then;): never_` with its block inline.  This syntax is not recommended
unless you have identical block handling in separate conditional branches, but
even that probably could be better served by pulling out a function to call in
both blocks.

```
if some_condition -> then{str_}:
     if other_condition
          if nested_condition
               then exit_(x)
     else
          then exit_("whatever")
     # COMPILE ERROR, this function returns here if
     # `other_condition && !nested_condition`.
     # needs a non-null exit value since we should return a `str_`.

# here's an example where we re-use a function for the block.
my_then: then_{str_}
     ... complicated logic ...
     # TODO: this syntax seems suspect, shouldn't it be `then exit_`?
     # or maybe this should just be a function.
     # e.g., `my_then: then_{str_}({ ... complicated logic ..., "made it" })`
     exit_("made it")

result: if some_condition -> my_then
elif some_thing_else
     print_("don't use `my_then` here")
     "no"
else -> my_then
```

### function blocks

Similar to conditionals, we allow defining functions with `block` in order
to allow low-level flow control.  Declarations like `my_function_(x: int_): str_`,
however, will be equivalent to `my_function_(x: int_, block{str_}:): never_`.  Thus
there is no way to overload a function defined with `block` statements compared
to one that is not defined explicitly with `block`.

```
# the `never_` return type means that this function can't use `return`, either
# explicitly via `return: ...` or implicitly by leaving a value as the last
# evaluated statement (which can occur if you don't use `block exit_(...)`
# or `block loop_(...)` on the last line of the function block).
# i.e., you must use `block exit_(...)` to return a value from this function.
my_function_(x: int_, block{str_};): never_
     inner_function_(y: int_): dbl_
          if y == 123
               block exit_("123")  # early return from `my_function_`
          y dbl_() ?? panic_()
     range_(x) each y:
          inner_function_(y)
     block exit_("normal exit")
```

## coroutines

We'll reserve `co_{of_}` for a coroutine for now, but I think futures are all
that is necessary.

# futures

oh-lang wants to make it very simple to do async code, without additional
metadata on functions like `async` (JavaScript).  You can indicate that
your function takes a long time to run by returning the `um_{of_}` type,
where `of_` is the type that the future `um_` will resolve to, but callers
will not be required to acknowledge this.  If you define some overload
`my_overload_(x: str_): um_{int_}`, an overload `my_overload_(x: str_): int_`
will be defined for you that comes *before* your async definition, so that
the default type of `value` in `value: my_overload_(x: "asdf")` is `int_`.
We generally recommend a timeout `er_` being present, however, so for
convenience, we define `um_{of_:, er_:}: um_{hm_{ok_: of_, :er_}}`.

The reason we obscure futures in this way is to avoid needing to change any
nested function's signatures to return futures if an inner function becomes
a future.  If the caller wants to treat a function as a future, i.e., to run
many such futures in parallel, then they ask for it explicitly as a future
by calling a function `f_()` via `f_() um` or `um: f_()`.  You can also type
the variable explicitly as `um_{of_}`, e.g., `f: um_{of_} = f_()`.  Note that
`f: um_(f_())` is a compile error because casting to a future would still run
`f_()` serially.  You can use `f: um_(immediate: 123)` to create an "immediate
future"; `f: um_(immediate: h_())` similarly will run `h_()` serially and put
its result into the immediate future.  If `h_` takes a long time to run, prefer
`f: h_() um` of course.

```
# you don't even need to type your function as `um_{~}`, but it's highly recommended:
some_very_long_running_function_(int): um_{string_}
     result; ""
     int each COUNT_int:
          sleep_(seconds: COUNT_int)
          result += str_(COUNT_int)
     result

# this approach calls the default `string_` return overload, which blocks:
print_("starting a long running function...")
my_name: some_very_long_running_function_(10)
print_("the result is $(my_name) many seconds later")

# this approach calls the function as a future:
print_("starting a future, won't make progress unless polled")
# `future` here has the type `um_{string_}`:
future: some_very_long_running_function_(10) um
# OR: `future: um_{string_} = some_very_long_running_function_(10)`
# OR: `future: um_{~} = some_very_long_running_function_(10)` (infers the inner type)
# OR: `future: um_{~inner_} = some_very_long_running_function_(10)` (infers inner type and gives it a name)
# which is useful if you want to use the `inner_` type later in this block.
print_("this `print` executes right away")
result: string_ = future
print_("the result is $(result) many seconds later")
```

That is the basic way to resolve a future, but you can also use
the `::decide_(): of_` method for an explicit conversion from `um_{of_}`
to `of_`.  Ultimately futures are most useful when combined for
parallelism.  Here are two examples, one using an array of futures
and one using an object of futures:

```
# you don't even need to type your function as `um_{~}`, but it's highly recommended:
after_(seconds: int_, resolve: string_): um_{string_}
     sleep_(seconds)
     resolve

futures_array; array_{um_{string_}}
# no need to use `after_(...) um` here since `futures_array`
# elements are already typed as `um_{string_}`:
futures_array append_(after_(seconds: 2, resolve: "hello"))
futures_array append_(after_(seconds: 1, resolve: "world"))
print_("this executes immediately.  deciding futures now...")
results_array: decide_(futures_array)
print_(results_array)   # prints `["hello", "world"]` after 2ish seconds.

# here we put them all in at once.
# notice you can use `field: um_{type_} = fn_()` or `field: fn_() um`.
futures_object:
[    greeting: after_(seconds: 2, resolve: "hello") um
     noun: um_{string_} = after_(seconds: 1, resolve: "world")
]
print_(decide_(futures_object)) # prints `[greeting: "hello", noun: "world"]`

# if your field types are already futures, you don't need to be
# explicit with `um`.
future_type_: [greeting: um_{str_}, noun: um_{str_}]
# note that we need to explicitly type this via `the_type_(args...)`
# so that the compiler knows that the arguments are futures and should
# receive the `um_` overload.
futures: future_type_
(    greeting: after_(seconds: 2, resolve: "hi")
     noun: after_(seconds: 1, resolve: "you")
)
# this whole statement should take ~2s and not ~3s; the two fields are
# initialized in parallel.
futures decide_() print_()  # prints `[greeting: "hi", noun: "you"]`
```

Notice that all containers with `um_` types for elements will have
an overload defined for `decide_`, which can be used like with the
`futures_array` example above.  Similarly all object types with `um_`
fields have a `decide_` function that awaits all internal fields that
are futures before returning.  You can also use `container decide_()`
instead of `decide_(container)` in case that makes more sense.

We will also include a compile error if something inside a futures
container is defined without `um`:

```
# if any field in an object/container is an `um_` class, we expect everyone to be.
# this is to save developers from accidentally forgetting an `um_`
object_or_container:
[    greeting: after_(seconds: 2, resolve: "hello")    # COMPILE ERROR!
     noun: after_(seconds: 1, resolve: "world") um     # ok
]
```

If you do need to pass in an immediate future as a container element
(e.g., to simplify the API when calling with different conditions),
use `um_(immediate: ...)` to make it clear that you want it that way.

# enums and masks

## enumerations

We can create a new type that exhaustively declares multiple subtypes that it could be.
Enumerations consist of mutually exclusive options -- no two values may be held
simultaneously.  See masks [`any_of_`](#masks) for a similar class type that allows
multiple options at once.

The syntax is `type_case_: one_of_` followed by a list of `tag`s, i.e., named fields
with an optional subtype (defaults to `null_`) and optional tag number (defaults to
next available tag number after the previous tag).  The common case, i.e., enumerations,
only include a `variable_case:` declaration.  But you can add subtypes for data the
named field holds,  e.g., `variable_name: type_case_`, with default type names possible
to abbreviate `u32:` as `u32: u32_`.  You can also explicitly assign the tag numbers,
e.g., `variable_name: 5` to use 5 as the tag number for `variable_name` (which won't
include any subtype data unless `variable_name_` is in scope as a type), or
`data_name: data_type_ = 6` to use 6 as the tag number for the field `data_name` that
holds subtype `data_type_`).

Enums use tag number types that are by default the smallest standard integral type that
holds all values, and can be signed types (in contrast to masks which are only unsigned).
If desired, you can specify the underlying tag number type using `i8_ one_of_{...}` instead
of `one_of_{...}`, but this will be a compile error if the type is not big enough to
handle all options.  It will not be a compile warning if the `one_of_` includes types
inside (e.g., `i8_ one_of_{u32:, f32:}`); we'll assume you want the tag number type to be
an `i8_`.  However, it should be obvious that the full type will be at least the size of the
tag number plus the largest element in the `one_of_`; possibly more to achieve alignment.

Here is an example enum with some values that aren't specified.  Even though
the values aren't specified, they are deterministically chosen.

```
my_enum_: one_of_
{    first_value_defaults_to_zero:
     second_value_increments:
     third_value_is_specified: 123
     fourth_value_increments:
}
assert_(my_enum_ first_value_defaults_to_zero) == 0
assert_(my_enum_ second_value_increments) == 1
assert_(my_enum_ third_value_is_specified) == 123
assert_(my_enum_ fourth_value_increments) == 124

# behind the scenes, tags have a bit of reflection going on:
my_enum_ first_value_defaults_to_zero_
     ==   tag_{one_of_: my_enum_, 0, null_, field: "first_value_defaults_to_zero"}
my_enum_ second_value_increments_
     ==   tag_{one_of_: my_enum_, 1, null_, field: "second_value_increments"}
my_enum_ third_value_is_specified_
     ==   tag_{one_of_: my_enum_, 123, null_, field: "third_value_is_specified"}
my_enum_ fourth_value_increments_
     ==   tag_{one_of_: my_enum_, 124, null_, field: "fourth_value_increments"}
```

You can even pass in existing variable(s) to the enum, although they should be
compile-time constants.  This uses the same logic as function arguments to
determine what the name of the enum value is.

```
super: 12
crazy: 15
# the following will define
# `other_enum_ other_value1 = 0`,
# `other_enum_ super = 12`,
# and `other_enum_ other_value2 = 15`.
other_enum_: one_of_
{    other_value1:
     super:
     other_value2: crazy
}
```

Here is an example enum with just specified values, all inline:

```
# fits in a `u1_`.
# TODO: add syntax for globalizing values in an enum so you don't need to do
# `bool_ false` when you ask for that value.  e.g., `@global false: 0, @global true: 1`
# or maybe `whatever_: one_of_{...}, @enscope_all(whatever_)`
bool_: one_of_{false: 0, true: 1}
```

Enums provide a few extra additional methods for free as well, including
the number of values that are enumerated via the class function `count_(): count_arch_`,
and the min and max values `min_(): enum_type_`, `max_(): enum_type_`.  You can also
check if an enum instance `enum` is a specific value `this_value` via
`enum is_this_value_()` which will return true iff so.

```
test: bool_ = false # or `test: bool_ false`

if test == _ true   # OK
     print_("test is true :(")
if test is_false_() # also OK
     print_("test is false!")

# get the count (number of enumerated values) of the enum:
print_("bool has $(bool_ count_()) possibilities:")
# get the lowest and highest values of the enum:
print_("starting at $(bool_ min_()) and going to $(bool_ max_())")
```

Because of this, it is a bit confusing to create an enum that has `count` as an
enumerated value name, but it is not illegal, since we can still distinguish between the
enumerated value (`enum_name_ count`) and total number of enumerated values
(`enum_name_ count_()`).  Similarly for `min`/`max`.
TODO: Actually this does get confusing since we can have data types in a `one_of_`,
and `the_enum_ count_(123)` with `the_enum_: one_of_{count:, ...}`.
we might need to do something like `one_of_ count_(the_enum_)`.
or `count_{one_of_: the_enum_}()`.
this does make it a bit confusing for generics: should `count_(the_enum_)`
be the same as `the_enum_ count_()`, and be the same as `count_{the_enum_}()`?
or maybe we do something like `the_enum_ tags_() count_()` (and similarly for `min_`/`max_`).
maybe reserve `tags` and `tag` for enums and masks.

Also note that the `count_()` class function will return the total number of
enumerations, not the number +1 after the last enum value.  This can be confusing
in case you use non-standard enumerations (i.e., with values less than 0):

```
# this will fit in an `i2_` due to having negative values.
sign_: one_of_
{    negative: -1
     zero: 0
     positive: 1
}

print_("sign has $(sign_ count_()) values")   # 3
print_("starting at $(sign_ min_()) and going to $(sign_ max_())")     # -1 and 1

# this will fit in a `u4_` due to values going up to 9.
weird_: one_of_
{    x: 1
     y: 2
     z: 3
     q: 9
}

print_(weird_ count_())     # prints 4
print_(weird_ min_())       # prints 1
print_(weird_ max_())       # prints 9
```

### default values for a `one_of_`

Note that the default value for a `one_of_` is the first value, unless zero is an option
(and it's not the first value).  Note that `null` does not belong in a `one_of_`, but
will automatically be space-optimized for if you leave enough tags in your enum and
you request a nullish type, e.g., `one_of_{a:, ...}?`.  The reason we don't allow
`null`s in a `one_of_` is to align with [`any_of_` logic](#masks).  If a `one_of_`
instance is nullable, e.g., `x?: one_of_{a:, b:}`, then `null` is the default value.

### testing enums with lots of values

Note that if you are checking many values, a `what` statement may be more useful
than testing each value against the various possibilities.  Also note that you don't need
to explicitly set each enum value; they start at 0 and increment by default.
But you do always need to include the declaration operator `:` because we
are declaring new values in the enum.

```
option_: one_of_
{    unselected:
     not_a_good_option:
     content_with_life:
     better_options_out_there:
     best_option_still_coming:
     oops_you_missed_it:
     now_you_will_be_sad_forever:
}

print_("number of options should be 7:  $(option_ count_())")

option1: option_ content_with_life

# avoid doing this if you are checking many possibilities:
if option1 is_not_a_good_option_()  # OK
     print_("oh no")
elif option1 == _ oops_you_missed_it  # also OK
     print_("whoops")
...

# instead, consider doing this:
what option1
     _ not_a_good_option
          print_("oh no")
     _ best_option_still_coming
          print_("was the best actually...")
     _ unselected
          fall_through
     else
          print_("that was boring")
```

Note that we don't have to do `option_ not_a_good_option` (and similarly for other options)
along with the cases.  The compiler knows that since `option1` is of type `option_`,
so we can use `_` to namespace correctly as `option_`.

### `one_of_` with data

The `one_of_` type is a tagged union, which can easily mix simple enum values
(with no accompanying data) with tagged data types.

```
id_: one_of_
{    unknown:                      # defaults to tag 0, no subtype data
     another_pure_enum_value: 5    # enum with explicit tag
     int:                          # data type, implicitly tagged to 6
     dbl: 9                        # data type with explicit tag of 9
     named_type: u64_ = 15         # renamed data type with explicit tag of 15
     str: 19                       # data type (`str_`) with explicit tag of 19
     fragrance: u32_               # data type with implicit tag of 20
}

id_ unknown == tag_(0)
id_ unknown_ == tag_{one_of_: id_, 0, null_, field: "unknown"}
id_ another_pure_enum_value == tag_(5)
id_ another_pure_enum_value_ == tag_{one_of_: id_, 5, null_, field: "another_pure_enum_value"}
id_ int == tag_(6)
id_ int_ == tag_{one_of_: id_, 6, int_, field: "int"}
id_ dbl == tag_(9)
id_ dbl_ == tag_{one_of_: id_, 9, dbl_, field: "dbl"}
id_ named_type == tag_(15)
id_ named_type_ == tag_{one_of_: id_, 15, u64_, field: "named_type"}
id_ str == tag_(19)
id_ str_ == tag_{one_of_: id_, 19, str_, field: "str"}
id_ fragrance == tag_(20)
id_ fragrance_ == tag_{one_of_: id_, 20, u32_, field: "fragrance"}

# you can use the tag type to create an instance of `id_`:
my_id; id_ fragrance_(1234)   # `my_id` is of type `id_`
# ... some other operations
# this will be `null` if `my_id` is not an instance of `id_ fragrance`:
fragrance?: u32_ = my_id fragrance_()
# this will be `null` if `my_id` is not an instance of `id_ str`:
str?: my_id str_()
```

As seen in the example above, you can try to grab a copy of whatever data
is in the `one_of_` by using `variable_name field_name_()`; it will return
null if `field_name` is not populated.  Note that these methods are not
created for simple enum fields, since they would always return null even if
that enum tag was active.  Instead, use `is_enum_value_()` to check if the
field `enum_value` is present.

### `one_of_` with nested data

Consider this example `one_of_`.

```
tree_: one_of_
{    leaf: [value; int_]
     branch:
     [    left; heap_[tree_]
          right; heap_[tree_]
     ]
}
```

When checking a `tree_` type for its internal structure, you can use `is_leaf_()`
or `is_branch_()` if you just need a boolean, but if you need to manipulate
one of the internal types, you should use `::is_(fn_(internal_type:): null_): bool_`
or `;;is_(fn_(internal_type;): null_): bool_` if you need to modify it,
where `internal_type_` is either `leaf_` or `branch_` in this case.
For example:

```
tree; tree_ = if some_condition
     _ leaf_(value: 3)
else
     branch_(left: _ leaf_(value: 1), right: _ leaf_(value: 5))

if tree is_leaf_()
     # no type narrowing, not ideal.
     print_(tree)

# narrowing to a `leaf_` type that is readonly, while retaining a reference
# to the original `tree` variable.  the nested function only executes if
# `tree` is internally of type `leaf_`:
# TODO: how can we infer `leaf:` as `tree_ leaf_` type here?  might need to be `_ leaf:`
tree is_(fn_(leaf:): print_(leaf))  # for short, `tree is_({print_($leaf)})`

# narrowing to a `branch_` type that is writable.  `tree` was writable, so `branch` can be.
# the nested function only executes if `tree` is internally of type `branch_`:
tree is
(    fn_(branch;):
          print_(branch left, " ", branch right)
          # this operation can affect/modify the `tree` variable.
          branch left some_operation_()
)
```

Even better, use the [`is` operator](#is-operator) and define a block:

```
# you can also use this in a conditional; note we don't wrap in a lambda function
# because we're using fancier block syntax.
if tree is branch;
     branch left some_operation_()
     print_("a branch")
else
     print_("not a branch") 
```

If you need to manipulate most of the internal types, use `what` to narrow the type
and handle all the different cases.

```
what tree
     leaf: 
          print_(leaf)
     branch;
          # this operation can affect/modify the `tree` variable.
          print_(branch left, " ", branch right)
          branch left some_operation_()
```

If you want to make a copy, use: `new_leaf?; tree leaf_()` or
`my_branch?; tree branch_()`; these variables will be null if the `tree`
is not of that type, but they will also be a copy and any changes to the
new variables will not be reflected in `tree`.  If you expect to see
OOM errors and want to avoid panicking, use the result overload via e.g.
`new_leaf?: tree leaf_() assert_()`.

```
one_of_{..., ~t_}: []
{    # returns true if this `one_of_` is of type `t`, also allowing access
     # to the underlying value by passing it into the function.
     ;:is_(fn_(t;:): null_): bool_

     # type narrowing.
     # the signature for `if tree is branch; {#[do stuff with `branch`]#}`
     # the method returns true iff the block should be executed.
     # the block itself can return a value to the parent scope.
     ;:.is_(), block{declaring_:;. t_, exit_: ~u_}: bool_
}
```

### flattening and containing

Note that `one_of_{one_of{a:, b:}:, one_of{c:, d:}:}` is not the same as
`one_of_{a:, b:, c:, d:}`.  To get that result, use `flatten_`, e.g.,
`flatten_{one_of_{a:, b:}, one_of_{c:, d:}}` will equal `one_of_{a:, b:, c:, d:}`.
This is safe to use on other types, so `flatten_{one_of_{c:, d:}, e:}`
is `one_of_{c:, d:, e:}`.

### `one_of`s as function arguments

The default name for a `one_of_` argument is `one_of`.  E.g.,

```
# this is short for `my_function_(one_of: one_of_{int:, str:}): dbl_`:
my_function_(one_of{int:, str:}:): dbl_
     dbl_(one_of) ?? panic_()

print_(my_function_(123))      # prints 123.0
print_(my_function_("123.4"))  # prints 123.4
```

Internally this creates multiple function overloads:  one for when the argument's
type is unknown at compile time, and one for each of the possible argument types
when it is known (e.g., `int` and `str` in this case).  The latter is for
flexibility, the former is for speed.

If you need to use multiple `one_of_`s in a function and still want them to be
default-named, it's recommended to give specific names to each `one_of_`, e.g.,

```
int_or_string_: one_of_{int:, str:}
weird_number_: one_of_{u8:, i32:, dbl:}

my_function_(int_or_string:, weird_number:): dbl_
     dbl_(int_or_string) ?? panic_() * weird_number
```

However, you can also achieve the same thing using namespaces,
if you don't want to add specific names for the `one_of_`s.

```
# arguments short for `A_one_of: one_of_{int:, str:}` and `B_one_of: one_of_{u8:, i32:, dbl:}`.
my_function(A_one_of{int:, str:}, B_one_of{u8:, i32:, dbl:}): dbl_
     dbl_(A_one_of) ?? panic_() * B_one_of
```

Again, this fans out into multiple function overloads for each of the cases
of compile-time known and compile-time unknown arguments.  Similar to
nullable arguments, this may create compile-time errors if the developer
specifies an overload that the compiler is trying to generate.  In case
you have a specialization you'd like to override specifically, you need
to add an annotation like `@no_specialization(A_one_of: int_)` to the
function with a `one_of` argument, in this example for a `one_of_` named
`A_one_of` that has an `int_` option.

TODO: ensure that we can use `call` or similar to create our own version
of a `one_of` with metaprogramming or whatever.

### `one_of_` with specific instances

You can restrict inputs to an allowed subset of e.g., integers using
`one_of_` with the specific inputs.

```
# by default this will use the smallest integer (or even unsigned integer) type
# that will fit all values; in this case, a `u4_`.
# TODO: should this be a `u3_` because we have more than 4 but fewer than 8 values?
small_odd_: one_of_{1, 3, 5, 7, 9}

x: small_odd_ = 3  # OK

# tags work a bit differently because we don't have named fields, but we
# can use `@symbol(...)` to get the field name:
small_odd_ @symbol(1) == tag_(0)
small_odd_ @symbol(3) == tag_(1)
small_odd_ @symbol(5) == tag_(2)
# etc.
# and you can get the type like this:
small_odd_ @symbol(1_) == tag_{one_of_: small_odd_, 0, 1, field: "1"}
small_odd_ @symbol(3_) == tag_{one_of_: small_odd_, 1, 3, field: "3"}
small_odd_ @symbol(5_) == tag_{one_of_: small_odd_, 2, 5, field: "5"}
# etc.
```

Here is an example with specific string instances.

```
valid_color_: one_of_{"gray", "black", "white"}

get_(valid_color:): str_
     what valid_color
          "gray"
               "#757575"
          "black"
               "#000000"
          "white"
               "#ffffff"

assert_(get_("black")) == "#000000"

valid_color_ @symbol("gray") == tag(0)
valid_color_ @symbol("gray"_) == tag_{one_of_: valid_color_, 0, "gray", field: "gray"}
valid_color_ @symbol("black") == tag(1)
valid_color_ @symbol("black"_) == tag_{one_of_: valid_color_, 1, "black", field: "black"}
valid_color_ @symbol("white") == tag(2)
valid_color_ @symbol("white"_) == tag_{one_of_: valid_color_, 2, "white", field: "white"}
```

## select

If you *don't* want to allow the combined case as an argument, e.g., `one_of_{a:, b:}`,
you can use `select_{a:, b:}` for a function argument.  This will only generate overloads
with the distinct types and not the combined type, like so:

```
my_fn_(select_{int:, str:}:): str_
     "hello $(select)"

# becomes only these two functions internally:
my_fn_(int:): str_
     "hello $(int)"

my_fn_(str:): str_
     "hello $(str)"

# when calling:
my_fn_(5)           # OK
my_fn_("world")     # OK
MY_one_of; one_of_{int:, str:}(3)
# ... some logic that might change MY_one_of
my_fn_(MY_one_of)   # COMPILE ERROR
```

This is generally not recommended for function arguments, but it can
be used to restrict class generics to a list of only certain types, e.g.,
`primitives_: select_{i8:, i16:}, my_generic_{of_: primitives_}: [of;]`
will only allow specification as `my_generic_{i8_}` or `my_generic_{i16_}`,
and not `my_generic_{one_of_{i8:, i16:}}`.  Note that `select_` generally
needs to be put into a named type (e.g., `a_or_b_: select_{a:, b:}`) and
specified at compile time as exactly one of the subtypes since it is a meta-type.

## masks

Masks are generalized from enumerations to allow multiple values held simultaneously.
Each value can also be thought of as a flag or option, which are bitwise OR'd together
(i.e., `|`) for the variable instance.  Under the hood, these use unsigned integer types.
Trying to assign a negative tag will throw a compiler error.  Unlike enums which only hold
`one_of_` the fields at a time, masks hold "any of", so we use `any_of_{a:, b:, c:}`
to declare a mask which can be `a`, `b`, `c`, or some (non-empty) combination of all three.
If there are no values present, we use `null` to represent the value, so the variable must
be defined as nullable for that situation.

The reason why we overload `null` here is that if we want to define an options bag
for a function, e.g., `search_options_: any_of_{name: str_, id: u64_}` for
`search_(search_options:): t_`, we want `search_()` to be a separate overload
which means having passed in no options, which isn't declared implicitly.
Both overloads can be declared simultaneously (and explicitly) using a nullable
`search_options_` for the argument, i.e., `search_(search_options?:): t_`, but
if you require at least one field to be defined, you have the option of only
defining the non-null overload `search_(search_options:): t_`.

oh-lang masks share similar features with enums.

* You can specify the integer type that backs the mask tag, but in this case
it must be an unsigned type, e.g., `u32_ any_of_{...}`.  Note that by default, the tag
is exactly as many bits as it needs to be to fit the desired options, rounded up to
the nearest standard unsigned type (`u8_`, `u16_`, `u32_`, `u64_`, `u128_`, etc.).
We will add a warning if the user is getting into the `u128_+` territory.

* Masks don't need to specify their tags; but if you do specify them,
they must be powers of two.

* Masks can have an `is_this_value_()` method for a `this_value` option,
which is true if the mask is exactly equal to `this_value` **and nothing else**.
You can use `has_this_value_()` to see if it contains `this_value`,
but may contain other values besides `this_value`.

* Masks can contain data aside from tags.  E.g., `the_mask_: any_of_{some_type:, other_type:}`
where `some_type_` and `other_type_` are defined elsewhere.  In this case,
you can use `is` and `has` operators to narrow the type like this:
`if the_mask is some_type;` or `if the_mask has other_type;`.

* Unlike an enum, if a mask variable is modifiable, it likely needs to be
declared as nullable, because removing a value from the mask can make it empty.

### `any_of_` with default tags

Here is an example with default-assigned tags, which showcases the nullable issue.

```
food_: any_of_
{    carrots:
     potatoes:
     tomatoes:
}

food_ carrots == tag_(1)
food_ potatoes == tag_(2)
food_ tomatoes == tag_(4)
# note `food_ carrots_` is the `null_` type since there is no tagged data,
# and similarly for `food_ potatoes_` and `food_ tomatoes_`.

# has all the same static methods as enum:
food_ count_() == 7
food_ min_() == 1
food_ max_() == 7   # = _ carrots | _ potatoes | _ tomatoes

food: _ carrots | _ tomatoes
food has_carrots_()      # true
food has_potatoes_()     # false
food has_tomatoes_()     # true
food is_carrots_()       # false, since `food` is not just `carrots`.

# COMPILE ERROR: mutable `food_` instance should be nullable, i.e., `other_food?; ...`
other_food; food_ potatoes
other_food ><= _ potatoes     # toggle potatoes
# NOTE: now `other_food` is null; so
# `other_food` needs to be defined as `other_food?;`.
```

### `any_of_` with explicit tags

```
# if specified, the mask should have tags that are powers of two:
non_mutually_exclusive_type_: any_of_
{    x: 1
     y: 2
     z: 4
     t: 32     # cannot use 8 or 16 because reason Q
}

non_mutually_exclusive_type_ x == tag_(1)
non_mutually_exclusive_type_ y == tag_(2)
non_mutually_exclusive_type_ z == tag_(4)
non_mutually_exclusive_type_ t == tag_(32)
# note `non_mutually_exclusive_type_ x_` is the `null_` type
# since there is no tagged data, and similarly for `_ y_`, `_ z_`, and `_ t_`.

# has all the same static methods as enum:
non_mutually_exclusive_type_ count_() == 15
non_mutually_exclusive_type_ min_() == 1
non_mutually_exclusive_type_ max_() == 39    # = _ x | _ y | _ z | _ t

options?; non_mutually_exclusive_type_
options == tag_(0)  # true; masks start at 0.
options == null     # also true; `null`.
options |= _ x      # inferred mask type (via `_`)
options |= non_mutually_exclusive_type_ z    # explicit mask type

options has_x_()    # true
options has_y_()    # false
options has_z_()    # true
options has_t_()    # false
options tag_() == tag_(5)     # true, `_ x` is `tag_(1)` and `_ z` is `tag_(4)`.

options = _ t
options is_t_()     # true
options has_t_()    # true
```

### `any_of_` with data

```
search_options_: any_of_{name: str_, id: u64_}

# to be consistent with masks without data,
# the field will give you the tag number,
# but if you want the type use the `_` postfix.
search_options_ name == tag_(1)
search_options_ name_ == tag_{any_of_: search_options_, 1, str_, field: "name"}
search_options_ id == tag_(2)
search_options_ id_ == tag_{any_of_: search_options_, 2, u64_, field: "id"}

search_(search_options:): null_
     if search_options is name:
          # only triggers if `search_options` is exactly/only `name`
          print_("searching for name: $(name)...")
     elif search_options is id:
          # only triggers if `search_options` is exactly/only `id`
          print_("searching for id: $(id)...")
     else
          # both `id` and `name` can be defined
          name: search_options name_() assert_()
          id: search_options id_() assert_()
          print_("searching for name $(name) and id $(id)...")

# TODO: if we only have one overload, can we simplify to `search_(_ name_("cool dude"))`?
search_(search_options_ name_("cool dude"))  # prints "searching for (name: cool dude)"
```

### `any_of_` with explicitly tagged data

```
explicitly_tagged_: any_of_
{    name: str_ = 4      # cannot use 1 or 2 for reason I
     str: 16             # cannot use 8 for reason J
     next: u64_          # implicit tag, will use next available.
     wow: 64             # explicit tag, no data type
}

explicitly_tagged_ name == tag_(4)
explicitly_tagged_ name_ == tag_{any_of_: explicitly_tagged_, 4, str_, field: "name"}
explicitly_tagged_ str == tag_(16)
explicitly_tagged_ str_ == tag_{any_of_: explicitly_tagged_, 16, str_, field: "str"}
explicitly_tagged_ next == tag_(32)
# TODO: should we use `field: @"next"` to indicate `@"next"` is a symbol/field in the code?
explicitly_tagged_ next_ == tag_{any_of_: explicitly_tagged_, 32, u64_, field: "next"}
explicitly_tagged_ wow == tag_(64)
explicitly_tagged_ wow_ == tag_{any_of_: explicitly_tagged, 64, null_, field: "wow"}

explicitly_tagged; _ next_(1234)
explicitly_tagged == explicitly_tagged_ next_(1234)    # true
explicitly_tagged = _ name_("whatever")
if explicitly_tagged is name:
     print_("got name: $(name)")   # prints "got name: whatever"

explicitly_tagged |= _ wow
explicitly_tagged |= _ str_("cool")
# TODO: do we need to do `if explicitly_tagged is name: _ name_`?
#    or something like `if explicitly_tagged is _ name:`?
#    i like the latter as long as `explicitly_tagged is _ name:` expands to `explicitly_tagged is name: _ name_`
#    i think for consistency we need this because `name_` is not in scope here and we don't want to assume scope:
if explicitly_tagged is name:
     print_("should not print, no longer exlusively `name`")
if explicitly_tagged has name:
     print_("contains name: $(name)")   # prints "contains name: whatever"
if explicitly_tagged has str:
     print_("contains str: $(str)")     # prints "contains str: cool"
if explicitly_tagged has _ wow
     print_("also contains wow factor")      # should print

explicitly_tagged tag_() == _ name | _ str | _ wow     # true
```

TODO: what do we do with `explicitly_tagged_ name_("asdf") & explicitly_tagged_ name_("jkl;")`?
do we combine to `explicitly_tagged_ name_("asdf jkl;")` using the `&` operator for the name strings?
`_ next_(4 + 8) & _ next_(8 + 16)` could be `_ next_(8)` based on `&` for ints.
`_ next_(4 + 8) | _ next_(8 + 16)` could be `_ next_(4 + 8 + 16)` based on `|` for ints.
what would `|` look like for strings?
`><` doesn't really have any problems.
or we could just make `&` and `|` take the first value if present (and the second value is
present for `&` or regardless of the second value for `|`).

## named value-combinations

You can add some named combinations by extending a mask like this.

```
my_mask_: any_of_{x:, y:}
{    x_and_y: x | y
}

result: my_mask_ x_and_y
print(result & x)   # truthy, should be 1
print(result & y)   # truthy, should be 2
```

# lifetimes and closures

## lifetimes of variables and functions

Variable and function lifetimes are usually scoped to the block that they
were defined in.  Initialization happens when they are encountered, and
descoping/destruction happens in reverse order when the block ends.  With
lambda functions we have to be especially careful.  If a lambda function's
lifetime exceeds that of any of its hidden inputs' lifetimes, we'll get a
segfault or worse.

Let's illustrate the problem with an example:

```
# define a re-definable function.
live_it_up_(string:); index_
     string bytes_() count_()

if some_condition
     some_index; index_ = 9
     # redefine:
     live_it_up_(string:); index_
          string bytes_() count_() + ++some_index

     print_(live_it_up_("hi"))     # this should print 12
     print_(live_it_up_("ok"))     # this should print 13

print_(live_it_up_("no"))          # should this print 14 or 2??
```

Within the `if some_condition` block, a new variable `some_index` gets defined,
which is used to declare a new version of `live_it_up_`.  But once that block
is finished, `some_index` gets cleaned up without care that it was used elsewhere,
and if `live_it_up_` is called with the new definition, it may segfault (or start
changing some other random variable's data).  Therefore, we must not allow the
expectation that `live_it_up_("no")` will return 14 outside of the block.

We actually don't want `live_it_up_("no")` to return 2 here, either; we want this
code to fail at compilation.  We want to detect when users are trying to do
closures (popular in garbage-collected languages) and let them know this is
not allowed; at least, not like this.

You can define variables and use them inside impure functions
but they must be defined *before* the impure function is first declared.  So this
would be allowed:

```
some_index; index_ = 9
live_it_up_(string:); index_
     string bytes_() count_()

if some_condition
     # redefine:
     live_it_up_(string:); index_
          string bytes_() count_() + ++some_index

     print_(live_it_up_("hi"))     # this should print 12
     print_(live_it_up_("ok"))     # this should print 13

print_(live_it_up_("no"))     # can print 2 or 14 depending on `some_condition`.
```

Alternatively, we allow a lambda function to "take" a variable into
its own private scope so that we could redefine `live_it_up_` here with a new
internal variable.  We still wouldn't allow references; they'd have to be
new variables scoped into the function block.

```
if some_condition
     live_it_up_(string:); index_
          # "@dynamic" means declare once and let changes persist across function invocations.
          # Note that this functionality cannot be used to store references.
          @dynamic x; index_ = 12345
          string bytes_() count_() + ++x
```

Similarly, returning a function from within a function might look like
breaking scope.  But this is ok because functions are globally defined anyway:

```
next_generator_(int;): fn_(): int_
     fn_(): ++int
```

Because `;` and `:` arguments in functions are references, they are defined
before this function is called.  E.g., even if `int;` was given as a temporary,
it is defined before this function (and not deleted after the function call).

Here is an example where the return value is a function which uses
the another function from the input.  This is ok because the returned
function has internals that have greater lifetimes than itself; i.e.,
the input function will be destroyed only after the output function
is descoped.

```
# function that takes a function as an argument and returns a function
# example usage:
#   some_fn_(): "hey"
#   # need to specify the overload
#   other_fn_(): int_ = wow_(fn_(): str_ = some_fn_)
#   print_(other_fn_()) # 3
wow_(INPUT_fn_(): string_): fn_(): int_
     fn_(): int_
          INPUT_fn() bytes_() count_()
```

# grammar/syntax

Note on terminology:

* Declaration: declaring a variable or function (and its type) but not defining its value

* Definition: declaration along with an assignment of the function or variable.

* `identifier`: starts with a non-numeric character, can have numerical characters after that.

* `function_case_`/`type_case_`: Identifier which ends with a trailing underscore.
     A leading underscore is allowed to indicate an unused function/type.

* `variable_case`: Identifier which does *not* end with a trailing underscore.
     A leading underscore is allowed to indicate an unused variable.

TODO: use an oh-lang version.
See [the grammar definition](https://github.com/hm-lang/core/blob/main/transpiler/grammar.hm).

# tokenizer

TODO

# compiling

If your code compiles, we will also format it.

If there are any compile errors, the compiler will add some special
comments to the code that will be removed on next compile, e.g.,
`#@! ^ there's a syntax problem`

# implementation

See [TODO.md](https://github.com/oh-lang/oh/blob/main/TODO.md) for ideas.
