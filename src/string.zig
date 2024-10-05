const common = @import("common.zig");

const std = @import("std");

const StringError = error{
    string_too_long,
    out_of_memory,
};

// TODO: something like this would be cool
// const Pool = struct {
//     index_map: std.StringHashMap(usize),
//     array: std.ArrayList(u8),
// };

pub const Small = extern struct {
    pub const Error = StringError;
    pub const max_count: usize = std.math.maxInt(u16);

    /// Each `Small` will only ever get to 65535 as the max size, so
    /// 65534 will be the max (non-empty) `start` field and
    /// 65535 will be the max `end` field, so a `u16` type is ok here.
    pub const Range = common.Range(u16);
    pub inline fn range(start: anytype, end: anytype) Range {
        return .{ .start = @intCast(start), .end = @intCast(end) };
    }

    // TODO: rename to `getNoAllocSize()`
    fn getMediumSize() comptime_int {
        const small: Small = .{};
        const smallest_size = @sizeOf(@TypeOf(small.short));
        return smallest_size + @sizeOf(@TypeOf(small.remaining.if_small));
    }

    size: u16 = 0,
    short: [6]u8 = undefined,
    remaining: extern union {
        if_small: [8]u8,
        pointer: *u8,
    } = undefined,

    pub fn deinit(self: *Small) void {
        if (self.size > getMediumSize()) {
            common.allocator.free(self.buffer());
        }
        self.size = 0;
    }

    pub fn renew(self: *Small, new_chars: []const u8) StringError!void {
        const new_string = try Small.init(new_chars);
        self.deinit();
        self.* = new_string;
    }

    pub fn moot(self: *Small) Self {
        const new_string = *self;
        self.* = Self{};
        return new_string;
    }

    pub inline fn init(chars: []const u8) StringError!Small {
        var string = try Small.allocExactly(chars.len);
        @memcpy(string.buffer(), chars);
        string.sign();
        return string;
    }

    pub inline fn as64(chars: anytype) u64 {
        // We're expecting `chars` to be `*const [n:0]u8` with n <= 8
        if (chars.len > 8) {
            @compileError("Small.as64 must have 8 characters or less");
        }

        return internalAs64(chars);
    }

    /// Initializes a `Small` that is just on the stack (no allocations on the heap).
    /// For compile-time-known `chars` only.  For anything else, prefer `init` and
    /// just defer `deinit` to be safe.
    pub inline fn noAlloc(chars: anytype) Small {
        // We're expecting `chars` to be `*const [n:0]u8` with n <= getMediumSize()
        if (chars.len > comptime getMediumSize()) {
            @compileError(std.fmt.comptimePrint("Small.noAlloc must have {d} characters or less", .{getMediumSize()}));
        }
        return Small.init(chars) catch unreachable;
    }

    // Another initialization that doesn't require an allocation.
    // We use big endian such that "asdf" "jkl;" would combine as "asdfjkl;",
    // i.e., `;` gets the small digit and `a` gets the big digit.
    pub fn init64(b64: u64) Small {
        var result: Small = .{ .size = 8 };
        var buffer_count: u16 = 0;
        var remaining64 = b64;
        // Get the size of the buffer needed, first.
        while (remaining64 > 0) {
            remaining64 >>= 8;
            buffer_count += 1;
        }
        // Reset and do the actual writing now.
        remaining64 = b64;
        result.size = buffer_count;
        const write_buffer = result.buffer();
        while (remaining64 > 0) {
            buffer_count -= 1;
            write_buffer[buffer_count] = @intCast(remaining64 & 255);
            remaining64 >>= 8;
        }
        return result;
    }

    // TODO: we should see how flexible the allocation stuff is;
    // do we need the end of the range for the deallocation to work correctly?
    // if so, we could allow pop() from this string, etc.
    // Probably not something we want to support, though.
    /// Note this will not provide a signature.
    pub fn allocExactly(new_size: anytype) StringError!Small {
        if (new_size > max_count) {
            return StringError.string_too_long;
        }
        var string: Small = .{ .size = @intCast(new_size) };
        if (new_size > comptime getMediumSize()) {
            const heap = common.allocator.alloc(u8, new_size) catch {
                std.debug.print("couldn't allocate {d}-character string...\n", .{new_size});
                return StringError.out_of_memory;
            };
            string.remaining.pointer = @ptrCast(heap.ptr);
        }
        return string;
    }

    /// Signing is only useful for "large" strings.
    pub fn sign(self: *Small) void {
        if (self.size > comptime getMediumSize()) {
            const chars = self.slice();
            common.sign(&self.short, chars) catch {
                std.debug.print("shouldn't have had a problem signing {d} characters\n", .{chars.len});
            };
        }
    }

    pub fn signature(self: *const Small) []const u8 {
        if (self.size <= comptime getMediumSize()) {
            return self.slice();
        }
        return &self.short;
    }

    pub fn big64(self: *const Small) StringError!u64 {
        const chars = self.slice();
        if (chars.len > 8) {
            return StringError.string_too_long;
        }
        return internalAs64(chars);
    }

    pub inline fn at(self: *const Small, at_index: anytype) ?u8 {
        var index: i64 = @intCast(at_index);
        if (index < 0) {
            index += self.size;
            if (index < 0) {
                return null;
            }
        } else if (index >= self.size) {
            return null;
        }
        return self.slice()[@intCast(index)];
    }

    pub inline fn inBounds(self: *const Small, index: usize) u8 {
        std.debug.assert(index < self.count());
        return self.slice()[index];
    }

    pub inline fn count(self: *const Small) u16 {
        return self.size;
    }

    pub const PascalCase = enum {
        keep_starting_case, // lower_case -> lowerCase, Upper_case -> UpperCase
        start_lower, // Upper_case -> upperCase, lower_case -> lowerCase
        start_upper, // Upper_case -> UpperCase, lower_case -> LowerCase

        fn transform(self: PascalCase, char: u8, at_start: bool) u8 {
            if (!at_start) return char;

            return switch (self) {
                .keep_starting_case => char,
                .start_lower => uncapitalize(char),
                .start_upper => capitalize(char),
            };
        }
    };

    pub fn toPascalCase(self: *const Small, case: PascalCase) StringError!Self {
        if (self.count() == 0) {
            return Self{};
        }
        var work_buffer: [65535]u8 = undefined;
        var index: usize = 0;
        var capitalize_next = false;
        var at_start = true;
        for (self.slice()) |char| if (char == '_') {
            capitalize_next = true;
        } else {
            const modified_char = if (capitalize_next) capitalize(char) else char;
            capitalize_next = false;
            work_buffer[index] = case.transform(modified_char, at_start);
            at_start = false;
            index += 1;
        };
        return Self.init(work_buffer[0..index]);
    }

    pub const SnakeCase = enum {
        start_lower, // LowerSnakeCase -> lower_snake_case
        start_upper, // initialUpperSnakeCase -> Initial_upper_snake_case
        /// Like `keep_starting_case`, this case doesn't force the initial char to be
        /// "capitalized" or "uncapitalized".
        no_uppers, // Upper_case -> _upper_case
        keep_starting_case, // keepLower -> keep_lower or KeepUpper -> Keep_upper, _prefix_ok -> _prefix_ok

        // Since SnakeCase can add chars (e.g., myPascal -> my_pascal), return
        // up to two chars in a u16, little-endian style.  (First char is (result & 255)
        // and second char is (result >> 8).)
        fn transform(self: SnakeCase, char: u8, at_start: bool) u16 {
            return switch (self) {
                .start_lower => transformStartLower(char, at_start),
                .start_upper => transformStartUpper(char, at_start),
                .no_uppers => transformNoUppers(char, at_start),
                .keep_starting_case => transformKeepStartingCase(char, at_start),
            };
        }

        inline fn transformStartLower(char: u8, at_start: bool) u16 {
            if (!isCapital(char)) {
                return char;
            } else if (at_start) {
                return uncapitalize(char);
            } else {
                return underscoreChar(char);
            }
        }

        inline fn transformStartUpper(char: u8, at_start: bool) u16 {
            if (at_start) {
                return capitalize(char);
            } else if (!isCapital(char)) {
                return char;
            } else {
                return underscoreChar(char);
            }
        }

        inline fn transformNoUppers(char: u8, at_start: bool) u16 {
            _ = at_start;
            if (!isCapital(char)) {
                return char;
            } else {
                return underscoreChar(char);
            }
        }

        inline fn transformKeepStartingCase(char: u8, at_start: bool) u16 {
            if (at_start or !isCapital(char)) {
                return char;
            } else {
                return underscoreChar(char);
            }
        }

        inline fn underscoreChar(char: u8) u16 {
            const char16: u16 = uncapitalize(char);
            return '_' | (char16 << 8);
        }
    };

    pub fn toSnakeCase(self: *const Small, case: SnakeCase) StringError!Self {
        var work_buffer: [65535]u8 = undefined;
        var index: usize = 0;
        // If the string looks like we're supposed to capitalize.
        var capitalize_next = false;
        var at_start = true; // until we see a non-underscore character.
        for (self.slice()) |char| {
            if (char == '_') {
                capitalize_next = true;
                continue;
            }
            const modified_char = if (capitalize_next) capitalize(char) else char;
            capitalize_next = false;
            const sequence16 = case.transform(modified_char, at_start);
            at_start = false;
            if (index >= work_buffer.len) {
                return StringError.string_too_long;
            }
            work_buffer[index] = @intCast(sequence16 & 255);
            const next_char: u8 = @intCast(sequence16 >> 8);
            index += 1;

            if (next_char == 0) continue;

            if (index >= work_buffer.len) {
                return StringError.string_too_long;
            }
            work_buffer[index] = uncapitalize(next_char);
            index += 1;
        }
        return Self.init(work_buffer[0..index]);
    }

    /// Only use at start of string creation.  It won't be automatically signed;
    /// so you can't rely on that.
    pub fn buffer(self: *Small) []u8 {
        const medium_size = comptime getMediumSize();
        if (self.size <= medium_size) {
            const full_small_buffer: *[medium_size]u8 = @ptrCast(&self.short[0]);
            return full_small_buffer[0..self.size];
        } else {
            const full_buffer: *[Small.max_count]u8 = @ptrCast(self.remaining.pointer);
            return full_buffer[0..self.size];
        }
    }

    pub inline fn in(self: *const Small, in_range: Range) []const u8 {
        return self.slice()[@intCast(in_range.start)..@intCast(in_range.end)];
    }

    pub inline fn fullRange(self: *const Small) Range {
        return .{ .start = 0, .end = self.size };
    }

    pub fn slice(self: *const Small) []const u8 {
        const medium_size = comptime getMediumSize();

        if (self.size <= medium_size) {
            const full_small_buffer: *const [medium_size]u8 = @ptrCast(&self.short[0]);
            return full_small_buffer[0..self.size];
        } else {
            const full_buffer: *const [Small.max_count]u8 = @ptrCast(self.remaining.pointer);
            return full_buffer[0..self.size];
        }
    }

    pub fn contains(self: Small, message: []const u8, where: common.At) bool {
        const self_count = self.count();
        if (self_count < message.len) {
            return false;
        }
        return switch (where) {
            common.At.start => std.mem.eql(u8, self.in(range(0, message.len)), message),
            common.At.end => std.mem.eql(u8, self.in(range(self_count - message.len, self_count)), message),
        };
    }

    pub inline fn printLine(self: *const Small, writer: anytype) !void {
        try writer.print("{s}\n", .{self.slice()});
    }

    pub inline fn print(self: *const Small, writer: anytype) !void {
        try writer.print("{s}", .{self.slice()});
    }

    pub fn equals(self: Small, other: Small) bool {
        if (self.size != other.size) {
            return false;
        }
        return std.mem.eql(u8, self.slice(), other.slice());
    }

    pub fn expectEquals(a: Small, b: Small) !void {
        const equal = a.equals(b);
        if (!equal) {
            std.debug.print("expected {s}, got {s}\n", .{ b.slice(), a.slice() });
        }
        try std.testing.expect(equal);
    }

    pub fn expectNotEquals(a: Small, b: Small) !void {
        const equal = a.equals(b);
        if (equal) {
            std.debug.print("expected {s} to NOT equal {s}\n", .{ a.slice(), b.slice() });
        }
        try std.testing.expect(!equal);
    }

    // DEPRECATED: use expectEqualsSlice
    pub fn expectEqualsString(self: Small, string: []const u8) !void {
        try std.testing.expectEqualStrings(string, self.slice());
    }

    pub fn expectEqualsSlice(self: Small, other_slice: []const u8) !void {
        try std.testing.expectEqualStrings(other_slice, self.slice());
    }

    pub inline fn isUncapital(char: u8) bool {
        return char >= 'a' and char <= 'z';
    }

    pub inline fn capitalize(char: u8) u8 {
        return if (isUncapital(char))
            char - 32
        else
            char;
    }

    pub inline fn isCapital(char: u8) bool {
        return char >= 'A' and char <= 'Z';
    }

    pub inline fn uncapitalize(char: u8) u8 {
        return if (isCapital(char))
            char + 32
        else
            char;
    }

    // TODO: use this everywhere
    const Self = @This();
};

