use runner::ast::{
    Assignment, CacauProgram, ComparisonOperation, Expression, FunctionCall, HighLevelItem,
};

#[test]
fn comparisons() {
    let program = CacauProgram {
        items: vec![
            HighLevelItem::Expr(Expression::FunctionCall(FunctionCall {
                name: "assert",
                params: vec![Expression::BooleanLiteral(true)],
            })),
            HighLevelItem::Expr(Expression::Assignment(Box::new(Assignment {
                name: "text",
                type_annotation: None,
                expression: Expression::StringLiteral("foo"),
            }))),
            HighLevelItem::Expr(Expression::FunctionCall(FunctionCall {
                name: "assert",
                params: vec![Expression::CompOperation(Box::new(ComparisonOperation {
                    left: Expression::Identifier("text"),
                    op: runner::ast::ComparisonOperator::Equals,
                    right: Expression::StringLiteral("foo"),
                }))],
            })),
        ],
    };

    // run
    let mut stdout = Vec::new();
    runner::Runner::run(&program, &mut stdout);
}
