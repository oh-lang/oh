const common = @import("common.zig");
const DoNothing = @import("do_nothing.zig").DoNothing;
const Node = @import("node.zig").Node;
const Parser = @import("parser.zig").Parser;
const SmallString = @import("string.zig").Small;

const std = @import("std");

// TODO: add `SomeCondition { some_bracket_logic() }`
//      braces need to be lower priority than `access`.
//      same for `open = .none` blocks.
//&|if MyCondition
//&|    do_something()

// TODO: test to error out on spacing indented to +8 or more at start of file
// TODO: test to see what happens when indented to +4 at start of file

//test "parsing quotes" {
//    {
//    var parser: Parser = .{};
//    defer parser.deinit();
//    errdefer {
//        common.debugPrint("# file:\n", parser.tokenizer.file);
//    }
//    const file_slice = [_][]const u8{
//        "'hi quote'",
//    };
//    try parser.tokenizer.file.appendSlice(&file_slice);
//
//    try parser.complete(DoNothing{});
//
//    try parser.nodes.expectEqualsSlice(&[_]Node{
//        .end,
//    });
//    // No tampering done with the file, i.e., no errors.
//    try parser.tokenizer.file.expectEqualsSlice(&file_slice);
//    }
//
//}

test "parser simple expressions" {
    var parser: Parser = .{};
    defer parser.deinit();
    errdefer {
        common.debugPrint("# file:\n", parser.tokenizer.file);
    }
    const file_slice = [_][]const u8{
        "3.456",
        "    hello_you",
        "+1.234",
        "    -5.678",
        "    $$$Foe",
        "        Fum--",
    };
    try parser.tokenizer.file.appendSlice(&file_slice);

    try parser.complete(DoNothing{});

    try parser.nodes.expectEqualsSlice(&[_]Node{
        // [0]:
        Node{ .enclosed = .{ .open = .none, .tab = 0, .start = 1 } },
        Node{ .statement = .{ .node = 6, .next = 7 } }, // statement of 3.456 + indent
        Node{ .atomic_token = 1 }, // 3.456
        Node{ .enclosed = .{ .open = .none, .tab = 4, .start = 4 } }, // indent with hello_you
        Node{ .statement = .{ .node = 5, .next = 0 } }, // hello_you statement
        // [5]:
        Node{ .callable_token = 3 }, // hello_you
        Node{ .binary = .{ .operator = .op_indent, .left = 2, .right = 3 } }, // 3.456 indent
        Node{ .statement = .{ .node = 22, .next = 0 } }, // statement of +1.234 + indent
        Node{ .prefix = .{ .operator = .op_plus, .node = 9 } }, // +1.234
        Node{ .atomic_token = 7 }, // 1.234
        // [10]:
        Node{ .enclosed = .{ .open = .none, .tab = 4, .start = 11 } }, // block with -5.678 starting
        Node{ .statement = .{ .node = 12, .next = 14 } }, // -5.678 statement
        Node{ .prefix = .{ .operator = .op_minus, .node = 13 } }, // -5.678
        Node{ .atomic_token = 11 }, // 5.678
        Node{ .statement = .{ .node = 21, .next = 0 } }, // $$$Foe indent statement
        // [15]:
        Node{ .prefix = .{ .operator = .op_lambda3, .node = 16 } }, // $$$Foe
        Node{ .atomic_token = 15 }, // Foe
        Node{ .enclosed = .{ .open = .none, .tab = 8, .start = 18 } },
        Node{ .statement = .{ .node = 20, .next = 0 } },
        Node{ .atomic_token = 17 }, // Fum
        // [20]:
        Node{ .postfix = .{ .operator = .op_decrement, .node = 19 } },
        Node{ .binary = .{ .operator = .op_indent, .left = 15, .right = 17 } },
        Node{ .binary = .{ .operator = .op_indent, .left = 8, .right = 10 } },
        .end,
    });
    // No tampering done with the file, i.e., no errors.
    try parser.tokenizer.file.expectEqualsSlice(&file_slice);
}