fn internalAs64(chars: []const u8) u64 {
    std.debug.assert(chars.len <= 8);
    var result: u64 = 0;
    for (chars) |char| {
        result <<= 8;
        result |= char;
    }
    return result;
}

test "Small size is correct" {
    try std.testing.expectEqual(16, @sizeOf(Small));
    const small_string: Small = .{};
    try std.testing.expectEqual(2, @sizeOf(@TypeOf(small_string.size)));
    try std.testing.expectEqual(6, @sizeOf(@TypeOf(small_string.short)));
    try std.testing.expectEqual(8, @sizeOf(@TypeOf(small_string.remaining.if_small)));
    try std.testing.expectEqual(14, Small.getMediumSize());
    // TODO: check for `small_string.remaining.pointer` being at +8 from start of small_string
    // try std.testing.expectEqual(8, @typeInfo(@TypeOf(small_string.remaining.pointer)).Pointer.alignment);
}

test "noAlloc works" {
    try std.testing.expectEqualStrings("this is ok man", Small.noAlloc("this is ok man").slice());
}

test "init64 works" {
    var big = Small.init64('*');
    try std.testing.expectEqualStrings("*", big.slice());

    big = Small.init64('_');
    try std.testing.expectEqualStrings("_", big.slice());

    big = Small.init64(20853);
    try std.testing.expectEqualStrings("Qu", big.slice());

    big = Small.init64(6926888221546995238);
    try std.testing.expectEqualStrings("`!@#$%^&", big.slice());
}

