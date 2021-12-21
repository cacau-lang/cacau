use runner::Runner;

pub fn run(input: &str) -> String {
    let mut output = Vec::<u8>::new();
    let parsed = parser_lalrpop::parse(input);
    dbg!(&parsed);
    Runner::run(&parsed, &mut output);
    String::from_utf8(output).expect("Program output is not valid utf-8")
}

#[test]
fn reassignment() {
    assert_eq!(
        run(r###"
            let a = 10;
            a = a + 1;
            print(a);
        "###),
        "11"
    );
}
