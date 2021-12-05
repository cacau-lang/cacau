#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(cacau);

pub fn parse(input: &str) -> ast::CacauProgram {
    crate::cacau::CacauProgramParser::new()
        .parse(input)
        .expect("Could not parse")
}

#[cfg(test)]
mod test {

    use ast::ArithOp::*;
    use ast::Lit::*;
    use ast::*;

    fn b<T>(t: T) -> Box<T> {
        Box::new(t)
    }
    fn parse_exp(s: &str) -> Expr {
        crate::cacau::ExprParser::new().parse(s).unwrap()
    }
    fn parse_program(s: &str) -> CacauProgram {
        crate::cacau::CacauProgramParser::new().parse(s).unwrap()
    }
    fn l(n: i64) -> Expr {
        Expr::Lit(Int(n))
    }
    fn a(left: Expr, op: ArithOp, right: Expr) -> Expr {
        Expr::Arith(b(ArithExpr { left, op, right }))
    }

    #[test]
    fn test() {
        assert_eq!(parse_exp("22"), l(22));
        assert_eq!(parse_exp("(22)"), Expr::Paren(b(l(22))));
        assert_eq!(parse_exp("((22))"), Expr::Paren(b(Expr::Paren(b(l(22))))));

        assert_eq!(parse_exp("22 + 21"), a(l(22), Add, l(21)));
        assert_eq!(
            parse_exp("22 + 21 + 20"),
            a(a(l(22), Add, l(21)), Add, l(20))
        );

        #[rustfmt::skip]
        assert_eq!(
            parse_exp("22 + 21 * (10 - 5) - 10 * 8 / 9"),
            a(
                a(
                    l(22),
                    Add,
                    a(
                        l(21),
                        Mul,
                        Expr::Paren(b(
                            a(
                                l(10),
                                Sub,
                                l(5)
                            )
                        )),
                    ),
                ),
                Sub,
                a(
                    a(
                        l(10),
                        Mul,
                        l(8)
                    ),
                    Div,
                    l(9),
                )
            )
        );
    }

    #[test]
    fn empty_program() {
        assert_eq!(parse_program(""), CacauProgram { items: vec![] });
    }

    #[test]
    fn hello_world() {
        assert_eq!(
            parse_program(r#"println("Hello, world!");"#),
            CacauProgram {
                items: vec![Statement::Expr(Expr::FnCall(b(FnCall {
                    callee: Expr::Id("println".into()),
                    params: vec![Expr::Lit(Lit::String("Hello, world!".into()))]
                })))]
            }
        );
    }

    #[test]
    fn empty_string() {
        assert_eq!(parse_exp(r#""""#), Expr::Lit(Lit::String("".into())))
    }

    #[test]
    fn nonempty_string() {
        assert_eq!(
            parse_exp(r#""Test String""#),
            Expr::Lit(Lit::String("Test String".into()))
        )
    }

    #[test]
    fn bool_false() {
        assert_eq!(parse_exp("false"), Expr::Lit(Lit::Bool(false)))
    }

    #[test]
    fn bool_true() {
        assert_eq!(parse_exp("true"), Expr::Lit(Lit::Bool(true)))
    }

    #[test]
    fn simplest_function_call() {
        assert_eq!(
            parse_exp("f()"),
            Expr::FnCall(b(FnCall {
                callee: Expr::Id("f".into()),
                params: vec![]
            }))
        )
    }

    #[test]
    fn single_param_function_call() {
        assert_eq!(
            parse_exp("f(1)"),
            Expr::FnCall(b(FnCall {
                callee: Expr::Id("f".into()),
                params: vec![Expr::Lit(Lit::Int(1))]
            }))
        )
    }

    #[test]
    fn single_param_function_call2() {
        assert_eq!(
            parse_exp("f(\"a\")"),
            Expr::FnCall(b(FnCall {
                callee: Expr::Id("f".into()),
                params: vec![Expr::Lit(Lit::String("a".into()))]
            }))
        )
    }
}
