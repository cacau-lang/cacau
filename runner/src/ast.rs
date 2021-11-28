pub struct CacauProgram<'a> {
    pub items: Vec<HighLevelItem<'a>>,
}

pub enum HighLevelItem<'a> {
    Fn(Function<'a>),
    Struct(),
    Enum(),
    Expr(Expression<'a>),
}

pub struct Function<'a> {
    pub public: bool,
    pub name: &'a str,
    pub params: Vec<FunctionArgument<'a>>,
    pub output: &'a str,
    pub body: Expression<'a>,
}

pub struct Struct<'a> {
    pub name: &'a str,
    // TODO fiels and their types
}

pub struct Enum<'a> {
    pub name: &'a str,
    // TODO variants
}

pub struct Assignment<'a> {
    pub name: &'a str,
    pub type_annotation: Option<&'a str>,
    pub expression: Expression<'a>,
}

pub enum Expression<'a> {
    Identifier(&'a str),
    Assignment(Box<Assignment<'a>>),
    BooleanLiteral(bool),
    IntegerLiteral(i64),
    CharLiteral(char),
    StringLiteral(&'a str),
    FunctionCall(FunctionCall<'a>),
    ArithOperation(ArithmeticOperation<'a>),
    CompOperation(ComparisonOperation<'a>),
    BoolOperation(BooleanOperation<'a>),
    Negation(),
}

pub struct ArithmeticOperation<'a> {
    pub left: Box<Expression<'a>>,
    pub op: ArithmeticOperator,
    pub right: Box<Expression<'a>>,
}

pub struct ComparisonOperation<'a> {
    pub left: Box<Expression<'a>>,
    pub op: ComparisonOperator,
    pub right: Box<Expression<'a>>,
}

pub struct BooleanOperation<'a> {
    pub left: Box<Expression<'a>>,
    pub op: BooleanOperator,
    pub right: Box<Expression<'a>>,
}

pub enum ArithmeticOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Modulo,
}

pub enum BooleanOperator {
    Or,
    And,
    Xor,
}

pub enum ComparisonOperator {
    Equals,
    NotEquals,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

pub struct FunctionCall<'a> {
    pub name: &'a str,
    pub params: Vec<Expression<'a>>,
}

pub struct FunctionArgument<'a> {
    pub name: &'a str,
    pub type_: &'a str,
}
