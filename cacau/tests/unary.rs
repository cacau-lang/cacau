use runner::Runner;

pub fn run(input: &str) -> String {
    let mut output = Vec::<u8>::new();
    Runner::run(&parser_lalrpop::parse(input), &mut output);
    String::from_utf8(output).expect("Program output is not valid utf-8")
}

#[test]
fn simple_unary() {
    assert_eq!(
        run(r###"
            print(-1);
        "###),
        "-1"
    );
}

#[test]
fn simple_not() {
    assert_eq!(
        run(r###"
            print(not true);
        "###),
        "false"
    );
}

#[test]
fn unary_inside_expr() {
    assert_eq!(
        run(r###"
            print(5 - -2);
        "###),
        "7"
    );
}

#[test]
fn unaries_inside_expr() {
    assert_eq!(
        run(r###"
            print(- 5 - -2);
        "###),
        "-3"
    );
}

#[test]
fn unary_the_world() {
    assert_eq!(
        run(r###"
            print(- - 5 - -2);
        "###),
        "7"
    );
}