test "big64 works for small strings" {
    var small = Small.noAlloc("*");
    try std.testing.expectEqual('*', try small.big64());
    try small.expectEquals(Small.init64('*'));

    small = Small.noAlloc("_");
    try std.testing.expectEqual('_', try small.big64());
    try small.expectEquals(Small.init64('_'));

    small = Small.noAlloc("Qu");
    try std.testing.expectEqual(20853, try small.big64());
    try small.expectEquals(Small.init64(20853));

    small = Small.noAlloc("`!@#$%^&");
    try std.testing.expectEqual(6926888221546995238, try small.big64());
    try small.expectEquals(Small.init64(6926888221546995238));
}

test "big64 throws for >8 character strings" {
    const not_small = Small.noAlloc("123456789");
    try std.testing.expectError(StringError.string_too_long, not_small.big64());
}

test "equals works for noAlloc strings" {
    const empty_string: Small = .{};
    try std.testing.expectEqual(true, empty_string.equals(Small.noAlloc("")));

    const string1 = Small.noAlloc("hi");
    const string2 = Small.noAlloc("hi");
    try std.testing.expectEqual(true, string1.equals(string2));
    try std.testing.expectEqualStrings("hi", string1.slice());
    try string1.expectEquals(string2);

    const string3 = Small.noAlloc("hI");
    try std.testing.expectEqual(false, string1.equals(string3));
    try string1.expectNotEquals(string3);

    var string4 = try Small.init("hi this is going to be more than 16 characters");
    defer string4.deinit();
    try std.testing.expectEqual(false, string1.equals(string4));
    try string1.expectNotEquals(string4);
}

