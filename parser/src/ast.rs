use pest::Span;
use pest_ast::FromPest;

use crate::Rule;

fn span_as_str(span: Span) -> &str {
    span.as_str()
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Literal))]
pub enum Literal<'pest> {
    FloatLiteral(FloatLiteral<'pest>),
    IntegerLiteral(IntegerLiteral<'pest>),
    CharLiteral(CharLiteral<'pest>),
    BooleanLiteral(BooleanLiteral<'pest>),
    StringLiteral(StringLiteral<'pest>),
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::FloatLiteral))]
pub struct FloatLiteral<'pest> {
    #[pest_ast(outer(with(span_as_str), with(str::parse), with(Result::unwrap)))]
    pub value: f64,

    #[pest_ast(outer())]
    pub span: Span<'pest>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::IntegerLiteral))]
pub struct IntegerLiteral<'pest> {
    #[pest_ast(outer(with(span_as_str), with(str::parse), with(Result::unwrap)))]
    pub value: i64,

    #[pest_ast(outer())]
    pub span: Span<'pest>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::StringLiteral))]
pub struct StringLiteral<'pest> {
    // TODO: parse the string (removing quotes, expanding \n, \t, etc)
    #[pest_ast(outer(with(span_as_str)))]
    pub value: &'pest str,

    #[pest_ast(outer())]
    pub span: Span<'pest>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::CharLiteral))]
pub struct CharLiteral<'pest> {
    #[pest_ast(outer(with(span_as_str), with(str::parse), with(Result::unwrap)))]
    pub value: char,

    #[pest_ast(outer())]
    pub span: Span<'pest>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::BooleanLiteral))]
pub struct BooleanLiteral<'pest> {
    #[pest_ast(outer(with(span_as_str), with(str::parse), with(Result::unwrap)))]
    pub value: bool,

    #[pest_ast(outer())]
    pub span: Span<'pest>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Identifier))]
pub struct Identifier<'pest> {
    #[pest_ast(outer(with(span_as_str)))]
    pub value: &'pest str,

    #[pest_ast(outer())]
    pub span: Span<'pest>,
}
