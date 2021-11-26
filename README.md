# Reads stuff and turns them into parser trees (WIP)

## Usage

```console
cargo run <rule> <term>
```

Where `rule` is one of `Expression`, `Program`, `Literal`, `Float`, `Integer`, `Char`, `Boolean` or `String`, and `term` is the expression you want to parse.

## Examples

```console
$ cargo run Float 1.55
parsed: [Float(0, 4)]
parsed (verbose): [
    Pair {
        rule: Float,
        span: Span {
            str: "1.55",
            start: 0,
            end: 4,
        },
        inner: [],
    },
]
```

```console
$ cargo run Integer 1
parsed: [Integer(0, 1)]
parsed (verbose): [
    Pair {
        rule: Integer,
        span: Span {
            str: "1",
            start: 0,
            end: 1,
        },
        inner: [],
    },
]
```

```console
$ cargo run Expression 'x + 1'
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
```

```console
$ cargo run Program "$(< tests/00_hello_world.cau)"
parsed: [Statement(38, 59, [ExpressionStatement(38, 59, [Expression(38, 58, [FunctionCall(38, 58, [Identifier(38, 43), FunctionCallArgumentList(44, 57, [Expression(44, 57, [Literal(44, 57, [String(44, 57)])])])])])])]), Statement(60, 71, [ExpressionStatement(60, 71, [Expression(60, 70, [FunctionCall(60, 70, [Identifier(60, 65), FunctionCallArgumentList(66, 69, [Expression(66, 69, [Literal(66, 69, [Char(66, 69)])])])])])])]), Statement(81, 130, [ExpressionStatement(81, 130, [Expression(81, 129, [FunctionCall(81, 129, [Identifier(81, 86), FunctionCallArgumentList(87, 128, [Expression(87, 128, [Literal(87, 128, [String(87, 128)])])])])])])]), Statement(131, 143, [ExpressionStatement(131, 143, [Expression(131, 142, [FunctionCall(131, 142, [Identifier(131, 136), FunctionCallArgumentList(137, 141, [Expression(137, 141, [Literal(137, 141, [Integer(137, 141)])])])])])])]), Statement(144, 155, [ExpressionStatement(144, 155, [Expression(144, 154, [FunctionCall(144, 154, [Identifier(144, 149), FunctionCallArgumentList(150, 153, [Expression(150, 153, [Literal(150, 153, [Char(150, 153)])])])])])])]), EOI(155, 155)]
(... some really verbose parser tree)
````
