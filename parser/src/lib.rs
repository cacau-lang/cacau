mod ast;

pub use pest::iterators::Pairs;
use pest_consume::{Error, Parser};

use ast::Term;

type Node<'i> = pest_consume::Node<'i, Rule, ()>;

// This creates `Rule`
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct CacauParser;

pub fn parse(contents: &str) -> Result<(Node, Term), Error<Rule>> {
    let inputs = CacauParser::parse(Rule::ArithmeticOperation, contents)?;
    let parser_tree = inputs.single()?;
    dbg!(&parser_tree);

    let ast = ast::okay(parser_tree.clone())?;
    Ok((parser_tree, ast))
}

#[cfg(test)]
mod parser_tests {
    use super::*;
    use pest_consume::Nodes;

    // Check if expression matches rule && expression has no remainder
    fn whole_expression_parses(rule: Rule, expression: &str) -> bool {
        let inner = |mut nodes: Nodes<Rule, ()>| -> Option<bool> {
            let first = nodes.next()?;
            let second = nodes.next();
            Some(first.as_str() == expression && second.is_none())
        };

        CacauParser::parse(rule, expression)
            .ok()
            .and_then(inner)
            .unwrap_or(false)
    }

    macro_rules! parse_ok {
        ($rule: expr, $expression: expr) => {
            assert!(whole_expression_parses($rule, $expression))
        };
    }

    macro_rules! parse_err {
        ($rule: expr, $expression: expr) => {
            assert!(!whole_expression_parses($rule, $expression))
        };
    }

    #[test]
    fn identifiers() {
        // Reserved keywords:
        parse_err!(Rule::Identifier, "if");
        parse_err!(Rule::Identifier, "for");
        parse_ok!(Rule::Identifier, "if_");
        parse_ok!(Rule::Identifier, "for_");

        parse_ok!(Rule::Identifier, "snake_case_example");
        parse_ok!(Rule::Identifier, "MyStruct");
        parse_ok!(Rule::Identifier, "SCREAMANDSHOUT");
        parse_ok!(Rule::Identifier, "WITH_UNDERLINE");
        parse_ok!(Rule::Identifier, "x");
        parse_ok!(Rule::Identifier, "_");
        parse_ok!(Rule::Identifier, "____");
        parse_ok!(Rule::Identifier, "_a");
        parse_ok!(Rule::Identifier, "_________a");
        parse_ok!(Rule::Identifier, "a0");

        parse_err!(Rule::Identifier, "2");
        parse_err!(Rule::Identifier, "3abc");
        parse_err!(Rule::Identifier, "$anything");
        parse_err!(Rule::Identifier, "(anything)");

        // @ in identifiers
        parse_ok!(Rule::Identifier, "@anything");
        parse_ok!(Rule::Identifier, "@x0");
        parse_err!(Rule::Identifier, "zaz@z123");
        parse_err!(Rule::Identifier, "@0a");
    }

    #[test]
    fn integers() {
        parse_ok!(Rule::IntegerLiteral, "0000000001293812753840");
        parse_ok!(Rule::IntegerLiteral, "1_0000_0000");
        parse_ok!(Rule::IntegerLiteral, "100_000_000");
        parse_ok!(Rule::IntegerLiteral, "0");

        parse_err!(Rule::IntegerLiteral, "0_");
        parse_err!(Rule::IntegerLiteral, "_100_000_000");
        parse_err!(Rule::IntegerLiteral, "1__0000_0000");
        parse_err!(Rule::IntegerLiteral, "000000000129__3812753840");
    }

    #[test]
    fn floats() {
        parse_ok!(Rule::FloatLiteral, "123.4");
        parse_ok!(Rule::FloatLiteral, "000.000");
        parse_ok!(Rule::FloatLiteral, "98.7654321");
        parse_ok!(Rule::FloatLiteral, "987_654.50");
        parse_ok!(Rule::FloatLiteral, "987_654.100_100");

        parse_err!(Rule::FloatLiteral, "123");
        parse_err!(Rule::FloatLiteral, "1.");
        parse_err!(Rule::FloatLiteral, ".1");
        parse_err!(Rule::FloatLiteral, "5000..0");
        parse_err!(Rule::FloatLiteral, "1.2.3");
    }

    #[test]
    fn type_annotations() {
        parse_ok!(Rule::TypeAnnotation, ": int");
        parse_ok!(Rule::TypeAnnotation, ": float");
        parse_ok!(Rule::TypeAnnotation, ": string");

        parse_err!(Rule::TypeAnnotation, "123");
        parse_err!(Rule::TypeAnnotation, "string");
        parse_err!(Rule::TypeAnnotation, ": 10");
    }

