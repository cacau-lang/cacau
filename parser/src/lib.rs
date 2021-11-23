#[macro_use]
extern crate pest_ast;

pub use pest::Parser as ParserTrait;

use pest_derive::Parser;

pub mod ast;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ExpressionParser;

#[cfg(test)]
mod parser_tests {
    use pest::Parser;

    use crate::ExpressionParser;
    use crate::Rule;

    fn parse(rule: Rule, expression: &str) -> Option<()> {
        ExpressionParser::parse(rule, expression).ok().map(|_| ())
    }

    fn assert_parses(rule: Rule, expression: &str) {
        assert!(parse(rule, expression).is_some())
    }

    fn assert_does_not_parse(rule: Rule, expression: &str) {
        assert!(parse(rule, expression).is_none())
    }

    #[test]
    fn single_char_identifiers() {
        assert_parses(Rule::identifier, "x");
        assert_parses(Rule::identifier, "z");
        assert_parses(Rule::identifier, "_");

        assert_does_not_parse(Rule::identifier, "2");
        assert_does_not_parse(Rule::identifier, "7");
    }

    #[test]
    fn identifiers() {
        assert_parses(Rule::identifier, "a");
        assert_parses(Rule::identifier, "_abc");
        assert_parses(Rule::identifier, "abc_123");
        assert_parses(Rule::identifier, "zazz123");

        // Keywords are reserved
        assert_does_not_parse(Rule::identifier, "let");
        assert_does_not_parse(Rule::identifier, "if");
        assert_does_not_parse(Rule::identifier, "then");
        assert_does_not_parse(Rule::identifier, "else");
    }

    // TODO: this test is failing ;-;
    // #[test]
    // pub fn invalid_identifiers() {
    //     assert_does_not_parse(Rule::identifier, "2");
    //     assert_does_not_parse(Rule::identifier, "3abc");
    //     assert_does_not_parse(Rule::identifier, "@abc123");
    //     assert_does_not_parse(Rule::identifier, "zaz@z123");
    // }

    #[test]
    fn integers() {
        assert_parses(Rule::expression, "1234");
        assert_parses(Rule::expression, "000000");
        assert_parses(Rule::expression, "987654321");
        assert_parses(Rule::expression, "987654321");

        assert_parses(Rule::integer, "1234");
        assert_parses(Rule::integer, "000000");
        assert_parses(Rule::integer, "987654321");
        assert_parses(Rule::integer, "987654321");
    }

    #[test]
    fn floats() {
        assert_parses(Rule::expression, "123.4");
        assert_parses(Rule::expression, "000.000");
        assert_parses(Rule::expression, "98.7654321");
        assert_parses(Rule::expression, "987654.321");

        assert_parses(Rule::float, "123.4");
        assert_parses(Rule::float, "000.000");
        assert_parses(Rule::float, "98.7654321");
        assert_parses(Rule::float, "987654.321");

        assert_does_not_parse(Rule::float, "123");
        assert_does_not_parse(Rule::float, "50000");
        assert_does_not_parse(Rule::float, "abc123");
    }

    #[test]
    fn type_annotations() {
        assert_parses(Rule::type_annotation, ": int");
        assert_parses(Rule::type_annotation, ": float");
        assert_parses(Rule::type_annotation, ": string");

        assert_does_not_parse(Rule::type_annotation, "123");
        assert_does_not_parse(Rule::type_annotation, "string");
        assert_does_not_parse(Rule::type_annotation, ": 123");
    }

    #[test]
    fn assignment() {
        // TODO: add other kinds of expressions here when possible

        assert_parses(Rule::expression, "let x = 123");
        assert_parses(Rule::expression, "let nines = 999");

        assert_parses(Rule::assignment, "let x = \"Hey there\"");
        assert_parses(Rule::assignment, "let nines = 'c'");

        assert_parses(Rule::assignment, "let six: int = 5");

        assert_parses(Rule::assignment, "let six: BigDecimal = 10");

        assert_does_not_parse(Rule::assignment, "let name");
        assert_does_not_parse(Rule::assignment, "let ch x: b");
        assert_does_not_parse(Rule::assignment, "six: BigDecimal = 10");
    }

    #[test]
    fn function_argument() {
        assert_parses(Rule::function_argument, "x: int");
        assert_parses(Rule::function_argument, "y: string");
        assert_parses(Rule::function_argument, "lambda: num");

        assert_does_not_parse(Rule::function_argument, "x: 123");
        assert_does_not_parse(Rule::function_argument, "y: \"Hey\"");
    }

    #[test]
    fn function_return() {
        assert_parses(Rule::function_return, "-> x");
        assert_parses(Rule::function_return, "-> y");
        assert_parses(Rule::function_return, "-> z");

        assert_does_not_parse(Rule::function_return, "-> 123");
        assert_does_not_parse(Rule::function_return, "->");
        assert_does_not_parse(Rule::function_return, "z");
    }

