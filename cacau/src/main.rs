use std::{env, fs, fmt::Debug};

use from_pest::FromPest;

use parser::{ExpressionParser, ParserTrait, Rule};

use parser::ast::*;

fn main() {
    let mut args = env::args_os().skip(1);

    let rule_name = args.next().expect("no rule supplied");

    let mut file = args.next().unwrap_or("".into());

    if file == "-" || file == "" {
        file = "/dev/stdin".into()
    }

    let contents = fs::read_to_string(file).unwrap();

    let (rule, build_ast) = match rule_name.to_str().unwrap() {
        "num" => (Rule::num, Box::new(|p| Box::new(NumLiteral::from_pest(p).unwrap()) as Box<dyn Debug>) as Box<dyn FnOnce(_) -> _>),
        "float" => (Rule::float, Box::new(|p| Box::new(FloatLiteral::from_pest(p).unwrap()) as Box<dyn Debug>) as Box<dyn FnOnce(_) -> _>),
        "integer" => (Rule::integer, Box::new(|p| Box::new(IntegerLiteral::from_pest(p).unwrap()) as Box<dyn Debug>) as Box<dyn FnOnce(_) -> _>),
        "identifier" => (Rule::identifier, Box::new(|p| Box::new(Identifier::from_pest(p).unwrap()) as Box<dyn Debug>) as Box<dyn FnOnce(_) -> _>),
        name => panic!("unknown rule {}", name),
    };

    let mut parsed = ExpressionParser::parse(rule, &contents).expect("failed to parse");

    println!("parsed: {:#?}", parsed);

    let ast = build_ast(&mut parsed);

    println!("(whats left): {}", parsed);

    println!("ast: {:#?}", ast);
}
