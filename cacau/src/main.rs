use std::{env, fs};

use from_pest::FromPest;

use parser::{ExpressionParser, ParserTrait, Rule};

use parser::ast::NumLiteral;

fn main() {
    let file = env::args_os().skip(1).next().expect("no file supplied");

    let contents = fs::read_to_string(file).unwrap();

    let mut parsed = ExpressionParser::parse(Rule::num, &contents).expect("failed to parse");

    println!("parsed: {:#?}", parsed);

    let ast = NumLiteral::from_pest(&mut parsed).expect("mas devia ter dado certo");

    println!("(whats left): {}", parsed);

    println!("ast: {:#?}", ast);
}