test "generic types" {
    var parser: Parser = .{};
    defer parser.deinit();
    errdefer {
        common.debugPrint("# file:\n", parser.tokenizer.file);
    }
    const file_slice = [_][]const u8{
        "container54[of; i1234, at. str5[qusp], array[dongle]]",
    };
    try parser.tokenizer.file.appendSlice(&file_slice);

    try parser.complete(DoNothing{});

    try parser.nodes.expectEqualsSlice(&[_]Node{
        // [0]:
        Node{ .enclosed = .{ .open = .none, .tab = 0, .start = 1 } },
        Node{ .statement = .{ .node = 22, .next = 0 } },
        Node{ .callable_token = 1 }, // container54
        Node{ .enclosed = .{ .open = .bracket, .tab = 0, .start = 4 } },
        Node{ .statement = .{ .node = 7, .next = 8 } },
        // [5]:
        Node{ .callable_token = 5 }, // of
        Node{ .callable_token = 9 }, // i1234
        Node{ .binary = .{ .operator = .op_declare_writable, .left = 5, .right = 6 } },
        Node{ .statement = .{ .node = 11, .next = 16 } },
        Node{ .callable_token = 13 }, // at
        // [10]:
        Node{ .callable_token = 17 }, // str5
        Node{ .binary = .{ .operator = .op_declare_temporary, .left = 9, .right = 15 } },
        Node{ .enclosed = .{ .open = .bracket, .tab = 0, .start = 13 } },
        Node{ .statement = .{ .node = 14, .next = 0 } },
        Node{ .callable_token = 21 }, // qusp
        // [15]:
        Node{ .binary = .{ .operator = .op_access, .left = 10, .right = 12 } },
        Node{ .statement = .{ .node = 21, .next = 0 } },
        Node{ .callable_token = 27 }, // array
        Node{ .enclosed = .{ .open = .bracket, .tab = 0, .start = 19 } },
        Node{ .statement = .{ .node = 20, .next = 0 } },
        // [20]:
        Node{ .callable_token = 31 }, // dongle
        Node{ .binary = .{ .operator = .op_access, .left = 17, .right = 18 } },
        Node{ .binary = .{ .operator = .op_access, .left = 2, .right = 3 } },
        .end,
    });
    // No errors when parsing:
    try parser.tokenizer.file.expectEqualsSlice(&file_slice);
}

test "simple function calls" {
    var parser: Parser = .{};
    defer parser.deinit();
    errdefer {
        common.debugPrint("# file:\n", parser.tokenizer.file);
    }
    const file_slice = [_][]const u8{
        "superb(Brepus. 161, Canyon; Noynac, Candid)",
    };
    try parser.tokenizer.file.appendSlice(&file_slice);

    try parser.complete(DoNothing{});

    try parser.nodes.expectEqualsSlice(&[_]Node{
        // [0]:
        Node{ .enclosed = .{ .open = .none, .tab = 0, .start = 1 } },
        Node{ .statement = .{ .node = 14, .next = 0 } },
        Node{ .callable_token = 1 }, // superb
        Node{ .enclosed = .{ .open = .paren, .tab = 0, .start = 4 } },
        Node{ .statement = .{ .node = 7, .next = 8 } },
        // [5]:
        Node{ .atomic_token = 5 }, // Brepus
        Node{ .atomic_token = 9 }, // 161
        Node{ .binary = .{ .operator = .op_declare_temporary, .left = 5, .right = 6 } },
        Node{ .statement = .{ .node = 11, .next = 12 } },
        Node{ .atomic_token = 13 }, // Canyon
        // [10]:
        Node{ .atomic_token = 17 }, // Noynac
        Node{ .binary = .{ .operator = .op_declare_writable, .left = 9, .right = 10 } },
        Node{ .statement = .{ .node = 13, .next = 0 } },
        Node{ .atomic_token = 21 }, // Candid
        Node{ .binary = .{ .operator = .op_access, .left = 2, .right = 3 } },
        // [15]:
        .end,
    });
    // No errors when parsing:
    try parser.tokenizer.file.expectEqualsSlice(&file_slice);
}

