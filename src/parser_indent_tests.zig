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

test "parser deindent indents" {
    const expected_nodes = [_]Node{
        // [0]:
        Node{ .enclosed = .{ .open = .none, .tab = 0, .start = 1 } }, // \t...\b at tab 0
        Node{ .statement = .{ .node = 18, .next = 0 } }, // statement ... : {...}
        Node{ .callable_token = 1 }, // wowza_d
        Node{ .enclosed = .{ .open = .paren, .tab = 0, .start = 0 } }, // (...) at tab 0
        Node{ .binary = .{ .operator = .op_access, .left = 2, .right = 3 } }, // wowza_d   (...)
        // [5]:
        Node{ .enclosed = .{ .open = .brace, .tab = 0, .start = 6 } }, // {...} at tab 0
        Node{ .statement = .{ .node = 7, .next = 0 } }, // statement \t...\b at tab 4
        Node{ .enclosed = .{ .open = .none, .tab = 4, .start = 8 } }, // \t...\b at tab 4
        Node{ .statement = .{ .node = 11, .next = 12 } }, // statement spin_d   (...)
        Node{ .callable_token = 11 }, // spin_d
        // [10]:
        Node{ .enclosed = .{ .open = .paren, .tab = 4, .start = 0 } }, // (...) at tab 4
        Node{ .binary = .{ .operator = .op_access, .left = 9, .right = 10 } }, // spin_d   (...)
        Node{ .statement = .{ .node = 13, .next = 0 } }, // statement [...] at tab 4
        Node{ .enclosed = .{ .open = .bracket, .tab = 4, .start = 14 } }, // [...] at tab 4
        Node{ .statement = .{ .node = 15, .next = 16 } }, // statement 1
        // [15]:
        Node{ .atomic_token = 19 }, // 1
        Node{ .statement = .{ .node = 17, .next = 0 } }, // statement 2
        Node{ .atomic_token = 21 }, // 2
        Node{ .binary = .{ .operator = .op_declare_readonly, .left = 4, .right = 5 } }, // ... : {...}
        .end, // end
    };
    {
        // One-true-brace
        var parser: Parser = .{};
        defer parser.deinit();
        errdefer parser.debug();
        const file_slice = [_][]const u8{
            "wowza_d(): {",
            "    spin_d()",
            "    [1",
            "    2]",
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
            "wowza_d():",
            "{   spin_d()",
            "[   1",
            "    2",
            "]",
            "}",
        };
        try parser.tokenizer.file.appendSlice(&file_slice);
        common.debugPrint("\nSTASRTING STUFF\n\n", .{});

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
            "wowza_d():",
            "    spin_d()",
            "[   1",
            "    2",
            "]",
        };
        try parser.tokenizer.file.appendSlice(&file_slice);

        try parser.complete(DoNothing{});

        try parser.nodes.expectEqualsSlice(&[_]Node{
            // [0]:
            Node{ .enclosed = .{ .open = .none, .tab = 0, .start = 1 } }, // \t...\b at tab 0
            Node{ .statement = .{ .node = 16, .next = 0 } }, // statement ... : \t...\b
            Node{ .callable_token = 1 }, // wowza_d
            Node{ .enclosed = .{ .open = .paren, .tab = 0, .start = 0 } }, // (...) at tab 0
            Node{ .binary = .{ .operator = .op_access, .left = 2, .right = 3 } }, // wowza_d   (...)
            // [5]:
            Node{ .enclosed = .{ .open = .none, .tab = 4, .start = 6 } }, // \t...\b at tab 4
            Node{ .statement = .{ .node = 9, .next = 10 } }, // statement spin_d   (...)
            Node{ .callable_token = 9 }, // spin_d
            Node{ .enclosed = .{ .open = .paren, .tab = 4, .start = 0 } }, // (...) at tab 4
            Node{ .binary = .{ .operator = .op_access, .left = 7, .right = 8 } }, // spin_d   (...)
            // [10]:
            Node{ .statement = .{ .node = 11, .next = 0 } }, // statement [...] at tab 4
            Node{ .enclosed = .{ .open = .bracket, .tab = 4, .start = 12 } }, // [...] at tab 4
            Node{ .statement = .{ .node = 13, .next = 14 } }, // statement 1
            Node{ .atomic_token = 17 }, // 1
            Node{ .statement = .{ .node = 15, .next = 0 } }, // statement 2
            // [15]:
            Node{ .atomic_token = 19 }, // 2
            Node{ .binary = .{ .operator = .op_declare_readonly, .left = 4, .right = 5 } }, // ... : \t...\b
            .end, // end
        });
        // No tampering done with the file, i.e., no errors.
        try parser.tokenizer.file.expectEqualsSlice(&file_slice);
    }
}

