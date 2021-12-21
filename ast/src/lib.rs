#[derive(Debug, PartialEq, Clone)]
pub struct CacauProgram {
    pub items: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Fn(Function),
    Struct(),
    Enum(),
    Expr(Expr),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub public: bool,
    pub name: String,
    pub params: Vec<FunctionArgument>,
    pub output: String,
    pub body: Expr,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Struct {
    pub name: String,
    // TODO fields and their types
}

#[derive(Debug, PartialEq, Clone)]
pub struct Enum {
    pub name: String,
    // TODO variants
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDecl {
    pub name: String,
    pub type_annotation: Option<String>,
    pub expr: Expr,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Lit(Lit),
    Id(String),
    VarDecl(Box<VariableDecl>),
    FnCall(Box<FnCall>),
    Arith(Box<ArithExpr>),
    Cmp(Box<CmpExpr>),
    Logic(Box<LogicExpr>),
    Paren(Box<Expr>),
    Unary(Box<Unary>),
    Assign(Box<Assign>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Assign {
    pub destination: Expr,
    pub operator: Option<ArithOp>,
    pub value: Expr,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Lit {
    Bool(bool),
    Int(i64),
    Float(f64),
    Char(char),
    String(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Unary {
    Not(Expr),
    Minus(Expr),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Term(pub Expr);

#[derive(Debug, PartialEq, Clone)]
pub struct ArithExpr {
    pub left: Expr,
    pub op: ArithOp,
    pub right: Expr,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CmpExpr {
    pub left: Expr,
    pub op: CmpOp,
    pub right: Expr,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LogicExpr {
    pub left: Expr,
    pub op: LogicOp,
    pub right: Expr,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ArithOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LogicOp {
    Or,
    And,
    Xor,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CmpOp {
    EQ,
    NE,
    LT,
    LE,
    GT,
    GE,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FnCall {
    pub callee: Expr,
    pub params: Vec<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionArgument {
    pub name: String,
    pub type_: String,
}
