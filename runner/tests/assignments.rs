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
