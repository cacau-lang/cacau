use runner::Runner;

pub fn run(input: &str) -> String {
    let mut output = Vec::<u8>::new();
    Runner::run(&parser_lalrpop::parse(input), &mut output);
    String::from_utf8(output).expect("Program output is not valid utf-8")
}

#[test]
fn hello_world() {
    assert_eq!(
        run(r###"
            print("Hello, World!");
        "###),
        "Hello, World!"
    );
}