    #[test]
    fn function_declaration() {
        // TODO: rewrite these function indentifiers whenever the underline is available
        assert_parses(Rule::function_declaration, "fn main");
        assert_parses(
            Rule::function_declaration,
            "pub fn no_args_but_returns_something -> int",
        );
        assert_parses(
            Rule::function_declaration,
            "fn no_args_but_returns_something -> int",
        );
        assert_parses(Rule::function_declaration, "fn one_arg_no_return x: int");
        assert_parses(
            Rule::function_declaration,
            "fn one_arg_one_return x: int -> int",
        );
        assert_parses(Rule::function_declaration, "fn several_args x: int, y: int");
        assert_parses(
            Rule::function_declaration,
            "pub fn several_args x: int, y: int",
        );
        assert_parses(
            Rule::function_declaration,
            "fn several_args_with_return x: int, y: int, z: int -> bool",
        );

        assert_does_not_parse(Rule::function_declaration, "fn");
        assert_does_not_parse(Rule::function_declaration, "pub fn");
        assert_does_not_parse(Rule::function_declaration, "fn -> bool");

        // TODO: use SOI and EOI matching here whenever available
        // No function name
        // assert_does_not_parse(Rule::function_declaration, "fn x: int, y: int, z: int -> bool");

        // Arrow set but return type not specified
        // assert_does_not_parse(Rule::function_declaration, "fn func x: int, y: int, z: int ->");

        // Incorrect argument specification
        // assert_does_not_parse(Rule::function_declaration, "fn func x: -> bool");
    }

    #[test]
    fn char() {
        // TODO: ideally check that Unicode values parse

        for ch in 'a'..'Z' {
            let ch = format!("'{}'", ch);
            assert_parses(Rule::char, &ch);
            assert_parses(Rule::expression, &ch);
        }

        assert_does_not_parse(Rule::char, "''");
        assert_does_not_parse(Rule::expression, "''");
    }

    #[test]
    fn string() {
        assert_parses(Rule::string, "\"HELLO THERE\"");
        assert_parses(Rule::string, "\"\"");

        assert_parses(Rule::string, "\"órgão público\"");

        assert_does_not_parse(Rule::char, "''");

        assert_does_not_parse(Rule::char, "\"");

        assert_does_not_parse(Rule::char, "some text without enclosing double quotes");

        assert_does_not_parse(Rule::char, "\"some text with missing closing double quotes");
    }

    #[test]
    fn boolean() {
        assert_parses(Rule::boolean, "true");
        assert_parses(Rule::boolean, "false");

        assert_does_not_parse(Rule::boolean, "False");
        assert_does_not_parse(Rule::boolean, "Talse");
    }

    #[test]
    fn boolean_operations() {
        assert_parses(Rule::boolean_expr, "true and false");
        assert_parses(Rule::boolean_expr, "false or not true");
        assert_parses(Rule::boolean_expr, "false or (not (true or false))");
        assert_parses(Rule::boolean_expr, "false or (false and true)");
        assert_parses(Rule::boolean_expr, "false or (false and (true or false))");
        assert_parses(Rule::boolean_expr, "not true");
        assert_parses(
            Rule::boolean_expr,
            "false or (false and (true or (true and false)))",
        );
        assert_parses(
            Rule::boolean_expr,
            "(false and (true or (true and false))) or (false and (true or (true and (true))))",
        );
    }

    #[test]
    fn enum_definition() {
        assert_parses(
            Rule::enum_definition,
            "pub enum NameOrId { Name(string), Id(Uuid) }",
        );
        assert_parses(Rule::enum_definition, "pub enum Status { Polling, Ready }");
        assert_parses(Rule::enum_definition, "pub enum Status { Polling, Ready, }");
        assert_parses(Rule::enum_definition, "pub enum NoVariant { }");

        assert_does_not_parse(Rule::enum_definition, "pub enum");

        assert_does_not_parse(Rule::enum_definition, "pub enum NoVariantDoneWrong");

        // Enum without name
        assert_does_not_parse(Rule::enum_definition, "pub enum { Polling, Ready }");
        // Wrong keyword
        assert_does_not_parse(Rule::enum_definition, "pub enub Status { Polling, Ready }");
    }

    #[test]
    fn struct_definition() {
        assert_parses(Rule::struct_definition, "struct User { }");
        assert_parses(Rule::struct_definition, "pub struct User { }");
        assert_parses(
            Rule::struct_definition,
            "pub struct User { pub username: string, age: int, birth_date: Date }",
        );

        // Missing braces
        assert_does_not_parse(Rule::struct_definition, "struct User");

        // Missing type for field
        assert_does_not_parse(
            Rule::struct_definition,
            "pub struct User { pub username: , age: int, birth_date: Date }",
        );
    }

