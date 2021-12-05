#[derive(Debug, PartialEq)]
pub struct CacauProgram {
    pub items: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Fn(Function),
    Struct(),
    Enum(),
    Expr(Expr),
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub public: bool,
    pub name: String,
    pub params: Vec<FunctionArgument>,
    pub output: String,
    pub body: Expr,
}

#[derive(Debug, PartialEq)]
pub struct Struct {
    pub name: String,
    // TODO fields and their types
}

#[derive(Debug, PartialEq)]
pub struct Enum {
    pub name: String,
    // TODO variants
}

#[derive(Debug, PartialEq)]
pub struct Assignment {
    pub name: String,
    pub type_annotation: Option<String>,
    pub expression: Expr,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Lit(Lit),
    Id(String),
    Assign(Box<Assignment>),
    FnCall(Box<FnCall>),
    Arith(Box<ArithExpr>),
    Cmp(Box<CmpExpr>),
    Logic(Box<LogicExpr>),
    Paren(Box<Expr>),
    Unary(Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Lit {
    Bool(bool),
    Int(i64),
    Float(f64),
    Char(char),
    String(String),
}

#[derive(Debug, PartialEq)]
pub enum Unary {
    Not(Box<Expr>),
    Minus(Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub struct Term(pub Expr);

#[derive(Debug, PartialEq)]
pub struct ArithExpr {
    pub left: Expr,
    pub op: ArithOp,
    pub right: Expr,
}

#[derive(Debug, PartialEq)]
pub struct CmpExpr {
    pub left: Expr,
    pub op: CmpOp,
    pub right: Expr,
}

#[derive(Debug, PartialEq)]
pub struct LogicExpr {
    pub left: Expr,
    pub op: LogicOp,
    pub right: Expr,
}

#[derive(Debug, PartialEq)]
pub enum ArithOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
}

#[derive(Debug, PartialEq)]
pub enum LogicOp {
    Or,
    And,
}

#[derive(Debug, PartialEq)]
pub enum CmpOp {
    EQ,
    NE,
    LT,
    LE,
    GT,
    GE,
}

#[derive(Debug, PartialEq)]
pub struct FnCall {
    pub callee: Expr,
    pub params: Vec<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionArgument {
    pub name: String,
    pub type_: String,
}
