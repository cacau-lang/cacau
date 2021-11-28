use runner::ast::{
    Assignment, CacauProgram, ComparisonOperation, ComparisonOperator, Expression, FunctionCall,
    HighLevelItem,
};

#[test]
fn comparisons() {
    use runner::ast::ComparisonOperator::*;

    fn assert_cmp(
        var: &'static str,
        op: ComparisonOperator,
        value: Expression<'static>,
    ) -> HighLevelItem<'static> {
        HighLevelItem::Expr(Expression::FunctionCall(FunctionCall {
            name: "assert",
            params: vec![Expression::CompOperation(Box::new(ComparisonOperation {
                left: Expression::Identifier(var),
                op,
                right: value,
            }))],
        }))
    }

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
            assert_cmp("text", Equals, Expression::StringLiteral("foo")),
            assert_cmp("text", GreaterEquals, Expression::StringLiteral("foo")),
            assert_cmp("text", LessEquals, Expression::StringLiteral("foo")),
            assert_cmp("text", Greater, Expression::StringLiteral("aaa")),
            assert_cmp("text", GreaterEquals, Expression::StringLiteral("aaa")),
            assert_cmp("text", Less, Expression::StringLiteral("zzz")),
            assert_cmp("text", LessEquals, Expression::StringLiteral("zzz")),
            assert_cmp("text", NotEquals, Expression::StringLiteral("bar")),
        ],
    };

    // run
    let mut stdout = Vec::new();
    runner::Runner::run(&program, &mut stdout);
}
