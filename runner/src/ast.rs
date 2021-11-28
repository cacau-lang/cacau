pub struct CacauProgram<'a> {
    pub items: Vec<HighLevelItem<'a>>,
}

#[derive(Debug)]
pub enum HighLevelItem<'a> {
    Fn(Function<'a>),
    Struct(),
    Enum(),
    Expr(Expression<'a>),
}

#[derive(Debug)]
pub struct Function<'a> {
    pub public: bool,
    pub name: &'a str,
    pub params: Vec<FunctionArgument<'a>>,
    pub output: &'a str,
    pub body: Expression<'a>,
}

#[derive(Debug)]
pub struct Struct<'a> {
    pub name: &'a str,
    // TODO fiels and their types
}

#[derive(Debug)]
pub struct Enum<'a> {
    pub name: &'a str,
    // TODO variants
}

#[derive(Debug)]
pub struct Assignment<'a> {
    pub name: &'a str,
    pub type_annotation: Option<&'a str>,
    pub expression: Expression<'a>,
}

#[derive(Debug)]
pub enum Expression<'a> {
    Identifier(&'a str),
    Assignment(Box<Assignment<'a>>),
    BooleanLiteral(bool),
    IntegerLiteral(i64),
    FloatLiteral(f64),
    CharLiteral(char),
    StringLiteral(&'a str),
    FunctionCall(FunctionCall<'a>),
    ArithOperation(Box<ArithmeticOperation<'a>>),
    CompOperation(Box<ComparisonOperation<'a>>),
    BoolOperation(Box<BooleanOperation<'a>>),
    Negation(),
}

#[derive(Debug)]
pub struct ArithmeticOperation<'a> {
    pub left: Expression<'a>,
    pub op: ArithmeticOperator,
    pub right: Expression<'a>,
}

#[derive(Debug)]
pub struct ComparisonOperation<'a> {
    pub left: Expression<'a>,
    pub op: ComparisonOperator,
    pub right: Expression<'a>,
}

#[derive(Debug)]
pub struct BooleanOperation<'a> {
    pub left: Expression<'a>,
    pub op: BooleanOperator,
    pub right: Expression<'a>,
}

#[derive(Debug)]
pub enum ArithmeticOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Modulo,
}

#[derive(Debug)]
pub enum BooleanOperator {
    Or,
    And,
    Xor,
}

#[derive(Debug)]
pub enum ComparisonOperator {
    Equals,
    NotEquals,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

#[derive(Debug)]
pub struct FunctionCall<'a> {
    pub name: &'a str,
    pub params: Vec<Expression<'a>>,
}

#[derive(Debug)]
pub struct FunctionArgument<'a> {
    pub name: &'a str,
    pub type_: &'a str,
}
