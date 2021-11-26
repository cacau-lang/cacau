use pest::Span;
use pest_ast::FromPest;

use crate::Rule;

fn span_as_str(span: Span) -> &str {
    span.as_str()
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Literal))]
pub enum Literal {
    FloatLiteral(FloatLiteral),
    IntegerLiteral(IntegerLiteral),
    CharLiteral(CharLiteral),
    BooleanLiteral(BooleanLiteral),
    StringLiteral(StringLiteral),
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::FloatLiteral))]
pub struct FloatLiteral {
    #[pest_ast(outer(with(span_as_str), with(str::parse), with(Result::unwrap)))]
    pub value: f64,
    //#[pest_ast(outer())]
    //pub span: Span<'pest>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::IntegerLiteral))]
pub struct IntegerLiteral {
    #[pest_ast(outer(with(span_as_str), with(str::parse), with(Result::unwrap)))]
    pub value: i64,
    //#[pest_ast(outer())]
    //pub span: Span<'pest>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::StringLiteral))]
pub struct StringLiteral {
    // TODO: parse the string (removing quotes, expanding \n, \t, etc)
    #[pest_ast(outer(with(span_as_str), with(String::from)))]
    pub value: String,
    //#[pest_ast(outer())]
    //pub span: Span<'pest>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::CharLiteral))]
pub struct CharLiteral {
    #[pest_ast(outer(with(span_as_str), with(str::parse), with(Result::unwrap)))]
    pub value: char,
    //#[pest_ast(outer())]
    //pub span: Span<'pest>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::BooleanLiteral))]
pub struct BooleanLiteral {
    #[pest_ast(outer(with(span_as_str), with(str::parse), with(Result::unwrap)))]
    pub value: bool,
    //#[pest_ast(outer())]
    //pub span: Span<'pest>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Identifier))]
pub struct Identifier {
    #[pest_ast(outer(with(span_as_str), with(String::from)))]
    pub value: String,
    //#[pest_ast(outer())]
    //pub span: Span<'pest>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Term))]
pub enum Term {
    Literal(Literal),
    Identifier(Identifier),
    ArithmeticOperation(Box<ArithmeticOperation>),
}

/*lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use Assoc::*;
        use Rule::*;

        PrecClimber::new(vec![
            Operator::new(Add, Left) | Operator::new(Subtract, Left),
            Operator::new(Multiply, Left)
                | Operator::new(Divide, Left)
                | Operator::new(Modulo, Left),
            Operator::new(Power, Right),
        ])
    };
}*/

// this was crazy. never was even close to work

/*#[derive(Debug)]
struct ArithmeticOperation {}

impl<'pest> ::from_pest::FromPest<'pest> for ArithmeticOperation {
    type Rule = Rule;

    type FatalError = ::from_pest::Void;

    fn from_pest(
        pest: &mut ::from_pest::pest::iterators::Pairs<'pest, Rule>,
    ) -> ::std::result::Result<Self, ::from_pest::ConversionError<::from_pest::Void>> {
        let mut expr = pest.clone();

        PREC_CLIMBER.climb(expr, |pair| match pair.as_rule() {
            Rule::Literal => Term::Literal(Literal),
        });

        let this = ArithmeticOperation {};

        Ok(this)
    }
}*/

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::ArithmeticOperation))]
pub struct ArithmeticOperation {
    pub first: Term,

    #[pest_ast(inner(with(span_as_str), with(String::from)))]
    pub rest: String,
    //#[pest_ast(outer())]
    //pub span: Span<'pest>,
}
#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::ArithmeticPiece))]
pub struct ArithmeticPiece {
    pub operator: ArithmeticOperator,

    pub operation: ArithmeticOperation,
    //#[pest_ast(outer())]
    //pub span: Span<'pest>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::ArithmeticOperator))]
pub enum ArithmeticOperator {
    Add(Add),
    Subtract(Subtract),
    Multiply(Multiply),
    Divide(Divide),
    Power(Power),
    Modulo(Modulo),
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Add))]
pub struct Add {
    //#[pest_ast(outer())]
//pub span: Span<'pest>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Subtract))]
pub struct Subtract {
    //#[pest_ast(outer())]
//pub span: Span<'pest>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Multiply))]
pub struct Multiply {
    //#[pest_ast(outer())]
//pub span: Span<'pest>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Divide))]
pub struct Divide {
    //#[pest_ast(outer())]
//pub span: Span<'pest>,
}
#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Power))]
pub struct Power {
    //#[pest_ast(outer())]
//pub span: Span<'pest>,
}
#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Modulo))]
pub struct Modulo {
    //#[pest_ast(outer())]
//pub span: Span<'pest>,
}
