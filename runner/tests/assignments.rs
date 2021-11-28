use runner::ast::{Assignment, CacauProgram, Expression, FunctionCall, HighLevelItem};

#[test]
fn simple_assign() {
    let program = CacauProgram {
        items: vec![
            HighLevelItem::Expr(Expression::Assignment(Box::new(Assignment {
                name: "hello",
                expression: Expression::StringLiteral("Hello, World!"),
                type_annotation: None,
            }))),
            HighLevelItem::Expr(Expression::FunctionCall(FunctionCall {
                name: "println",
                params: vec![Expression::Identifier("hello")],
            })),
        ],
    };

    // run
    let mut stdout = Vec::new();
    runner::Runner::run(&program, &mut stdout);

    // check output
    assert_eq!(String::from_utf8(stdout).unwrap(), "Hello, World!\n")
}

#[test]
fn assigns_of_various_types() {
    let program = CacauProgram {
        items: vec![
            HighLevelItem::Expr(Expression::Assignment(Box::new(Assignment {
                name: "text",
                expression: Expression::StringLiteral("foo"),
                type_annotation: None,
            }))),
            HighLevelItem::Expr(Expression::Assignment(Box::new(Assignment {
                name: "integer",
                expression: Expression::IntegerLiteral(100),
                type_annotation: None,
            }))),
            HighLevelItem::Expr(Expression::Assignment(Box::new(Assignment {
                name: "decimal",
                expression: Expression::FloatLiteral(100.0),
                type_annotation: None,
            }))),
            HighLevelItem::Expr(Expression::Assignment(Box::new(Assignment {
                name: "character",
                expression: Expression::CharLiteral('1'),
                type_annotation: None,
            }))),
            HighLevelItem::Expr(Expression::Assignment(Box::new(Assignment {
                name: "truth",
                expression: Expression::BooleanLiteral(true),
                type_annotation: None,
            }))),
            HighLevelItem::Expr(Expression::FunctionCall(FunctionCall {
                name: "println",
                params: vec![Expression::Identifier("text")],
            })),
            HighLevelItem::Expr(Expression::FunctionCall(FunctionCall {
                name: "println",
                params: vec![Expression::Identifier("integer")],
            })),
            HighLevelItem::Expr(Expression::FunctionCall(FunctionCall {
                name: "println",
                params: vec![Expression::Identifier("decimal")],
            })),
            HighLevelItem::Expr(Expression::FunctionCall(FunctionCall {
                name: "println",
                params: vec![Expression::Identifier("character")],
            })),
            HighLevelItem::Expr(Expression::FunctionCall(FunctionCall {
                name: "println",
                params: vec![Expression::Identifier("truth")],
            })),
        ],
    };

    // run
    let mut stdout = Vec::new();
    runner::Runner::run(&program, &mut stdout);

    // check output
    assert_eq!(
        String::from_utf8(stdout).unwrap(),
        "foo
100
100.00000
1
true
"
    );
}
