use std::{env, fs};

use parser::{Rule, ParserTrait, ExpressionParser};

fn main() {
    let file = env::args_os().skip(1).next().expect("no file supplied");

    let contents = fs::read_to_string(file).unwrap();

    let parsed = ExpressionParser::parse(Rule::program, &contents).expect("failed to parse");

    println!("parsed: {}", parsed);
}