test "equals works for large strings" {
    var string1 = try Small.init("hello world this is over 16 characters");
    defer string1.deinit();
    var string2 = try Small.init("hello world this is over 16 characters");
    defer string2.deinit();
    try std.testing.expectEqual(true, string1.equals(string2));
    try std.testing.expectEqualStrings("hello world this is over 16 characters", string1.slice());
    try string1.expectEquals(string2);

    var string3 = try Small.init("hello world THIS is over 16 characters");
    defer string3.deinit();
    try std.testing.expectEqual(false, string1.equals(string3));
    try string1.expectNotEquals(string3);

    const string4 = Small.noAlloc("hello");
    try std.testing.expectEqual(false, string1.equals(string4));
    try string1.expectNotEquals(string4);
}

test "at/inBounds works for large strings" {
    var string = try Small.init("Thumb thunk rink?");
    defer string.deinit();
    const count: i64 = @intCast(string.count());
    try std.testing.expectEqual('T', string.at(0));
    try std.testing.expectEqual('h', string.inBounds(1));
    try std.testing.expectEqual('u', string.at(2));
    try std.testing.expectEqual('m', string.inBounds(3));
    try std.testing.expectEqual('b', string.at(4));
    try std.testing.expectEqual(' ', string.inBounds(5));

    // Reverse indexing works as well for `at`
    try std.testing.expectEqual('T', string.at(-count));
    try std.testing.expectEqual('h', string.at(-count + 1));
    try std.testing.expectEqual('i', string.at(-4));
    try std.testing.expectEqual('n', string.at(-3));
    try std.testing.expectEqual('k', string.at(-2));
    try std.testing.expectEqual('?', string.at(-1));

    // OOBs works for `at`
    try std.testing.expectEqual(null, string.at(-count - 1));
    try std.testing.expectEqual(null, string.at(count));
}