    // #[test]
    // fn assignment() {
    //     // TODO: add other kinds of expressions here when possible
    //     parse_ok!(Rule::Expression, "let x = 123");
    //     parse_ok!(Rule::Expression, "let nines = 999");
    // }

    #[test]
    fn assignment() {
        parse_ok!(Rule::AssignmentStatement, "x = 1;");
        parse_ok!(Rule::AssignmentStatement, "x += 1;");
        parse_ok!(Rule::AssignmentStatement, "x -= 1;");
        parse_ok!(Rule::AssignmentStatement, "x *= 1;");
        parse_ok!(Rule::AssignmentStatement, "x /= 1;");
        parse_ok!(Rule::AssignmentStatement, "x ^= 1;");
        parse_ok!(Rule::AssignmentStatement, "x %= 1;");
        parse_ok!(Rule::AssignmentStatement, "x &= 1;");
        parse_ok!(Rule::AssignmentStatement, "x |= 1;");
        parse_ok!(Rule::AssignmentStatement, "x <<= 1;");
        parse_ok!(Rule::AssignmentStatement, "x >>= 1;");
    }

    #[test]
    fn let_statement() {
        parse_ok!(Rule::LetStatement, "let x = \"Hey there\";");
        parse_ok!(Rule::LetStatement, "let nines = 'c';");
        parse_ok!(Rule::LetStatement, "let six: int = 5;");
        parse_ok!(Rule::LetStatement, "let six: BigDecimal = 10;");
        parse_err!(Rule::LetStatement, "let name;");
        parse_err!(Rule::LetStatement, "let ch x: b;");
        parse_err!(Rule::LetStatement, "six: BigDecimal = 10;");
    }

    #[test]
    fn function_argument_list() {
        parse_ok!(Rule::FunctionParameterList, "x: int");
        parse_ok!(Rule::FunctionParameterList, "x: int,");
        parse_ok!(Rule::FunctionParameterList, "x: int, y: int");
        parse_ok!(Rule::FunctionParameterList, "x: int, y: int,");
        parse_ok!(Rule::FunctionParameterList, "y:y");
        parse_err!(Rule::FunctionParameterList, ",x: int");
        parse_err!(Rule::FunctionParameterList, "x, y");
        parse_err!(Rule::FunctionParameterList, "x: 123");
    }

    // #[test]
    // fn function_return() {
    //     parse_ok!(Rule::FunctionReturn, "-> x");
    //     parse_ok!(Rule::FunctionReturn, "-> y");
    //     parse_ok!(Rule::FunctionReturn, "-> z");

    //     parse_err!(Rule::FunctionReturn, "-> 123");
    //     parse_err!(Rule::FunctionReturn, "->");
    //     parse_err!(Rule::FunctionReturn, "z");
    // }

    // #[test]
    // fn function_declaration() {
    //     // TODO: rewrite these function indentifiers whenever the underline is available
    //     parse_ok!(Rule::FunctionDeclaration, "fn main");
    //     parse_ok!(
    //         Rule::FunctionDeclaration,
    //         "pub fn no_args_but_returns_something -> int",
    //     );
    //     parse_ok!(
    //         Rule::FunctionDeclaration,
    //         "fn no_args_but_returns_something -> int",
    //     );
    //     parse_ok!(Rule::FunctionDeclaration, "fn one_arg_no_return x: int");
    //     parse_ok!(
    //         Rule::FunctionDeclaration,
    //         "fn one_arg_one_return x: int -> int",
    //     );
    //     parse_ok!(Rule::FunctionDeclaration, "fn several_args x: int, y: int");
    //     parse_ok!(
    //         Rule::FunctionDeclaration,
    //         "pub fn several_args x: int, y: int",
    //     );
    //     parse_ok!(
    //         Rule::FunctionDeclaration,
    //         "fn several_args_with_return x: int, y: int, z: int -> bool",
    //     );

    //     parse_err!(Rule::FunctionDeclaration, "fn");
    //     parse_err!(Rule::FunctionDeclaration, "pub fn");
    //     parse_err!(Rule::FunctionDeclaration, "fn -> bool");

    //     // TODO: use SOI and EOI matching here whenever available
    //     // No function name
    //     // parse_err!(Rule::FunctionDeclaration, "fn x: int, y: int, z: int -> bool");

    //     // Arrow set but return type not specified
    //     // parse_err!(Rule::FunctionDeclaration, "fn func x: int, y: int, z: int ->");

    //     // Incorrect argument specification
    //     // parse_err!(Rule::FunctionDeclaration, "fn func x: -> bool");
    // }

