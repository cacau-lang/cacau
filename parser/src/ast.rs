use lazy_static::lazy_static;
use pest::prec_climber::PrecClimber;
use pest_consume::{match_nodes, Error};

use crate::{CacauParser, Rule};

type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use pest::prec_climber::{
            Assoc::{Left, Right},
            Operator,
        };
        use Rule::{Add, Divide, Modulo, Multiply, Power, Subtract};

        PrecClimber::new(vec![
            Operator::new(Add, Left) | Operator::new(Subtract, Left),
            Operator::new(Multiply, Left)
                | Operator::new(Divide, Left)
                | Operator::new(Modulo, Left),
            Operator::new(Power, Right),
        ])
    };
}

// damn rust keeps saying CacauParser::ArithmeticOperation is private
pub fn okay(o: Node) -> Result<Term> {
    CacauParser::ArithmeticOperation(o)
}

#[pest_consume::parser]
#[allow(non_snake_case)]
impl CacauParser {
    pub fn EOI(_input: Node) -> Result<()> {
        Ok(())
    }

    pub fn FloatLiteral(input: Node) -> Result<FloatLiteral> {
        let x: f64 = input.as_str().trim().parse().map_err(|e| input.error(e))?;

        Ok(FloatLiteral(x))
    }

    pub fn IntegerLiteral(input: Node) -> Result<IntegerLiteral> {
        let x: i64 = input.as_str().trim().parse().map_err(|e| input.error(e))?;

        Ok(IntegerLiteral(x))
    }

    pub fn CharLiteral(input: Node) -> Result<CharLiteral> {
        let x: char = input.as_str().trim().parse().map_err(|e| input.error(e))?;

        Ok(CharLiteral(x))
    }

    pub fn BooleanLiteral(input: Node) -> Result<BooleanLiteral> {
        let x: bool = input.as_str().trim().parse().map_err(|e| input.error(e))?;

        Ok(BooleanLiteral(x))
    }

    pub fn StringLiteral(input: Node) -> Result<StringLiteral> {
        Ok(StringLiteral(input.as_str().into()))
    }

    pub fn Literal(input: Node) -> Result<Literal> {
        let literal = match_nodes!(input.into_children();
            [FloatLiteral(n)] => Literal::FloatLiteral(n),
            [IntegerLiteral(n)] => Literal::IntegerLiteral(n),
            [CharLiteral(n)] => Literal::CharLiteral(n),
            [BooleanLiteral(n)] => Literal::BooleanLiteral(n),
            [StringLiteral(n)] => Literal::StringLiteral(n),
        );
        Ok(literal)
    }

    pub fn Identifier(input: Node) -> Result<Identifier> {
        let x: String = input.as_str().to_owned();

        Ok(Identifier(x))
    }

    pub fn Term(input: Node) -> Result<Term> {
        let term = match_nodes!(input.into_children();
            [Literal(n)] => Term::Literal(n),
            [Identifier(n)] => Term::Identifier(n),
            [ArithmeticOperation(n)] => n,
        );
        Ok(term)
    }

    #[prec_climb(Term, PREC_CLIMBER)]
    pub fn ArithmeticOperation(left: Term, op: Node, right: Term) -> Result<Term> {
        let op = match op.as_rule() {
            Rule::Add => ArithmeticOperator::Add,
            Rule::Subtract => ArithmeticOperator::Subtract,
            Rule::Multiply => ArithmeticOperator::Multiply,
            Rule::Divide => ArithmeticOperator::Divide,
            Rule::Power => ArithmeticOperator::Power,
            Rule::Modulo => ArithmeticOperator::Modulo,
            rule => return Err(op.error(format!("Rule {:?} isn't an operator", rule))),
        };
        let op = Box::new(ArithmeticOperation { left, op, right });
        Ok(Term::ArithmeticOperation(op))
    }

    /*
    fn ArithmeticOperation(op: Node) -> Result<ArithmeticOperation> {
        let mut op = op;
        while <Self as ::pest_consume::Parser>::allows_shortcut(op.as_rule()) {
            if let ::std::result::Result::Ok(child) = op.children().single() {
                if child.as_aliased_rule::<Self>()
                    == <Self as ::pest_consume::Parser>::rule_alias(Rule::ArithmeticOperation)
                {
                    op = child;
                    continue;
                }
            }
            break;
        }
        match op.as_rule() {
            Rule::ArithmeticOperation => {
                #[allow(non_snake_case)]
                fn ArithmeticOperation(
                    left: Term,
                    op: Node,
                    right: Term,
                ) -> Result<ArithmeticOperation> {
                    match op.as_rule() {
                        Rule::Add => Ok(ArithmeticOperation {
                            left,
                            op: ArithmeticOperator::Add,
                            right,
                        }),
                        r => panic!(),
                    }
                }
                op.into_children()
                    .prec_climb(&*PREC_CLIMBER, Self::Term, ArithmeticOperation)
            }
            r => panic!(),
        }
    }*/
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum Literal {
    FloatLiteral(FloatLiteral),
    IntegerLiteral(IntegerLiteral),
    CharLiteral(CharLiteral),
    BooleanLiteral(BooleanLiteral),
    StringLiteral(StringLiteral),
}

#[derive(Debug)]
pub struct FloatLiteral(f64);

#[derive(Debug)]
pub struct IntegerLiteral(i64);

#[derive(Debug)]
pub struct CharLiteral(char);

#[derive(Debug)]
pub struct BooleanLiteral(bool);
// TODO: parse the string (removing quotes, expanding \n, \t, etc)
#[derive(Debug)]
pub struct StringLiteral(String);

#[derive(Debug)]
pub struct Identifier(String);

#[derive(Debug)]
pub enum Term {
    Literal(Literal),
    Identifier(Identifier),
    ArithmeticOperation(Box<ArithmeticOperation>),
}

#[derive(Debug)]
pub struct ArithmeticOperation {
    pub left: Term,
    pub op: ArithmeticOperator,
    pub right: Term,
}

#[derive(Debug)]
pub enum ArithmeticOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Modulo,
}
