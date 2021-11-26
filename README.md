# Reads stuff and turns them into parser trees and ASTs (WIP)

## Usage

```console
cargo run <term>
```

It runs only on the rule `ArithmeticOperation`

But doesn't work very well.. it can't read operators for some reason.

So for example it reads `1 + 1` as `1` (well the parser tree returns the 1 + 1 fine, but the AST has only the first element)

:(

Some example runs:

```console
$ cargo run -q '1'
parser tree (small): ArithmeticOperation(0, 1, [Term(0, 1, [Literal(0, 1, [IntegerLiteral(0, 1)])])])
parser tree (verbose): Node {
    pair: Pair {
        rule: ArithmeticOperation,
        span: Span {
            str: "1",
            start: 0,
            end: 1,
        },
        inner: [
            Pair {
                rule: Term,
                span: Span {
                    str: "1",
                    start: 0,
                    end: 1,
                },
                inner: [
                    Pair {
                        rule: Literal,
                        span: Span {
                            str: "1",
                            start: 0,
                            end: 1,
                        },
                        inner: [
                            Pair {
                                rule: IntegerLiteral,
                                span: Span {
                                    str: "1",
                                    start: 0,
                                    end: 1,
                                },
                                inner: [],
                            },
                        ],
                    },
                ],
            },
        ],
    },
    user_data: (),
}
ast: Literal(
    IntegerLiteral(
        IntegerLiteral(
            1,
        ),
    ),
)
```

```console
$ cargo run -q 'x + 5.5'
parser tree (small): ArithmeticOperation(0, 7, [Term(0, 1, [Identifier(0, 1)]), ArithmeticOperator(2, 3, [Add(2, 3)]), ArithmeticOperation(4, 7, [Term(4, 7, [Literal(4, 7, [FloatLiteral(4, 7)])])])])
parser tree (verbose): Node {
    pair: Pair {
        rule: ArithmeticOperation,
        span: Span {
            str: "x + 5.5",
            start: 0,
            end: 7,
        },
        inner: [
            Pair {
                rule: Term,
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
            Pair {
                rule: ArithmeticOperator,
                span: Span {
                    str: "+",
                    start: 2,
                    end: 3,
                },
                inner: [
                    Pair {
                        rule: Add,
                        span: Span {
                            str: "+",
                            start: 2,
                            end: 3,
                        },
                        inner: [],
                    },
                ],
            },
            Pair {
                rule: ArithmeticOperation,
                span: Span {
                    str: "5.5",
                    start: 4,
                    end: 7,
                },
                inner: [
                    Pair {
                        rule: Term,
                        span: Span {
                            str: "5.5",
                            start: 4,
                            end: 7,
                        },
                        inner: [
                            Pair {
                                rule: Literal,
                                span: Span {
                                    str: "5.5",
                                    start: 4,
                                    end: 7,
                                },
                                inner: [
                                    Pair {
                                        rule: FloatLiteral,
                                        span: Span {
                                            str: "5.5",
                                            start: 4,
                                            end: 7,
                                        },
                                        inner: [],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
        ],
    },
    user_data: (),
}
ast: Identifier(
    Identifier(
        "x",
    ),
)
```
