# Reads stuff and turns them into parser trees and ASTs (WIP)

## Usage

```console
cargo run <rule> <term>
```

Where `rule` is one of `Expression`, `Program`, `Literal`, `FloatLiteral`, `IntegerLiteral`, `CharLiteral`, `BooleanLiteral`, `StringLiteral` or `Identifier`, and `term` is the expression you want to parse.

But `Expression`, `Program` and `Identifier` doesn't have AST nodes yet (to build an AST node you need to build all children). For them, it shows only the parser tree.

## Examples

```console
$ cargo run FloatLiteral 1.55
parsed: [FloatLiteral(0, 4)]
parsed (verbose): [
    Pair {
        rule: FloatLiteral,
        span: Span {
            str: "1.55",
            start: 0,
            end: 4,
        },
        inner: [],
    },
]
ast: FloatLiteral {
    value: 1.55,
    span: Span {
        str: "1.55",
        start: 0,
        end: 4,
    },
}

```

```console
$ cargo run IntegerLiteral 1                                                                                                      101 ↵
parsed: [IntegerLiteral(0, 1)]
parsed (verbose): [
    Pair {
        rule: IntegerLiteral,
        span: Span {
            str: "1",
            start: 0,
            end: 1,
        },
        inner: [],
    },
]
ast: IntegerLiteral {
    value: 1,
    span: Span {
        str: "1",
        start: 0,
        end: 1,
    },
}

```

```console
$ cargo run IntegerLiteral -1 # bug: can't parse negative integers yet. OOps.
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/cacau IntegerLiteral -1`
thread 'main' panicked at 'failed to parse: Error { variant: ParsingError { positives: [IntegerLiteral], negatives: [] }, location: Pos(0), line_col: Pos((1, 1)), path: None, line: "-1", continued_line: None }', cacau/src/main.rs:14:58
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

```console
$ cargo run Expression 'x + 1'
    Blocking waiting for file lock on build directory
   Compiling cacau v0.1.0 (/proj/cacau/cacau)
    Finished dev [unoptimized + debuginfo] target(s) in 0.44s
     Running `target/debug/cacau Expression 'x + 1'`
parsed: [Expression(0, 1, [Identifier(0, 1)])]
parsed (verbose): [
    Pair {
        rule: Expression,
        span: Span {
            str: "x",
            start: 0,
            end: 1,
        },
        inner: [
            Pair {
                rule: Identifier,
                span: Span {
                    str: "x",
                    start: 0,
                    end: 1,
                },
                inner: [],
            },
        ],
    },
]
(no ast)
```

```console
$ cargo run Program "$(< tests/00_hello_world.cau)"
parsed: [Statement(38, 59, [ExpressionStatement(38, 59, [Expression(38, 58, [FunctionCall(38, 58, [Identifier(38, 43), FunctionCallArgumentList(44, 57, [Expression(44, 57, [Literal(44, 57, [String(44, 57)])])])])])])]), Statement(60, 71, [ExpressionStatement(60, 71, [Expression(60, 70, [FunctionCall(60, 70, [Identifier(60, 65), FunctionCallArgumentList(66, 69, [Expression(66, 69, [Literal(66, 69, [Char(66, 69)])])])])])])]), Statement(81, 130, [ExpressionStatement(81, 130, [Expression(81, 129, [FunctionCall(81, 129, [Identifier(81, 86), FunctionCallArgumentList(87, 128, [Expression(87, 128, [Literal(87, 128, [String(87, 128)])])])])])])]), Statement(131, 143, [ExpressionStatement(131, 143, [Expression(131, 142, [FunctionCall(131, 142, [Identifier(131, 136), FunctionCallArgumentList(137, 141, [Expression(137, 141, [Literal(137, 141, [Integer(137, 141)])])])])])])]), Statement(144, 155, [ExpressionStatement(144, 155, [Expression(144, 154, [FunctionCall(144, 154, [Identifier(144, 149), FunctionCallArgumentList(150, 153, [Expression(150, 153, [Literal(150, 153, [Char(150, 153)])])])])])])]), EOI(155, 155)]
... some really verbose parser tree
.....
(no ast)
```
