use std::env;

use parser::parse;

fn main() {
    let mut args = env::args_os().skip(1);

    let term = args.next().expect("no term supplied");

    let (parser_tree, ast) = parse(term.to_str().unwrap()).expect("failed to parse");

    println!("parser tree (small): {}", parser_tree);

    println!("parser tree (verbose): {:#?}", parser_tree);

    println!("ast: {:#?}", ast);
}