    #[test]
    fn math_op() {
        assert_parses(Rule::math_expr, "2");
        assert_parses(Rule::math_expr, "-2");
        assert_parses(Rule::math_expr, "-2 * 5");
        assert_parses(Rule::math_expr, "-(2*5)");
        assert_parses(Rule::math_expr, "x + y");
        assert_parses(Rule::math_expr, "2 + 2");
        assert_parses(Rule::math_expr, "2 - (2*3)");
        assert_parses(Rule::math_expr, "(2^3)/2");
        assert_parses(Rule::math_expr, "(2-y) * (3+6)");

        assert_parses(Rule::expression, "2");
        assert_parses(Rule::expression, "2 + 2");
        assert_parses(Rule::expression, "2 - (2*3)");
        assert_parses(Rule::expression, "(2^3)/2");
        assert_parses(Rule::expression, "(2-2) * (3+6)");
        assert_parses(Rule::expression, "(2-2) % (3+6)");

        assert_does_not_parse(Rule::math_expr, "-");
        assert_does_not_parse(Rule::math_expr, "()");

        assert_does_not_parse(Rule::expression, "()");
        assert_does_not_parse(Rule::math_expr, "*3");
    }

    #[test]
    fn function_definition() {
        // TODO: Add identifiers here whenever possible

        assert_parses(Rule::function_definition, "fn two -> int { 2 }");
        assert_parses(
            Rule::function_definition,
            "fn double x: int -> int { 2 + 2 }",
        );
        assert_parses(
            Rule::function_definition,
            "pub fn double x: int, b: bool, s: str -> SomeType { 2 * (2-3) ^ 5 }",
        );

        assert_does_not_parse(
            Rule::function_definition,
            "pub fn double x: int, b: bool, s: str -> SomeType { 2 * (2-3) ^ true }",
        );
        assert_does_not_parse(
            Rule::function_definition,
            "fn x: int -> int { true and (false) }",
        );
        assert_does_not_parse(Rule::function_definition, "2 + 2");
        assert_does_not_parse(Rule::function_definition, "2");
    }

    #[test]
    fn if_expr() {
        assert_parses(Rule::program, "if true and false { 2*2 } else { 5}");
        assert_parses(Rule::program, "if true and false { false }");
        assert_parses(Rule::if_expr, "if true and false { false }");
        assert_parses(Rule::if_expr, "if 2*6 { false }");

        assert_does_not_parse(Rule::program, "if true and false { 2*2 } else");
        assert_does_not_parse(Rule::program, "if true and false 2*2 ");
    }

    #[test]
    fn elif() {
        assert_parses(Rule::if_expr, "if some_condition { 2*2 }");
        assert_parses(
            Rule::if_expr,
            "if some_condition { 2*2 } elif another_condition { 4* 4} ",
        );
        assert_parses(
            Rule::if_expr,
            "if some_condition { 2*2 } elif another_condition { 4* 4} else { 6*6 } ",
        );

        assert_does_not_parse(Rule::if_expr, "if some_condition ");
        assert_does_not_parse(
            Rule::if_expr,
            "if some_condition  elif another_condition { 4* 4} ",
        );
        assert_does_not_parse(
            Rule::program,
            "if some_condition { 2*2 } elif another_condition { 4* 4} else ",
        );
    }

    #[test]
    fn program() {
        // TODO: further testing

        let valid_program = r###"
        pub struct UserData {
            pub name: string,
            password: ZeroableString
         }

        pub enum User {
            Admin(UserData),
            Regular(UserData),
        }

        pub fn four -> int {
           2 + 2
        }"###;

        let missing_else_expr = r###"
        pub fn test {
            if true and false { 2*2 } else
        }
        "###;

        assert_parses(Rule::program, valid_program);
        assert_does_not_parse(Rule::program, missing_else_expr);
    }

    #[test]
    fn comparison() {
        assert_parses(Rule::comparison, "x == y");
        assert_parses(Rule::comparison, "2 != 3");
        assert_parses(Rule::comparison, "x != y");
        assert_parses(Rule::comparison, "x >= y");
        assert_parses(Rule::comparison, "x <= y");
        assert_parses(Rule::comparison, "x < y");
        assert_parses(Rule::comparison, "x > y");
        assert_parses(Rule::comparison, "\"abc\" > \"oop\"");

        assert_does_not_parse(Rule::comparison, "x>");
        assert_does_not_parse(Rule::comparison, "y<=");
        assert_does_not_parse(Rule::comparison, "==");
        assert_does_not_parse(Rule::comparison, "<= z");
    }

    #[test]
    fn function_calls() {
        assert_parses(Rule::function_call, "print()");
        assert_parses(Rule::function_call, "print('b')");
        assert_parses(Rule::function_call, "println(\"haha\", 'c', 2, 2*2)");
        assert_parses(Rule::function_call, "println(double)");

        assert_does_not_parse(Rule::function_call, "(\"haha\", 'c', 2, 2*2)");
    }
}