test "parser indent nesting" {
    const expected_nodes = [_]Node{
        // [0]:
        Node{ .enclosed = .{ .open = .none, .tab = 0, .start = 1 } },
        Node{ .statement = .{ .node = 30, .next = 0 } }, // first (only) statement in root block
        Node{ .callable_token = 1 }, // greet_thee
        Node{ .enclosed = .{ .open = .paren, .tab = 0, .start = 0 } }, // ()
        Node{ .binary = .{ .operator = .op_access, .left = 2, .right = 3 } }, // greet_thee ()
        // [5]:
        Node{ .enclosed = .{ .open = .brace, .tab = 0, .start = 6 } }, // {...}
        Node{ .statement = .{ .node = 7, .next = 0 } }, // outer indent statement
        Node{ .enclosed = .{ .open = .none, .tab = 4, .start = 8 } }, // outer indent
        Node{ .statement = .{ .node = 17, .next = 18 } }, // print statement
        Node{ .callable_token = 11 }, // print
        // [10]:
        Node{ .enclosed = .{ .open = .paren, .tab = 4, .start = 11 } }, // (...) inside print
        Node{ .statement = .{ .node = 12, .next = 0 } }, // statement inside print
        Node{ .enclosed = .{ .open = .none, .tab = 8, .start = 13 } }, // indent inside print
        Node{ .statement = .{ .node = 14, .next = 15 } }, // first statement in print indent
        Node{ .atomic_token = 15 }, // 99731
        // [15]:
        Node{ .statement = .{ .node = 16, .next = 0 } }, // second statement in print indent
        Node{ .atomic_token = 17 }, // World
        Node{ .binary = .{ .operator = .op_access, .left = 9, .right = 10 } }, // print (...)
        Node{ .statement = .{ .node = 29, .next = 0 } }, // wow54 statement
        Node{ .callable_token = 21 }, // wow54
        // [20]:
        Node{ .enclosed = .{ .open = .paren, .tab = 4, .start = 21 } }, // wow54 paren
        Node{ .statement = .{ .node = 22, .next = 0 } }, // inside wow54(...) statement
        Node{ .enclosed = .{ .open = .bracket, .tab = 4, .start = 23 } }, // [...]
        Node{ .statement = .{ .node = 24, .next = 0 } }, // inside wow54([...]) statement
        Node{ .enclosed = .{ .open = .none, .tab = 8, .start = 25 } }, // indent inside ([...])
        // [25]:
        Node{ .statement = .{ .node = 26, .next = 27 } }, // first statement inside ([...])
        Node{ .atomic_token = 27 }, // 57973
        Node{ .statement = .{ .node = 28, .next = 0 } }, // second statement inside ([...])
        Node{ .atomic_token = 29 }, // 67974
        Node{ .binary = .{ .operator = .op_access, .left = 19, .right = 20 } }, // wow (...)
        // [30]:
        Node{ .binary = .{ .operator = .op_declare_readonly, .left = 4, .right = 5 } }, // :
        .end,
    };
    {
        // One-true-brace
        var parser: Parser = .{};
        defer parser.deinit();
        errdefer {
            common.debugPrint("# file:\n", parser.tokenizer.file);
            common.debugPrint("# nodes:\n", parser.nodes);
        }
        const file_slice = [_][]const u8{
            "greet_thee(): {",
            "    print(",
            "        99731",
            "        World",
            "    )",
            "    wow54([",
            "        57973",
            "        67974",
            "    ])",
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
        errdefer {
            common.debugPrint("# file:\n", parser.tokenizer.file);
            common.debugPrint("# nodes:\n", parser.nodes);
        }
        const file_slice = [_][]const u8{
            "greet_thee():",
            "{   print",
            "    (   99731",
            "        World",
            "    )",
            "    wow54",
            "    ([  57973",
            "        67974",
            "    ])",
            "}",
        };
        try parser.tokenizer.file.appendSlice(&file_slice);

        try parser.complete(DoNothing{});

        try parser.nodes.expectEqualsSlice(&expected_nodes);
        // No tampering done with the file, i.e., no errors.
        try parser.tokenizer.file.expectEqualsSlice(&file_slice);
    }
}

test "parser simple bracket indent" {
    const expected_nodes = [_]Node{
        // [0]:
        Node{ .enclosed = .{ .open = .none, .tab = 0, .start = 1 } },
        Node{ .statement = .{ .node = 8, .next = 0 } },
        Node{ .atomic_token = 1 }, // Bart4
        Node{ .enclosed = .{ .open = .bracket, .tab = 0, .start = 4 } },
        Node{ .statement = .{ .node = 5, .next = 0 } },
        // [5]:
        Node{ .enclosed = .{ .open = .none, .tab = 4, .start = 6 } },
        Node{ .statement = .{ .node = 7, .next = 0 } },
        Node{ .atomic_token = 5 }, // Bented4
        Node{ .binary = .{ .operator = .op_access, .left = 2, .right = 3 } },
        .end,
    };
    {
        var parser: Parser = .{};
        defer parser.deinit();
        errdefer {
            common.debugPrint("# file:\n", parser.tokenizer.file);
        }
        const file_slice = [_][]const u8{
            "Bart4",
            "[   Bented4",
            "]",
        };
        try parser.tokenizer.file.appendSlice(&file_slice);

        try parser.complete(DoNothing{});

        try parser.nodes.expectEqualsSlice(&expected_nodes);
        // No tampering done with the file, i.e., no errors.
        try parser.tokenizer.file.expectEqualsSlice(&file_slice);
    }
    {
        var parser: Parser = .{};
        defer parser.deinit();
        errdefer {
            common.debugPrint("# file:\n", parser.tokenizer.file);
        }
        const file_slice = [_][]const u8{
            "Bart4[",
            "    Bented4",
            "]",
        };
        try parser.tokenizer.file.appendSlice(&file_slice);

        try parser.complete(DoNothing{});

        try parser.nodes.expectEqualsSlice(&expected_nodes);
        // No tampering done with the file, i.e., no errors.
        try parser.tokenizer.file.expectEqualsSlice(&file_slice);
    }
}

test "parser simple paren indent" {
    const expected_nodes = [_]Node{
        // [0]:
        Node{ .enclosed = .{ .open = .none, .tab = 0, .start = 1 } },
        Node{ .statement = .{ .node = 9, .next = 0 } },
        Node{ .atomic_token = 1 }, // Bart3
        Node{ .enclosed = .{ .open = .paren, .tab = 0, .start = 4 } },
        Node{ .statement = .{ .node = 5, .next = 0 } },
        // [5]:
        Node{ .enclosed = .{ .open = .none, .tab = 4, .start = 6 } },
        Node{ .statement = .{ .node = 7, .next = 0 } },
        Node{ .prefix = .{ .operator = .op_plus, .node = 8 } },
        Node{ .atomic_token = 7 }, // Bented3
        Node{ .binary = .{ .operator = .op_access, .left = 2, .right = 3 } },
        // [10]:
        .end,
    };
    {
        var parser: Parser = .{};
        defer parser.deinit();
        errdefer {
            common.debugPrint("# file:\n", parser.tokenizer.file);
        }
        const file_slice = [_][]const u8{
            "Bart3",
            "(   +Bented3",
            ")",
        };
        try parser.tokenizer.file.appendSlice(&file_slice);

        try parser.complete(DoNothing{});

        try parser.nodes.expectEqualsSlice(&expected_nodes);
        // No tampering done with the file, i.e., no errors.
        try parser.tokenizer.file.expectEqualsSlice(&file_slice);
    }
    {
        var parser: Parser = .{};
        defer parser.deinit();
        errdefer {
            common.debugPrint("# file:\n", parser.tokenizer.file);
        }
        const file_slice = [_][]const u8{
            "Bart3(",
            "    +Bented3",
            ")",
        };
        try parser.tokenizer.file.appendSlice(&file_slice);

        try parser.complete(DoNothing{});

        try parser.nodes.expectEqualsSlice(&expected_nodes);
        // No tampering done with the file, i.e., no errors.
        try parser.tokenizer.file.expectEqualsSlice(&file_slice);
    }
}

// TODO: implicit blocks
test "parser simple explicit brace block" {
    const expected_nodes = [_]Node{
        // [0]:
        Node{ .enclosed = .{ .open = .none, .tab = 0, .start = 1 } },
        Node{ .statement = .{ .node = 10, .next = 0 } }, // root block first (only) statement
        Node{ .atomic_token = 1 }, // Bart5
        Node{ .enclosed = .{ .open = .brace, .tab = 0, .start = 4 } }, // {...}
        Node{ .statement = .{ .node = 5, .next = 0 } }, // internal statement
        // [5]:
        Node{ .enclosed = .{ .open = .none, .tab = 4, .start = 6 } }, // indent
        Node{ .statement = .{ .node = 7, .next = 8 } }, // Bented51 statement
        Node{ .atomic_token = 7 }, // Bented51
        Node{ .statement = .{ .node = 9, .next = 0 } }, // Bented52 statement
        Node{ .atomic_token = 9 }, // Bented52
        // [10]:
        Node{ .binary = .{ .operator = .op_declare_readonly, .left = 2, .right = 3 } }, // :
        .end,
    };
    {
        var parser: Parser = .{};
        defer parser.deinit();
        errdefer {
            common.debugPrint("# file:\n", parser.tokenizer.file);
        }
        const file_slice = [_][]const u8{
            "Bart5:",
            "{   Bented51",
            "    Bented52",
            "}",
        };
        try parser.tokenizer.file.appendSlice(&file_slice);

        try parser.complete(DoNothing{});

        try parser.nodes.expectEqualsSlice(&expected_nodes);
        // No tampering done with the file, i.e., no errors.
        try parser.tokenizer.file.expectEqualsSlice(&file_slice);
    }
    {
        var parser: Parser = .{};
        defer parser.deinit();
        errdefer {
            common.debugPrint("# file:\n", parser.tokenizer.file);
        }
        const file_slice = [_][]const u8{
            "Bart5: {",
            "    Bented51",
            "    Bented52",
            "}",
        };
        try parser.tokenizer.file.appendSlice(&file_slice);

        try parser.complete(DoNothing{});

        try parser.nodes.expectEqualsSlice(&expected_nodes);
        // No tampering done with the file, i.e., no errors.
        try parser.tokenizer.file.expectEqualsSlice(&file_slice);
    }
}

test "parser implied blocks" {
    {
        var parser: Parser = .{};
        defer parser.deinit();
        errdefer {
            common.debugPrint("# file:\n", parser.tokenizer.file);
        }
        const file_slice = [_][]const u8{
            "Start4",
            "    Indented4",
        };
        try parser.tokenizer.file.appendSlice(&file_slice);

        try parser.complete(DoNothing{});

        try parser.nodes.expectEqualsSlice(&[_]Node{
            // [0]:
            Node{ .enclosed = .{ .open = .none, .tab = 0, .start = 1 } },
            Node{ .statement = .{ .node = 6, .next = 0 } },
            Node{ .atomic_token = 1 }, // Start4
            Node{ .enclosed = .{ .open = .none, .tab = 4, .start = 4 } },
            Node{ .statement = .{ .node = 5, .next = 0 } },
            // [5]:
            Node{ .atomic_token = 3 }, // Indented4
            Node{ .binary = .{ .operator = .op_indent, .left = 2, .right = 3 } },
            .end,
        });
        // No tampering done with the file, i.e., no errors.
        try parser.tokenizer.file.expectEqualsSlice(&file_slice);
    }
    {
        var parser: Parser = .{};
        defer parser.deinit();
        errdefer {
            common.debugPrint("# file:\n", parser.tokenizer.file);
        }
        const file_slice = [_][]const u8{
            "Start3",
            "    +Indented3",
        };
        try parser.tokenizer.file.appendSlice(&file_slice);

        try parser.complete(DoNothing{});

        try parser.nodes.expectEqualsSlice(&[_]Node{
            // [0]:
            Node{ .enclosed = .{ .open = .none, .tab = 0, .start = 1 } },
            Node{ .statement = .{ .node = 7, .next = 0 } },
            Node{ .atomic_token = 1 }, // Start3
            Node{ .enclosed = .{ .open = .none, .tab = 4, .start = 4 } },
            Node{ .statement = .{ .node = 5, .next = 0 } },
            // [5]:
            Node{ .prefix = .{ .operator = .op_plus, .node = 6 } },
            Node{ .atomic_token = 5 }, // Indented3
            Node{ .binary = .{ .operator = .op_indent, .left = 2, .right = 3 } },
            .end,
        });
        // No tampering done with the file, i.e., no errors.
        try parser.tokenizer.file.expectEqualsSlice(&file_slice);
    }
    {
        var parser: Parser = .{};
        defer parser.deinit();
        errdefer {
            common.debugPrint("# file:\n", parser.tokenizer.file);
        }
        const file_slice = [_][]const u8{
            "Start5:",
            "    Indented51",
            "    Indented52",
        };
        try parser.tokenizer.file.appendSlice(&file_slice);

        try parser.complete(DoNothing{});

        try parser.nodes.expectEqualsSlice(&[_]Node{
            // [0]:
            Node{ .enclosed = .{ .open = .none, .tab = 0, .start = 1 } },
            Node{ .statement = .{ .node = 8, .next = 0 } },
            Node{ .atomic_token = 1 }, // Start5
            Node{ .enclosed = .{ .open = .none, .tab = 4, .start = 4 } },
            Node{ .statement = .{ .node = 5, .next = 6 } },
            // [5]:
            Node{ .atomic_token = 5 }, // Indented51
            Node{ .statement = .{ .node = 7, .next = 0 } },
            Node{ .atomic_token = 7 }, // Indented52
            Node{ .binary = .{ .operator = .op_declare_readonly, .left = 2, .right = 3 } },
            .end,
        });
        // No tampering done with the file, i.e., no errors.
        try parser.tokenizer.file.expectEqualsSlice(&file_slice);
    }
}

test "parser line continuations via indent-to-operator then indent-to-atom" {
    var parser: Parser = .{};
    defer parser.deinit();
    errdefer {
        common.debugPrint("# file:\n", parser.tokenizer.file);
    }
    const file_slice = [_][]const u8{
        "Random_id3",
        "    +   Other_id4",
        "    -   Other_id5",
    };
    try parser.tokenizer.file.appendSlice(&file_slice);

    try parser.complete(DoNothing{});

    try parser.nodes.expectEqualsSlice(&[_]Node{
        // [0]:
        Node{ .enclosed = .{ .open = .none, .tab = 0, .start = 1 } },
        Node{ .statement = .{ .node = 6, .next = 0 } },
        Node{ .atomic_token = 1 }, // Random_id3
        Node{ .atomic_token = 5 }, // Other_id4
        Node{ .binary = .{ .operator = .op_plus, .left = 2, .right = 3 } },
        // [5]:
        Node{ .atomic_token = 9 }, // Other_id5
        Node{ .binary = .{ .operator = .op_minus, .left = 4, .right = 5 } },
        .end,
    });
    // No tampering done with the file, i.e., no errors.
    try parser.tokenizer.file.expectEqualsSlice(&file_slice);
}

test "indent errors" {
    {
        var parser: Parser = .{};
        defer parser.deinit();
        errdefer {
            common.debugPrint("# file:\n", parser.tokenizer.file);
        }
        try parser.tokenizer.file.appendSlice(&[_][]const u8{
            "H3",
            "   Only3spaces",
        });

        try std.testing.expectError(Parser.Error.syntax, parser.complete(DoNothing{}));

        // No tampering done with the file, i.e., no errors.
        try parser.tokenizer.file.expectEqualsSlice(&[_][]const u8{
            "H3",
            "   Only3spaces",
            "#@!~ indents should be 4-spaces wide",
        });
    }
    {
        var parser: Parser = .{};
        defer parser.deinit();
        errdefer {
            common.debugPrint("# file:\n", parser.tokenizer.file);
        }
        try parser.tokenizer.file.appendSlice(&[_][]const u8{
            "H5",
            "     Only5spaces",
        });

        try std.testing.expectError(Parser.Error.syntax, parser.complete(DoNothing{}));

        // No tampering done with the file, i.e., no errors.
        try parser.tokenizer.file.expectEqualsSlice(&[_][]const u8{
            "H5",
            "     Only5spaces",
            "#@!~~~ indents should be 4-spaces wide",
        });
    }
}

test "mixed commas and newlines with block" {
    const expected_nodes = [_]Node{
        // [0]:
        Node{ .enclosed = .{ .open = .none, .tab = 0, .start = 1 } },
        Node{ .statement = .{ .node = 18, .next = 0 } },
        Node{ .callable_token = 1 }, // goober
        Node{ .enclosed = .{ .open = .brace, .tab = 0, .start = 4 } }, // {...}
        Node{ .statement = .{ .node = 5, .next = 0 } }, // first statement in {...}
        // [5]:
        Node{ .enclosed = .{ .open = .none, .tab = 4, .start = 6 } }, // indent in {...}
        Node{ .statement = .{ .node = 7, .next = 8 } }, // first statement in indent
        Node{ .atomic_token = 5 }, // 405
        Node{ .statement = .{ .node = 9, .next = 10 } }, // second statement
        Node{ .atomic_token = 9 }, // 406
        // [10]:
        Node{ .statement = .{ .node = 11, .next = 12 } }, // third statement
        Node{ .atomic_token = 11 }, // 407
        Node{ .statement = .{ .node = 13, .next = 14 } }, // etc.
        Node{ .atomic_token = 13 }, // 408
        Node{ .statement = .{ .node = 15, .next = 16 } },
        // [15]:
        Node{ .atomic_token = 17 }, // 409
        Node{ .statement = .{ .node = 17, .next = 0 } },
        Node{ .atomic_token = 21 }, // 510
        Node{ .binary = .{ .operator = .op_indent, .left = 2, .right = 3 } }, // goober {}
        .end,
    };
    {
        // Explicit brace, one-true-brace style
        var parser: Parser = .{};
        defer parser.deinit();
        errdefer {
            common.debugPrint("# file:\n", parser.tokenizer.file);
        }
        const file_slice = [_][]const u8{
            "goober {",
            "    405, 406",
            "    407",
            "    408, 409,",
            "    510,",
            "}",
        };
        try parser.tokenizer.file.appendSlice(&file_slice);

        try parser.complete(DoNothing{});

        try parser.nodes.expectEqualsSlice(&expected_nodes);
        // No tampering done with the file, i.e., no errors.
        try parser.tokenizer.file.expectEqualsSlice(&file_slice);
    }
    {
        // Explicit brace, Horstmann style
        var parser: Parser = .{};
        defer parser.deinit();
        errdefer {
            common.debugPrint("# file:\n", parser.tokenizer.file);
        }
        const file_slice = [_][]const u8{
            "goober",
            "{   405, 406",
            "    407",
            "    408, 409,",
            "    510,",
            "}",
        };
        try parser.tokenizer.file.appendSlice(&file_slice);

        try parser.complete(DoNothing{});

        try parser.nodes.expectEqualsSlice(&expected_nodes);
        // No tampering done with the file, i.e., no errors.
        try parser.tokenizer.file.expectEqualsSlice(&file_slice);
    }
    {
        // implicit brace
        var parser: Parser = .{};
        defer parser.deinit();
        errdefer {
            common.debugPrint("# file:\n", parser.tokenizer.file);
        }
        const file_slice = [_][]const u8{
            "goober",
            "    405, 406",
            "    407",
            "    408, 409,",
            "    510,",
            "",
        };
        try parser.tokenizer.file.appendSlice(&file_slice);

        try parser.complete(DoNothing{});

        // TODO: do we want to make this exactly like the explicit braces above?
        try parser.nodes.expectEqualsSlice(&[_]Node{
            // [0]:
            Node{ .enclosed = .{ .open = .none, .tab = 0, .start = 1 } },
            Node{ .statement = .{ .node = 16, .next = 0 } },
            Node{ .callable_token = 1 }, // goober
            Node{ .enclosed = .{ .open = .none, .tab = 4, .start = 4 } },
            Node{ .statement = .{ .node = 5, .next = 6 } }, // first statement
            // [5]:
            Node{ .atomic_token = 3 }, // 405
            Node{ .statement = .{ .node = 7, .next = 8 } }, // second
            Node{ .atomic_token = 7 }, // 406
            Node{ .statement = .{ .node = 9, .next = 10 } }, // third
            Node{ .atomic_token = 9 }, // 407
            // [10]:
            Node{ .statement = .{ .node = 11, .next = 12 } },
            Node{ .atomic_token = 11 }, // 408
            Node{ .statement = .{ .node = 13, .next = 14 } },
            Node{ .atomic_token = 15 }, // 409
            Node{ .statement = .{ .node = 15, .next = 0 } },
            // [15]:
            Node{ .atomic_token = 19 }, // 510
            Node{ .binary = .{ .operator = .op_indent, .left = 2, .right = 3 } },
            .end,
        });
        // No tampering done with the file, i.e., no errors.
        try parser.tokenizer.file.expectEqualsSlice(&file_slice);
    }
}