test "at/inBounds works for small strings" {
    const string = Small.noAlloc("Hi there!!");
    const count: i64 = @intCast(string.count());
    try std.testing.expectEqual('H', string.at(0));
    try std.testing.expectEqual('i', string.inBounds(1));
    try std.testing.expectEqual(' ', string.at(2));
    try std.testing.expectEqual('t', string.inBounds(3));
    try std.testing.expectEqual('h', string.at(4));
    try std.testing.expectEqual('e', string.inBounds(5));

    // Reverse indexing works as well for `at`
    try std.testing.expectEqual('H', string.at(-count));
    try std.testing.expectEqual('i', string.at(-count + 1));
    try std.testing.expectEqual('r', string.at(-4));
    try std.testing.expectEqual('e', string.at(-3));
    try std.testing.expectEqual('!', string.at(-2));
    try std.testing.expectEqual('!', string.at(-1));

    // OOBs works for `at`
    try std.testing.expectEqual(null, string.at(-count - 1));
    try std.testing.expectEqual(null, string.at(count));
}

test "non-allocked String.toPascalCase start_upper works" {
    try (try Small.noAlloc("_hi_you__a!@").toPascalCase(.start_upper)).expectEqualsSlice("HiYouA!@");
    try (try Small.noAlloc("hey_jamboree_").toPascalCase(.start_upper)).expectEqualsSlice("HeyJamboree");
    try (try Small.noAlloc("___ohNo").toPascalCase(.start_upper)).expectEqualsSlice("OhNo");
    try (try Small.noAlloc("!$eleven#:").toPascalCase(.start_upper)).expectEqualsSlice("!$eleven#:");
    try (try Small.noAlloc("a_b_c___d___e").toPascalCase(.start_upper)).expectEqualsSlice("ABCDE");
    try (try Small.noAlloc("_____").toPascalCase(.start_upper)).expectEqualsSlice("");
    try (try Small.noAlloc("AlreadyCased").toPascalCase(.start_upper)).expectEqualsSlice("AlreadyCased");
    try (try Small.noAlloc("almostCased").toPascalCase(.start_upper)).expectEqualsSlice("AlmostCased");
}

