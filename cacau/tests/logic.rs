use runner::Runner;

pub fn run(input: &str) -> String {
    let mut output = Vec::<u8>::new();
    let parsed = parser_lalrpop::parse(input);
    dbg!(&parsed);
    Runner::run(&parsed, &mut output);
    String::from_utf8(output).expect("Program output is not valid utf-8")
}

#[test]
fn comparison_and_logic_precedence() {
    assert_eq!(
        run(r###"
            print(0 == 0 and 1 == 1);
        "###),
        "true"
    );
}
