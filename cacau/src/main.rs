use std::env;

use parser::{parse, rule_parser_from_str};

fn main() {
    let mut args = env::args_os().skip(1);

    let rule_name = args.next().expect("no rule supplied");

    let term = args.next().expect("no term supplied");

    let (rule, from_pest) = rule_parser_from_str(rule_name.to_str().unwrap());

    let mut parsed = parse(rule, term.to_str().unwrap()).expect("failed to parse");

    println!("parsed: {}", parsed);

    println!("parsed (verbose): {:#?}", parsed);

    if let Some(build_ast) = from_pest {
        let ast = build_ast(&mut parsed);

        println!("ast: {:#?}", ast);
    } else {
        println!("(no ast)");
    }
}
