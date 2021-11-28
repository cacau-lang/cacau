use runner::ast::{CacauProgram, Expression, FunctionCall, HighLevelItem};

#[test]
fn test_hello() {
    // hello world AST
    let program = CacauProgram {
        items: vec![HighLevelItem::Expr(Expression::FunctionCall(
            FunctionCall {
                name: "println",
                params: vec![Expression::StringLiteral("Hello, World!")],
            },
        ))],
    };

    // run
    let mut stdout = Vec::new();
    runner::Runner::run(&program, &mut stdout);

    // check output
    assert_eq!(String::from_utf8(stdout).unwrap(), "Hello, World!\n")
}