test "non-allocked String.toPascalCase start_lower works" {
    try (try Small.noAlloc("_hi_you__a!@").toPascalCase(.start_lower)).expectEqualsSlice("hiYouA!@");
    try (try Small.noAlloc("hey_jamboree_").toPascalCase(.start_lower)).expectEqualsSlice("heyJamboree");
    try (try Small.noAlloc("___ohNo").toPascalCase(.start_lower)).expectEqualsSlice("ohNo");
    try (try Small.noAlloc("!$eleven#:").toPascalCase(.start_lower)).expectEqualsSlice("!$eleven#:");
    try (try Small.noAlloc("a_b_c___d___e").toPascalCase(.start_lower)).expectEqualsSlice("aBCDE");
    try (try Small.noAlloc("_____").toPascalCase(.start_upper)).expectEqualsSlice("");
    try (try Small.noAlloc("alreadyCased").toPascalCase(.start_lower)).expectEqualsSlice("alreadyCased");
    try (try Small.noAlloc("AlmostCased").toPascalCase(.start_lower)).expectEqualsSlice("almostCased");
}

test "non-allocked String.toPascalCase keep_starting_case works" {
    try (try Small.noAlloc("_hi_you__a!@").toPascalCase(.keep_starting_case)).expectEqualsSlice("HiYouA!@");
    try (try Small.noAlloc("hey_jamboree_").toPascalCase(.keep_starting_case)).expectEqualsSlice("heyJamboree");
    try (try Small.noAlloc("___ohNo").toPascalCase(.keep_starting_case)).expectEqualsSlice("OhNo");
    try (try Small.noAlloc("!$eleven#:").toPascalCase(.keep_starting_case)).expectEqualsSlice("!$eleven#:");
    try (try Small.noAlloc("a_b_c___d___e").toPascalCase(.keep_starting_case)).expectEqualsSlice("aBCDE");
    try (try Small.noAlloc("_____").toPascalCase(.keep_starting_case)).expectEqualsSlice("");
    try (try Small.noAlloc("alreadyCased").toPascalCase(.keep_starting_case)).expectEqualsSlice("alreadyCased");
    try (try Small.noAlloc("AlsoCased").toPascalCase(.keep_starting_case)).expectEqualsSlice("AlsoCased");
}

test "non-allocked String.toSnakeCase start_lower works" {
    try (try Small.noAlloc("HiYouA!@").toSnakeCase(.start_lower)).expectEqualsSlice("hi_you_a!@");
    try (try Small.noAlloc("HeyJamboree").toSnakeCase(.start_lower)).expectEqualsSlice("hey_jamboree");
    try (try Small.noAlloc("OhNo").toSnakeCase(.start_lower)).expectEqualsSlice("oh_no");
    try (try Small.noAlloc("!$eleven#:").toSnakeCase(.start_lower)).expectEqualsSlice("!$eleven#:");
    try (try Small.noAlloc("aBCDE").toSnakeCase(.start_lower)).expectEqualsSlice("a_b_c_d_e");
    try (try Small.noAlloc("_____").toSnakeCase(.start_lower)).expectEqualsSlice("");
    try (try Small.noAlloc("lower_cased").toSnakeCase(.start_lower)).expectEqualsSlice("lower_cased");
    try (try Small.noAlloc("Upper_cased").toSnakeCase(.start_lower)).expectEqualsSlice("upper_cased");
    try (try Small.noAlloc("_prefix_cased").toSnakeCase(.start_lower)).expectEqualsSlice("prefix_cased");
    try (try Small.noAlloc("___super_pix").toSnakeCase(.start_lower)).expectEqualsSlice("super_pix");
}