test "generic function calls" {
    var parser: Parser = .{};
    defer parser.deinit();
    errdefer {
        common.debugPrint("# file:\n", parser.tokenizer.file);
    }
    const file_slice = [_][]const u8{
        "fungus[type1: t_array[str7], type2, type3; i64](Life: 17, Cardio!, Fritz; foo_fritz)",
    };
    try parser.tokenizer.file.appendSlice(&file_slice);

    try parser.complete(DoNothing{});

    try parser.nodes.expectEqualsSlice(&[_]Node{
        // [0]:
        Node{ .enclosed = .{ .open = .none, .tab = 0, .start = 1 } },
        Node{ .statement = .{ .node = 31, .next = 0 } },
        Node{ .callable_token = 1 }, // fungus
        Node{ .enclosed = .{ .open = .bracket, .tab = 0, .start = 4 } },
        Node{ .statement = .{ .node = 7, .next = 12 } },
        // [5]:
        Node{ .callable_token = 5 }, // type1
        Node{ .callable_token = 9 }, // t_array
        Node{ .binary = .{ .operator = .op_declare_readonly, .left = 5, .right = 11 } },
        Node{ .enclosed = .{ .open = .bracket, .tab = 0, .start = 9 } },
        Node{ .statement = .{ .node = 10, .next = 0 } },
        // [10]:
        Node{ .callable_token = 13 }, // str7
        Node{ .binary = .{ .operator = .op_access, .left = 6, .right = 8 } },
        Node{ .statement = .{ .node = 13, .next = 14 } },
        Node{ .callable_token = 19 }, // type2
        Node{ .statement = .{ .node = 17, .next = 0 } },
        // [15]:
        Node{ .callable_token = 23 }, // type3
        Node{ .callable_token = 27 }, // i64
        Node{ .binary = .{ .operator = .op_declare_writable, .left = 15, .right = 16 } },
        Node{ .binary = .{ .operator = .op_access, .left = 2, .right = 3 } },
        Node{ .enclosed = .{ .open = .paren, .tab = 0, .start = 20 } },
        // [20]:
        Node{ .statement = .{ .node = 23, .next = 24 } },
        Node{ .atomic_token = 33 }, // Life
        Node{ .atomic_token = 37 }, // 17
        Node{ .binary = .{ .operator = .op_declare_readonly, .left = 21, .right = 22 } },
        Node{ .statement = .{ .node = 26, .next = 27 } },
        // [25]:
        Node{ .atomic_token = 41 }, // Cardio
        Node{ .postfix = .{ .operator = .op_not, .node = 25 } },
        Node{ .statement = .{ .node = 30, .next = 0 } },
        Node{ .atomic_token = 47 }, // Fritz
        Node{ .callable_token = 51 }, // foo_fritz
        // [30]:
        Node{ .binary = .{ .operator = .op_declare_writable, .left = 28, .right = 29 } },
        Node{ .binary = .{ .operator = .op_access, .left = 18, .right = 19 } },
        .end,
    });
    try parser.tokenizer.file.expectEqualsSlice(&file_slice);
}

test "simple parentheses, brackets, and braces" {
    {
        var parser: Parser = .{};
        defer parser.deinit();
        errdefer {
            common.debugPrint("# file:\n", parser.tokenizer.file);
        }
        try parser.tokenizer.file.lines.append(try SmallString.init("+(Wow, Great)"));

        try parser.complete(DoNothing{});

        try parser.nodes.expectEqualsSlice(&[_]Node{
            Node{ .enclosed = .{ .open = .none, .tab = 0, .start = 1 } },
            Node{ .statement = .{ .node = 2, .next = 0 } },
            Node{ .prefix = .{ .operator = .op_plus, .node = 3 } },
            Node{ .enclosed = .{ .open = .paren, .tab = 0, .start = 4 } },
            Node{ .statement = .{ .node = 5, .next = 6 } },
            // [5]:
            Node{ .atomic_token = 5 }, // Wow
            Node{ .statement = .{ .node = 7, .next = 0 } },
            Node{ .atomic_token = 9 }, // Great
            .end,
        });
    }
    {
        var parser: Parser = .{};
        defer parser.deinit();
        errdefer {
            common.debugPrint("# file:\n", parser.tokenizer.file);
        }
        try parser.tokenizer.file.lines.append(try SmallString.init("[wow, jam, time]!"));

        try parser.complete(DoNothing{});

        try parser.nodes.expectEqualsSlice(&[_]Node{
            Node{ .enclosed = .{ .open = .none, .tab = 0, .start = 1 } },
            Node{ .statement = .{ .node = 9, .next = 0 } },
            Node{ .enclosed = .{ .open = .bracket, .tab = 0, .start = 3 } },
            Node{ .statement = .{ .node = 4, .next = 5 } },
            Node{ .callable_token = 3 }, // wow
            // [5]:
            Node{ .statement = .{ .node = 6, .next = 7 } },
            Node{ .callable_token = 7 }, // jam
            Node{ .statement = .{ .node = 8, .next = 0 } },
            Node{ .callable_token = 11 }, // time
            Node{ .postfix = .{ .operator = .op_not, .node = 2 } },
            // [10]:
            .end,
        });
        // No errors in attempts to parse `callable`.
        try parser.tokenizer.file.expectEqualsSlice(&[_][]const u8{
            "[wow, jam, time]!",
        });
    }
    {
        var parser: Parser = .{};
        defer parser.deinit();
        errdefer {
            common.debugPrint("# file:\n", parser.tokenizer.file);
        }
        try parser.tokenizer.file.lines.append(try SmallString.init("{Boo: 33, hoo: 123 + 44}-57"));

        try parser.complete(DoNothing{});

        try parser.nodes.expectEqualsSlice(&[_]Node{
            // [0]:
            Node{ .enclosed = .{ .open = .none, .tab = 0, .start = 1 } },
            Node{ .statement = .{ .node = 14, .next = 0 } },
            Node{ .enclosed = .{ .open = .brace, .tab = 0, .start = 3 } },
            Node{ .statement = .{ .node = 6, .next = 7 } },
            Node{ .atomic_token = 3 }, // Boo
            // [5]:
            Node{ .atomic_token = 7 }, //33
            Node{ .binary = .{ .operator = .op_declare_readonly, .left = 4, .right = 5 } },
            Node{ .statement = .{ .node = 10, .next = 0 } },
            Node{ .callable_token = 11 }, // hoo
            Node{ .atomic_token = 15 }, // 123
            // [10]:
            Node{ .binary = .{ .operator = .op_declare_readonly, .left = 8, .right = 12 } },
            Node{ .atomic_token = 19 }, // 44
            Node{ .binary = .{ .operator = .op_plus, .left = 9, .right = 11 } },
            Node{ .atomic_token = 25 }, // 57
            Node{ .binary = .{ .operator = .op_minus, .left = 2, .right = 13 } },
            // [15]:
            .end,
        });
        // Attempts to parse generics or arguments for `callable` don't add errors:
        try parser.tokenizer.file.expectEqualsSlice(&[_][]const u8{
            "{Boo: 33, hoo: 123 + 44}-57",
        });
    }
}

