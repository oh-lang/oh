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
specify `I * P`, you need to write `I` a special way.

https://github.com/BuildIt-lang/buildit
