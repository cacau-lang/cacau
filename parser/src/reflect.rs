use std::fmt::Debug;

use from_pest::FromPest;
use pest::iterators::Pairs;

use crate::Rule;

use crate::ast::*;

type BuildAstFn<'a> = Box<dyn FnOnce(&mut Pairs<'a, Rule>) -> Box<dyn Debug + 'a>>;

pub fn rule_parser_from_str<'a>(rule_name: &'a str) -> (Rule, Option<BuildAstFn<'a>>) {
    match rule_name {
        // TODO: a way to get this from the .pest file
        "Expression" => (Rule::Expression, None),
        "Program" => (Rule::Program, None),

        "Literal" => (
            Rule::Literal,
            Some(Box::new(|p| Box::new(Literal::from_pest(p).unwrap()))),
        ),
        "FloatLiteral" => (
            Rule::FloatLiteral,
            Some(Box::new(|p| Box::new(FloatLiteral::from_pest(p).unwrap()))),
        ),
        "IntegerLiteral" => (
            Rule::IntegerLiteral,
            Some(Box::new(|p| {
                Box::new(IntegerLiteral::from_pest(p).unwrap())
            })),
        ),
        "CharLiteral" => (
            Rule::CharLiteral,
            Some(Box::new(|p| Box::new(CharLiteral::from_pest(p).unwrap()))),
        ),
        "BooleanLiteral" => (
            Rule::BooleanLiteral,
            Some(Box::new(|p| {
                Box::new(BooleanLiteral::from_pest(p).unwrap())
            })),
        ),
        "StringLiteral" => (
            Rule::StringLiteral,
            Some(Box::new(|p| Box::new(StringLiteral::from_pest(p).unwrap()))),
        ),

        "IdentifierLiteral" => (
            Rule::Identifier,
            Some(Box::new(|p| Box::new(Identifier::from_pest(p).unwrap()))),
        ),

        name => panic!("unknown rule {}", name),
    }
}