test "trailing commas are ok" {
    {
        var parser: Parser = .{};
        defer parser.deinit();
        errdefer {
            common.debugPrint("# file:\n", parser.tokenizer.file);
        }
        try parser.tokenizer.file.lines.append(try SmallString.init("(C0, C1, C2,)"));

        try parser.complete(DoNothing{});

        try parser.nodes.expectEqualsSlice(&[_]Node{
            // [0]:
            Node{ .enclosed = .{ .open = .none, .tab = 0, .start = 1 } },
            Node{ .statement = .{ .node = 2, .next = 0 } },
            Node{ .enclosed = .{ .open = .paren, .tab = 0, .start = 3 } },
            Node{ .statement = .{ .node = 4, .next = 5 } },
            Node{ .atomic_token = 3 }, // C0
            // [5]:
            Node{ .statement = .{ .node = 6, .next = 7 } },
            Node{ .atomic_token = 7 }, // C1
            Node{ .statement = .{ .node = 8, .next = 0 } },
            Node{ .atomic_token = 11 }, // C2
            .end,
        });
    }
    {
        var parser: Parser = .{};
        defer parser.deinit();
        errdefer {
            common.debugPrint("# file:\n", parser.tokenizer.file);
        }
        try parser.tokenizer.file.lines.append(try SmallString.init("[Bk0, Bk1,]-761"));

        try parser.complete(DoNothing{});

        try parser.nodes.expectEqualsSlice(&[_]Node{
            // [0]:
            Node{ .enclosed = .{ .open = .none, .tab = 0, .start = 1 } },
            Node{ .statement = .{ .node = 8, .next = 0 } },
            Node{ .enclosed = .{ .open = .bracket, .tab = 0, .start = 3 } },
            Node{ .statement = .{ .node = 4, .next = 5 } },
            Node{ .atomic_token = 3 }, // Bk0
            // [5]:
            Node{ .statement = .{ .node = 6, .next = 0 } },
            Node{ .atomic_token = 7 }, // Bk1
            Node{ .atomic_token = 15 }, // 761
            Node{ .binary = .{ .operator = .op_minus, .left = 2, .right = 7 } },
            .end,
        });
    }
    {
        var parser: Parser = .{};
        defer parser.deinit();
        errdefer {
            common.debugPrint("# file:\n", parser.tokenizer.file);
        }
        try parser.tokenizer.file.lines.append(try SmallString.init("{Bc0,}+Abcdef"));

        try parser.complete(DoNothing{});

        try parser.nodes.expectEqualsSlice(&[_]Node{
            // [0]:
            Node{ .enclosed = .{ .open = .none, .tab = 0, .start = 1 } },
            Node{ .statement = .{ .node = 6, .next = 0 } },
            Node{ .enclosed = .{ .open = .brace, .tab = 0, .start = 3 } },
            Node{ .statement = .{ .node = 4, .next = 0 } },
            Node{ .atomic_token = 3 }, // Bc0
            // [5]:
            Node{ .atomic_token = 11 }, // Abcdef
            Node{ .binary = .{ .operator = .op_plus, .left = 2, .right = 5 } },
            .end,
        });
    }
}

// TODO: error tests, e.g., "cannot postfix this"
