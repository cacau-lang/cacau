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
    pub name: &'a str,
    // TODO params, body, return type
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
    Boolean(bool),
    Integer(i64),
    Char(&'a str),
    FunctionCall(FunctionCall<'a>),
}

pub struct FunctionCall<'a> {
    pub name: &'a str,
    pub params: Vec<Expression<'a>>,
}
