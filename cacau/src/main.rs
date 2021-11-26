use std::env;

use parser::{parse, rule_from_str};

fn main() {
    let mut args = env::args_os().skip(1);

    let rule_name = args.next().expect("no rule supplied");

    let term = args.next().expect("no term supplied");

    let rule = rule_from_str(rule_name.to_str().unwrap());

    let parsed = parse(rule, term.to_str().unwrap()).expect("failed to parse");

    println!("parsed: {}", parsed);

    println!("parsed (verbose): {:#?}", parsed);
}