    // #[test]
    // fn char() {
    //     // TODO: ideally check that Unicode values parse

    //     for ch in 'a'..'Z' {
    //         let ch = format!("'{}'", ch);
    //         parse_ok!(Rule::Char, &ch);
    //         parse_ok!(Rule::Expression, &ch);
    //     }

    //     parse_err!(Rule::Char, "''");
    //     parse_err!(Rule::Expression, "''");
    // }

    // #[test]
    // fn string() {
    //     parse_ok!(Rule::String, "\"HELLO THERE\"");
    //     parse_ok!(Rule::String, "\"\"");

    //     parse_ok!(Rule::String, "\"órgão público\"");

    //     parse_err!(Rule::Char, "''");

    //     parse_err!(Rule::Char, "\"");

    //     parse_err!(Rule::Char, "some text without enclosing double quotes");

    //     parse_err!(Rule::Char, "\"some text with missing closing double quotes");
    // }

    // #[test]
    // fn boolean() {
    //     parse_ok!(Rule::Boolean, "true");
    //     parse_ok!(Rule::Boolean, "false");

    //     parse_err!(Rule::Boolean, "False");
    //     parse_err!(Rule::Boolean, "Talse");
    // }

    // #[test]
    // fn boolean_operations() {
    //     parse_ok!(Rule::BooleanExpr, "true and false");
    //     parse_ok!(Rule::BooleanExpr, "false or not true");
    //     parse_ok!(Rule::BooleanExpr, "false or (not (true or false))");
    //     parse_ok!(Rule::BooleanExpr, "false or (false and true)");
    //     parse_ok!(Rule::BooleanExpr, "false or (false and (true or false))");
    //     parse_ok!(Rule::BooleanExpr, "not true");
    //     parse_ok!(
    //         Rule::BooleanExpr,
    //         "false or (false and (true or (true and false)))",
    //     );
    //     parse_ok!(
    //         Rule::BooleanExpr,
    //         "(false and (true or (true and false))) or (false and (true or (true and (true))))",
    //     );
    // }

    // #[test]
    // fn enum_definition() {
    //     parse_ok!(
    //         Rule::EnumDefinition,
    //         "pub enum NameOrId { Name(string), Id(Uuid) }",
    //     );
    //     parse_ok!(Rule::EnumDefinition, "pub enum Status { Polling, Ready }");
    //     parse_ok!(Rule::EnumDefinition, "pub enum Status { Polling, Ready, }");
    //     parse_ok!(Rule::EnumDefinition, "pub enum NoVariant { }");

    //     parse_err!(Rule::EnumDefinition, "pub enum");

    //     parse_err!(Rule::EnumDefinition, "pub enum NoVariantDoneWrong");

    //     // Enum without name
    //     parse_err!(Rule::EnumDefinition, "pub enum { Polling, Ready }");
    //     // Wrong keyword
    //     parse_err!(Rule::EnumDefinition, "pub enub Status { Polling, Ready }");
    // }

    // #[test]
    // fn struct_definition() {
    //     parse_ok!(Rule::StructDefinition, "struct User { }");
    //     parse_ok!(Rule::StructDefinition, "pub struct User { }");
    //     parse_ok!(
    //         Rule::StructDefinition,
    //         "pub struct User { pub username: string, age: int, birth_date: Date }",
    //     );

    //     // Missing braces
    //     parse_err!(Rule::StructDefinition, "struct User");

    //     // Missing type for field
    //     parse_err!(
    //         Rule::StructDefinition,
    //         "pub struct User { pub username: , age: int, birth_date: Date }",
    //     );
    // }

    // #[test]
    // fn math_op() {
    //     parse_ok!(Rule::MathExpr, "2");
    //     parse_ok!(Rule::MathExpr, "-2");
    //     parse_ok!(Rule::MathExpr, "-2 * 5");
    //     parse_ok!(Rule::MathExpr, "-(2*5)");
    //     parse_ok!(Rule::MathExpr, "x + y");
    //     parse_ok!(Rule::MathExpr, "2 + 2");
    //     parse_ok!(Rule::MathExpr, "2 - (2*3)");
    //     parse_ok!(Rule::MathExpr, "(2^3)/2");
    //     parse_ok!(Rule::MathExpr, "(2-y) * (3+6)");

    //     parse_ok!(Rule::Expression, "2");
    //     parse_ok!(Rule::Expression, "2 + 2");
    //     parse_ok!(Rule::Expression, "2 - (2*3)");
    //     parse_ok!(Rule::Expression, "(2^3)/2");
    //     parse_ok!(Rule::Expression, "(2-2) * (3+6)");
    //     parse_ok!(Rule::Expression, "(2-2) % (3+6)");

