use runner::Runner;

pub fn run(input: &str) -> String {
    let mut output = Vec::<u8>::new();
    Runner::run(&parser_lalrpop::parse(input), &mut output);
    String::from_utf8(output).expect("Program output is not valid utf-8")
}

#[test]
fn simple_decl_string() {
    assert_eq!(
        run(r###"
            let a = "Hello, World!";
            print(a);
        "###),
        "Hello, World!"
    );
}

#[test]
fn simple_decl_char() {
    assert_eq!(
        run(r###"
            let a = 'N';
            print(a);
        "###),
        "N"
    );
}

#[test]
fn simple_decl_int() {
    assert_eq!(
        run(r###"
            let a = 10;
            println(a);
        "###),
        "10\n"
    );
}

#[test]
fn simple_decl_with_type() {
    assert_eq!(
        run(r###"
            let a: int = 10;
            println(a);
        "###),
        "10\n"
    );
}