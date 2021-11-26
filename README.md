# Reads stuff and turns them into parser trees and ASTs (WIP)

## Usage

```console
cargo run <rule> <term>
```

Where `rule` is one of `Expression`, `Program`, `Literal`, `FloatLiteral`, `IntegerLiteral`, `CharLiteral`, `BooleanLiteral`, `StringLiteral` or `Identifier`, and `term` is the expression you want to parse.

But `Expression`, `Program` and `Identifier` doesn't have AST nodes yet (to build an AST node you need to build all children). For them, it shows only the parser tree.

And actually there's also `ArithmeticOperation` wich is like `BinaryOperations` but was modified to only have arithmetic operations inside it. (Just to get started at removing left recursion)

Also, `ArithmeticOperation` generates a crazy parser tree and doesn't have any operator climbing yet.

## Examples

```console
$ cargo run -q FloatLiteral 1.55
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
}

```

```console
$ cargo run -q IntegerLiteral 1                                                                                                      101 ↵
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
}

```

```console
$ cargo run -q IntegerLiteral -1 # bug: can't parse negative integers yet. OOps.
thread 'main' panicked at 'failed to parse: Error { variant: ParsingError { positives: [IntegerLiteral], negatives: [] }, location: Pos(0), line_col: Pos((1, 1)), path: None, line: "-1", continued_line: None }', cacau/src/main.rs:14:58
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

```console
$ cargo run -q Expression 'x + 1' # this parses only the variable, because it doesn't have binary operators yets
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
$ cargo run -q Program "$(< tests/00_hello_world.cau)"
parsed: [Statement(38, 59, [ExpressionStatement(38, 59, [Expression(38, 58, [FunctionCall(38, 58, [Identifier(38, 43), FunctionCallArgumentList(44, 57, [Expression(44, 57, [Literal(44, 57, [String(44, 57)])])])])])])]), Statement(60, 71, [ExpressionStatement(60, 71, [Expression(60, 70, [FunctionCall(60, 70, [Identifier(60, 65), FunctionCallArgumentList(66, 69, [Expression(66, 69, [Literal(66, 69, [Char(66, 69)])])])])])])]), Statement(81, 130, [ExpressionStatement(81, 130, [Expression(81, 129, [FunctionCall(81, 129, [Identifier(81, 86), FunctionCallArgumentList(87, 128, [Expression(87, 128, [Literal(87, 128, [String(87, 128)])])])])])])]), Statement(131, 143, [ExpressionStatement(131, 143, [Expression(131, 142, [FunctionCall(131, 142, [Identifier(131, 136), FunctionCallArgumentList(137, 141, [Expression(137, 141, [Literal(137, 141, [Integer(137, 141)])])])])])])]), Statement(144, 155, [ExpressionStatement(144, 155, [Expression(144, 154, [FunctionCall(144, 154, [Identifier(144, 149), FunctionCallArgumentList(150, 153, [Expression(150, 153, [Literal(150, 153, [Char(150, 153)])])])])])])]), EOI(155, 155)]
... some really verbose parser tree
.....
(no ast)
```

````console
$ cargo run -q ArithmeticOperation '(x * y) + x'
parsed: [ArithmeticOperation(0, 11, [Term(0, 7, [ArithmeticOperation(1, 6, [Term(1, 2, [Identifier(1, 2)]), ArithmeticPiece(3, 6, [ArithmeticOperator(3, 4, [Multiply(3, 4)]), ArithmeticOperation(5, 6, [Term(5, 6, [Identifier(5, 6)])])])])]), ArithmeticPiece(8, 11, [ArithmeticOperator(8, 9, [Add(8, 9)]), ArithmeticOperation(10, 11, [Term(10, 11, [Identifier(10, 11)])])])])]
... another verbose parser tree
ast: ArithmeticOperation {
    first: ArithmeticOperation(
        ArithmeticOperation {
            first: Identifier(
                Identifier {
                    value: "x",
                },
            ),
            rest: "* y",
        },
    ),
    rest: "+ x",
}
```
````