    //     parse_err!(Rule::MathExpr, "-");
    //     parse_err!(Rule::MathExpr, "()");

    //     parse_err!(Rule::Expression, "()");
    //     parse_err!(Rule::MathExpr, "*3");
    // }

    // #[test]
    // fn function_definition() {
    //     // TODO: Add identifiers here whenever possible

    //     parse_ok!(Rule::FunctionDefinition, "fn two -> int { 2 }");
    //     parse_ok!(
    //         Rule::FunctionDefinition,
    //         "fn double x: int -> int { 2 + 2 }",
    //     );
    //     parse_ok!(
    //         Rule::FunctionDefinition,
    //         "pub fn double x: int, b: bool, s: str -> SomeType { 2 * (2-3) ^ 5 }",
    //     );

    //     parse_err!(
    //         Rule::FunctionDefinition,
    //         "pub fn double x: int, b: bool, s: str -> SomeType { 2 * (2-3) ^ true }",
    //     );
    //     parse_err!(
    //         Rule::FunctionDefinition,
    //         "fn x: int -> int { true and (false) }",
    //     );
    //     parse_err!(Rule::FunctionDefinition, "2 + 2");
    //     parse_err!(Rule::FunctionDefinition, "2");
    // }

    // #[test]
    // fn if_expr() {
    //     parse_ok!(Rule::Program, "if true and false { 2*2 } else { 5}");
    //     parse_ok!(Rule::Program, "if true and false { false }");
    //     parse_ok!(Rule::IfExpr, "if true and false { false }");
    //     parse_ok!(Rule::IfExpr, "if 2*6 { false }");

    //     parse_err!(Rule::Program, "if true and false { 2*2 } else");
    //     parse_err!(Rule::Program, "if true and false 2*2 ");
    // }

    // #[test]
    // fn elif() {
    //     parse_ok!(Rule::IfExpr, "if some_condition { 2*2 }");
    //     parse_ok!(
    //         Rule::IfExpr,
    //         "if some_condition { 2*2 } elif another_condition { 4* 4} ",
    //     );
    //     parse_ok!(
    //         Rule::IfExpr,
    //         "if some_condition { 2*2 } elif another_condition { 4* 4} else { 6*6 } ",
    //     );

    //     parse_err!(Rule::IfExpr, "if some_condition ");
    //     parse_err!(
    //         Rule::IfExpr,
    //         "if some_condition  elif another_condition { 4* 4} ",
    //     );
    //     parse_err!(
    //         Rule::Program,
    //         "if some_condition { 2*2 } elif another_condition { 4* 4} else ",
    //     );
    // }

    // #[test]
    // fn program() {
    //     // TODO: further testing

    //     let valid_program = r###"
    //     pub struct UserData {
    //         pub name: string,
    //         password: ZeroableString
    //      }

    //     pub enum User {
    //         Admin(UserData),
    //         Regular(UserData),
    //     }

    //     pub fn four -> int {
    //        2 + 2
    //     }"###;

    //     let missing_else_expr = r###"
    //     pub fn test {
    //         if true and false { 2*2 } else
    //     }
    //     "###;

    //     parse_ok!(Rule::Program, valid_program);
    //     parse_err!(Rule::Program, missing_else_expr);
    // }

    // #[test]
    // fn comparison() {
    //     parse_ok!(Rule::Comparison, "x == y");
    //     parse_ok!(Rule::Comparison, "2 != 3");
    //     parse_ok!(Rule::Comparison, "x != y");
    //     parse_ok!(Rule::Comparison, "x >= y");
    //     parse_ok!(Rule::Comparison, "x <= y");
    //     parse_ok!(Rule::Comparison, "x < y");
    //     parse_ok!(Rule::Comparison, "x > y");
    //     parse_ok!(Rule::Comparison, "\"abc\" > \"oop\"");

    //     parse_err!(Rule::Comparison, "x>");
    //     parse_err!(Rule::Comparison, "y<=");
    //     parse_err!(Rule::Comparison, "==");
    //     parse_err!(Rule::Comparison, "<= z");
    // }

    // #[test]
    // fn function_calls() {
    //     parse_ok!(Rule::FunctionCall, "print()");
    //     parse_ok!(Rule::FunctionCall, "print('b')");
    //     parse_ok!(Rule::FunctionCall, "println(\"haha\", 'c', 2, 2*2)");
    //     parse_ok!(Rule::FunctionCall, "println(double)");

    //     parse_err!(Rule::FunctionCall, "(\"haha\", 'c', 2, 2*2)");
    // }
}