test "non-allocked String.toSnakeCase start_upper works" {
    try (try Small.noAlloc("HiYouA!@").toSnakeCase(.start_upper)).expectEqualsSlice("Hi_you_a!@");
    try (try Small.noAlloc("HeyJamboree").toSnakeCase(.start_upper)).expectEqualsSlice("Hey_jamboree");
    try (try Small.noAlloc("OhNo").toSnakeCase(.start_upper)).expectEqualsSlice("Oh_no");
    try (try Small.noAlloc("!$eleven#:").toSnakeCase(.start_upper)).expectEqualsSlice("!$eleven#:");
    try (try Small.noAlloc("aBCDE").toSnakeCase(.start_upper)).expectEqualsSlice("A_b_c_d_e");
    try (try Small.noAlloc("_____").toSnakeCase(.start_upper)).expectEqualsSlice("");
    try (try Small.noAlloc("lower_cased").toSnakeCase(.start_upper)).expectEqualsSlice("Lower_cased");
    try (try Small.noAlloc("Upper_cased").toSnakeCase(.start_upper)).expectEqualsSlice("Upper_cased");
    try (try Small.noAlloc("_prefix_cased").toSnakeCase(.start_upper)).expectEqualsSlice("Prefix_cased");
    try (try Small.noAlloc("___super_pix").toSnakeCase(.start_upper)).expectEqualsSlice("Super_pix");
}

test "non-allocked String.toSnakeCase no_uppers works" {
    try (try Small.noAlloc("HiYouA!@").toSnakeCase(.no_uppers)).expectEqualsSlice("_hi_you_a!@");
    try (try Small.noAlloc("HeyJamboree").toSnakeCase(.no_uppers)).expectEqualsSlice("_hey_jamboree");
    try (try Small.noAlloc("OhNo").toSnakeCase(.no_uppers)).expectEqualsSlice("_oh_no");
    try (try Small.noAlloc("!$eleven#:").toSnakeCase(.no_uppers)).expectEqualsSlice("!$eleven#:");
    try (try Small.noAlloc("aBCDE").toSnakeCase(.no_uppers)).expectEqualsSlice("a_b_c_d_e");
    try (try Small.noAlloc("_____").toSnakeCase(.no_uppers)).expectEqualsSlice("");
    try (try Small.noAlloc("lower_cased").toSnakeCase(.no_uppers)).expectEqualsSlice("lower_cased");
    try (try Small.noAlloc("Upper_cased").toSnakeCase(.no_uppers)).expectEqualsSlice("_upper_cased");
    try (try Small.noAlloc("_prefix_cased").toSnakeCase(.no_uppers)).expectEqualsSlice("_prefix_cased");
    try (try Small.noAlloc("___super_pix").toSnakeCase(.no_uppers)).expectEqualsSlice("_super_pix");
}

test "non-allocked String.toSnakeCase keep_starting_case works" {
    try (try Small.noAlloc("HiYouA!@").toSnakeCase(.keep_starting_case)).expectEqualsSlice("Hi_you_a!@");
    try (try Small.noAlloc("HeyJamboree").toSnakeCase(.keep_starting_case)).expectEqualsSlice("Hey_jamboree");
    try (try Small.noAlloc("OhNo").toSnakeCase(.keep_starting_case)).expectEqualsSlice("Oh_no");
    try (try Small.noAlloc("!$eleven#:").toSnakeCase(.keep_starting_case)).expectEqualsSlice("!$eleven#:");
    try (try Small.noAlloc("aBCDE").toSnakeCase(.keep_starting_case)).expectEqualsSlice("a_b_c_d_e");
    try (try Small.noAlloc("_____").toSnakeCase(.keep_starting_case)).expectEqualsSlice("");
    try (try Small.noAlloc("lower_cased").toSnakeCase(.keep_starting_case)).expectEqualsSlice("lower_cased");
    try (try Small.noAlloc("Upper_cased").toSnakeCase(.keep_starting_case)).expectEqualsSlice("Upper_cased");
    // TODO: i think this is probably ok, but we should consider retaining the prefix underscore.
    try (try Small.noAlloc("_prefix_cased").toSnakeCase(.keep_starting_case)).expectEqualsSlice("Prefix_cased");
    try (try Small.noAlloc("___super_pix").toSnakeCase(.keep_starting_case)).expectEqualsSlice("Super_pix");
}

