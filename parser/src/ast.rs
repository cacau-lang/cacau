use pest::Span;

use crate::Rule;

fn span_into_str(span: Span) -> &str {
    span.as_str()
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::integer))]
pub struct IntegerLiteral<'pest> {
    #[pest_ast(outer(with(span_into_str), with(str::parse), with(Result::unwrap)))]
    pub value: i64,

    #[pest_ast(outer())]
    pub span: Span<'pest>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::float))]
pub struct FloatLiteral<'pest> {
    #[pest_ast(outer(with(span_into_str), with(str::parse), with(Result::unwrap)))]
    pub value: f64,

    #[pest_ast(outer())]
    pub span: Span<'pest>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::num))]
pub enum NumLiteral<'pest> {
    Float(FloatLiteral<'pest>),
    Integer(IntegerLiteral<'pest>),
}
