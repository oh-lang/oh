const Number = @import("number.zig").Number;
const RunContext = @import("run_context.zig").RunContext;
const SmallString = @import("string.zig").Small;

const RunError = error{
    unknown,
};

pub const Run = struct {
    id_next: Id = 0,

    pub const Error = RunError;
    pub const Value = RunValue;
    pub const Declare = RunDeclare;
    /// Use `truthy` or `falsey` when you are in an interpreter and
    /// only need to see the true/false branch of the conditional.
    /// Use `unevaluated` if you want to see what both blocks will
    /// look like, e.g., for transpiling.
    pub const Condition = enum {
        truthy,
        falsey,
        unevaluated,
    };

    pub fn nextStatement(self: *Self) RunContext {
        return RunContext{ .statement = self.nextId() };
    }

    fn nextId(self: *Self) Id {
        const id = self.id_next;
        self.id_next += 1;
        return id;
    }

    pub const Id = u64;
    const Self = @This();
};

const RunDeclare = enum {
    generic, // `
    readonly, // :
    writable, // ;
    temporary, // .
    // TODO: either add these combinations OR declare functions multiple times
    //      with each single-declare value.  theoretically we want to be able
    //      to write oh-lang with the oh-lang transpiler, though, so that
    //      would require defining all these.  but parser is at a different stage...
    //    rr, // ::
    //    rw, // :;
    //    rt, // :.
    //    wr, // ;:
    //    ww, // ;;
    //    wt, // ;.
    //    tr, // .:
    //    tw, // .;
    //    tt, // ..
};

const RunValueTag = enum {
    identifier,
    number,
    // A value that is stored in the interpreter/transpiler,
    // i.e., is not a compile-time constant.
    intermediate,
};

const RunValue = union(RunValueTag) {
    identifier: SmallString,
    number: Number,
    intermediate: u64,

    pub const Tag = RunValueTag;
    const Self = @This();
};