test "does not sign short strings" {
    var string = Small.noAlloc("below fourteen");
    try std.testing.expectEqualStrings("below fourteen", string.signature());
    try std.testing.expectEqual(14, string.count());
}

test "signs large strings" {
    var string = try Small.init("above sixteen chars");
    defer string.deinit();

    try std.testing.expectEqualStrings("ab19rs", string.signature());
    try std.testing.expectEqual(19, string.count());
}

test "signs very large strings" {
    // This is the largest string we can do:
    var string = try Small.init("g" ** Small.max_count);
    defer string.deinit();

    try std.testing.expectEqualStrings("g65535", string.signature());
    try std.testing.expectEqual(65535, string.count());
}

test "too large of a string" {
    try std.testing.expectError(StringError.string_too_long, Small.init("g" ** (Small.max_count + 1)));
}

test "copies all bytes of short string" {
    // This is mostly just me verifying how zig does memory.
    // We want string copies to be safe.  Note that the address
    // of `string.slice()` may change if copied, e.g., for
    // `noAlloc` strings.
    var string_src = Small.noAlloc("0123456789abcd");
    string_src.size = 5;

    const string_dst = string_src;
    string_src.size = 14;

    for (string_src.buffer()) |*c| {
        c.* += 10;
    }
    try string_src.expectEquals(Small.noAlloc(":;<=>?@ABCklmn"));
    try string_dst.expectEquals(Small.noAlloc("01234"));
}

test "contains At.start for long string" {
    var string = try Small.init("long string should test this as well");
    defer string.deinit();

    try std.testing.expect(string.contains("lon", common.At.start));
    try std.testing.expect(string.contains("long string should tes", common.At.start));
    try std.testing.expect(string.contains("long string should test this as well", common.At.start));

    try std.testing.expect(!string.contains("long string should test this as well!", common.At.start));
    try std.testing.expect(!string.contains("short string", common.At.start));
    try std.testing.expect(!string.contains("string", common.At.start));
}

test "contains At.start for short string" {
    const string = Small.noAlloc("short string!");

    try std.testing.expect(string.contains("shor", common.At.start));
    try std.testing.expect(string.contains("short strin", common.At.start));
    try std.testing.expect(string.contains("short string!", common.At.start));

    try std.testing.expect(!string.contains("short string!?", common.At.start));
    try std.testing.expect(!string.contains("long string", common.At.start));
    try std.testing.expect(!string.contains("string", common.At.start));
}

test "contains At.end for long string" {
    var string = try Small.init("long string should test this as well");
    defer string.deinit();

    try std.testing.expect(string.contains("ell", common.At.end));
    try std.testing.expect(string.contains("is as well", common.At.end));
    try std.testing.expect(string.contains("long string should test this as well", common.At.end));

    try std.testing.expect(!string.contains("a long string should test this as well", common.At.end));
    try std.testing.expect(!string.contains("not well", common.At.end));
    try std.testing.expect(!string.contains("well!", common.At.end));
}

test "contains At.end for short string" {
    const string = Small.noAlloc("short string!");

    try std.testing.expect(string.contains("ing!", common.At.end));
    try std.testing.expect(string.contains("string!", common.At.end));
    try std.testing.expect(string.contains("short string!", common.At.end));

    try std.testing.expect(!string.contains("1 short string!", common.At.end));
    try std.testing.expect(!string.contains("long string!", common.At.end));
    try std.testing.expect(!string.contains("string", common.At.end));
}
