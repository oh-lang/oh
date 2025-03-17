const common = @import("common.zig");
const DoNothing = @import("do_nothing.zig").DoNothing;
const Node = @import("node.zig").Node;
const Parser = @import("parser.zig").Parser;
const SmallString = @import("string.zig").Small;

const std = @import("std");

// General test structure:
// Organize by topic but within each topic put easier tests near the end.
// This makes easy tests fail last, and they are easier to debug
// if they are at the end of the input.

test "parser multiline function with multiline generic return type" {
    const expected_nodes = [_]Node{
        // [0]:
        Node{ .enclosed = .{ .open = .none, .tab = 0, .start = 1 } }, // \t...\b at tab 0
        Node{ .statement = .{ .node = 38, .next = 0 } }, // statement ... 	 {...}
        Node{ .callable_token = 1 }, // gezerat
        Node{ .enclosed = .{ .open = .paren, .tab = 0, .start = 4 } }, // (...) at tab 0
        Node{ .statement = .{ .node = 5, .next = 0 } }, // statement \t...\b at tab 4
        // [5]:
        Node{ .enclosed = .{ .open = .none, .tab = 4, .start = 6 } }, // \t...\b at tab 4
        Node{ .statement = .{ .node = 9, .next = 10 } }, // statement Argument0 : str
        Node{ .atomic_token = 5 }, // Argument0
        Node{ .callable_token = 9 }, // str
        Node{ .binary = .{ .operator = .op_declare_readonly, .left = 7, .right = 8 } }, // Argument0 : str
        // [10]:
        Node{ .statement = .{ .node = 13, .next = 0 } }, // statement Argument1 : int
        Node{ .atomic_token = 11 }, // Argument1
        Node{ .callable_token = 15 }, // int
        Node{ .binary = .{ .operator = .op_declare_readonly, .left = 11, .right = 12 } }, // Argument1 : int
        Node{ .binary = .{ .operator = .op_access, .left = 2, .right = 3 } }, // gezerat   (...)
        // [15]:
        Node{ .callable_token = 21 }, // gelb
        Node{ .binary = .{ .operator = .op_declare_readonly, .left = 14, .right = 28 } }, // gezerat() : gelb[]
        Node{ .enclosed = .{ .open = .bracket, .tab = 0, .start = 18 } }, // [...] at tab 0
        Node{ .statement = .{ .node = 19, .next = 0 } }, // statement \t...\b at tab 4
        Node{ .enclosed = .{ .open = .none, .tab = 4, .start = 20 } }, // \t...\b at tab 4
        // [20]:
        Node{ .statement = .{ .node = 23, .next = 24 } }, // statement type0 : int
        Node{ .callable_token = 25 }, // type0
        Node{ .callable_token = 29 }, // int
        Node{ .binary = .{ .operator = .op_declare_readonly, .left = 21, .right = 22 } }, // type0 : int
        Node{ .statement = .{ .node = 27, .next = 0 } }, // statement type1 : str
        // [25]:
        Node{ .callable_token = 31 }, // type1
        Node{ .callable_token = 35 }, // str
        Node{ .binary = .{ .operator = .op_declare_readonly, .left = 25, .right = 26 } }, // type1 : str
        Node{ .binary = .{ .operator = .op_access, .left = 15, .right = 17 } }, // gelb   [...]
        Node{ .enclosed = .{ .open = .brace, .tab = 0, .start = 30 } }, // {...} at tab 0
        // [30]:
        Node{ .statement = .{ .node = 31, .next = 0 } }, // statement \t...\b at tab 4
        Node{ .enclosed = .{ .open = .none, .tab = 4, .start = 32 } }, // \t...\b at tab 4
        Node{ .statement = .{ .node = 37, .next = 0 } }, // statement go_bular   (...)
        Node{ .callable_token = 41 }, // go_bular
        Node{ .enclosed = .{ .open = .paren, .tab = 4, .start = 35 } }, // (...) at tab 4
        // [35]:
        Node{ .statement = .{ .node = 36, .next = 0 } }, // statement Argument0
        Node{ .atomic_token = 45 }, // Argument0
        Node{ .binary = .{ .operator = .op_access, .left = 33, .right = 34 } }, // go_bular   (...)
        Node{ .binary = .{ .operator = .op_indent, .left = 16, .right = 29 } }, // ... 	 {...}
        .end, // end
    };
    {
        // One-true-brace
        var parser: Parser = .{};
        defer parser.deinit();
        errdefer parser.debug();
        const file_slice = [_][]const u8{
            "gezerat(",
            "    Argument0: str",
            "    Argument1: int",
            "): gelb[",
            "    type0: int",
            "    type1: str",
            "] {",
            "    go_bular(Argument0)",
            "}",
        };
        try parser.tokenizer.file.appendSlice(&file_slice);

        try parser.complete(DoNothing{});

        try parser.nodes.expectEqualsSlice(&expected_nodes);
        // No tampering done with the file, i.e., no errors.
        try parser.tokenizer.file.expectEqualsSlice(&file_slice);
    }
    {
        // Horstmann
        var parser: Parser = .{};
        defer parser.deinit();
        errdefer parser.debug();
        const file_slice = [_][]const u8{
            "gezerat",
            "(   Argument0: str",
            "    Argument1: int",
            "):  gelb",
            "[   type0: int",
            "    type1: str",
            "]",
            "{   go_bular(Argument0)",
            "}",
        };
        try parser.tokenizer.file.appendSlice(&file_slice);
        common.debugPrint("\n\nSTARTINSDFSDF\n\n\n", .{});

        try parser.complete(DoNothing{});

        try parser.nodes.expectEqualsSlice(&expected_nodes);
        // No tampering done with the file, i.e., no errors.
        try parser.tokenizer.file.expectEqualsSlice(&file_slice);
    }
    {
        // No braces
        var parser: Parser = .{};
        defer parser.deinit();
        errdefer parser.debug();
        const file_slice = [_][]const u8{
            "gezerat",
            "(   Argument0: str",
            "    Argument1: int",
            "):  gelb",
            "[   type0: int",
            "    type1: str",
            "]",
            "    go_bular(Argument0)",
        };
        try parser.tokenizer.file.appendSlice(&file_slice);

        try parser.complete(DoNothing{});

        try parser.nodes.expectEqualsSlice(&[_]Node{
            // [0]:
            Node{ .enclosed = .{ .open = .none, .tab = 0, .start = 1 } }, // \t...\b at tab 0
            Node{ .statement = .{ .node = 36, .next = 0 } }, // statement ... 	 \t...\b
            Node{ .callable_token = 1 }, // gezerat
            Node{ .enclosed = .{ .open = .paren, .tab = 0, .start = 4 } }, // (...) at tab 0
            Node{ .statement = .{ .node = 5, .next = 0 } }, // statement \t...\b at tab 4
            // [5]:
            Node{ .enclosed = .{ .open = .none, .tab = 4, .start = 6 } }, // \t...\b at tab 4
            Node{ .statement = .{ .node = 9, .next = 10 } }, // statement Argument0 : str
            Node{ .atomic_token = 5 }, // Argument0
            Node{ .callable_token = 9 }, // str
            Node{ .binary = .{ .operator = .op_declare_readonly, .left = 7, .right = 8 } }, // Argument0 : str
            // [10]:
            Node{ .statement = .{ .node = 13, .next = 0 } }, // statement Argument1 : int
            Node{ .atomic_token = 11 }, // Argument1
            Node{ .callable_token = 15 }, // int
            Node{ .binary = .{ .operator = .op_declare_readonly, .left = 11, .right = 12 } }, // Argument1 : int
            Node{ .binary = .{ .operator = .op_access, .left = 2, .right = 3 } }, // gezerat   (...)
            // [15]:
            Node{ .callable_token = 21 }, // gelb
            Node{ .binary = .{ .operator = .op_declare_readonly, .left = 14, .right = 28 } }, // gezerat() : gelb[]
            Node{ .enclosed = .{ .open = .bracket, .tab = 0, .start = 18 } }, // [...] at tab 0
            Node{ .statement = .{ .node = 19, .next = 0 } }, // statement \t...\b at tab 4
            Node{ .enclosed = .{ .open = .none, .tab = 4, .start = 20 } }, // \t...\b at tab 4
            // [20]:
            Node{ .statement = .{ .node = 23, .next = 24 } }, // statement type0 : int
            Node{ .callable_token = 25 }, // type0
            Node{ .callable_token = 29 }, // int
            Node{ .binary = .{ .operator = .op_declare_readonly, .left = 21, .right = 22 } }, // type0 : int
            Node{ .statement = .{ .node = 27, .next = 0 } }, // statement type1 : str
            // [25]:
            Node{ .callable_token = 31 }, // type1
            Node{ .callable_token = 35 }, // str
            Node{ .binary = .{ .operator = .op_declare_readonly, .left = 25, .right = 26 } }, // type1 : str
            Node{ .binary = .{ .operator = .op_access, .left = 15, .right = 17 } }, // gelb   [...]
            Node{ .enclosed = .{ .open = .none, .tab = 4, .start = 30 } }, // \t...\b at tab 4
            // [30]:
            Node{ .statement = .{ .node = 35, .next = 0 } }, // statement go_bular   (...)
            Node{ .callable_token = 39 }, // go_bular
            Node{ .enclosed = .{ .open = .paren, .tab = 4, .start = 33 } }, // (...) at tab 4
            Node{ .statement = .{ .node = 34, .next = 0 } }, // statement Argument0
            Node{ .atomic_token = 43 }, // Argument0
            // [35]:
            Node{ .binary = .{ .operator = .op_access, .left = 31, .right = 32 } }, // go_bular   (...)
            Node{ .binary = .{ .operator = .op_indent, .left = 16, .right = 29 } }, // ... 	 \t...\b
            .end, // end
        });
        // No tampering done with the file, i.e., no errors.
        try parser.tokenizer.file.expectEqualsSlice(&file_slice);
    }
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

test "member access function calls" {
    var parser: Parser = .{};
    defer parser.deinit();
    errdefer parser.debug();
    const file_slice = [_][]const u8{
        "Frebly balsu(Brimly: 394)",
    };
    try parser.tokenizer.file.appendSlice(&file_slice);

    try parser.complete(DoNothing{});

    try parser.nodes.expectEqualsSlice(&[_]Node{
        // [0]:
        Node{ .enclosed = .{ .open = .none, .tab = 0, .start = 1 } }, // \t...\b at tab 0
        Node{ .statement = .{ .node = 10, .next = 0 } }, // statement ...   (...)
        Node{ .atomic_token = 1 }, // Frebly
        Node{ .callable_token = 3 }, // balsu
        Node{ .binary = .{ .operator = .op_access, .left = 2, .right = 3 } }, // Frebly   balsu
        // [5]:
        Node{ .enclosed = .{ .open = .paren, .tab = 0, .start = 6 } }, // (...) at tab 0
        Node{ .statement = .{ .node = 9, .next = 0 } }, // statement Brimly : 394
        Node{ .atomic_token = 7 }, // Brimly
        Node{ .atomic_token = 11 }, // 394
        Node{ .binary = .{ .operator = .op_declare_readonly, .left = 7, .right = 8 } }, // Brimly : 394
        // [10]:
        Node{ .binary = .{ .operator = .op_access, .left = 4, .right = 5 } }, // ...   (...)
        .end, // end
    });
    // No errors when parsing:
    try parser.tokenizer.file.expectEqualsSlice(&file_slice);
}

test "simple function calls" {
    var parser: Parser = .{};
    defer parser.deinit();
    errdefer parser.debug();
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
