use pest::prec_climber::Operator;

use super::*;

// An expression.
#[derive(Debug)]
pub enum Expr<'a> {
    Assign(Box<Assign<'a>>),
    Ternary(Box<Ternary<'a>>),
    Atom(Atom),
    LValue(LValue<'a>),
    ListInit(ListInit<'a>),
    StructInit(StructInit<'a>),
    Call(Call<'a>),
    BinExpr(Box<BinExpr<'a>>),
    UnExpr(Box<UnExpr<'a>>),
}

// A left-value, that can be assigned to.
#[derive(Default, Debug)]
pub struct Index<'a> {
    pub exprs: Vec<Expr<'a>>,
}

// A left-value, that can be assigned to.
#[derive(Debug)]
pub struct LValue<'a> {
    pub name: &'a str,
    pub first_index: Index<'a>,
    pub path: Vec<(&'a str, Index<'a>)>,
}

// A ternary expression.
#[derive(Debug)]
pub struct Ternary<'a> {
    pub cond: Expr<'a>,
    pub branch1: Expr<'a>,
    pub branch2: Expr<'a>,
}

// An assign expression.
#[derive(Debug)]
pub struct Assign<'a> {
    pub lvalue: LValue<'a>,
    pub expr: Expr<'a>,
}

// A list initialization.
#[derive(Debug)]
pub struct ListInit<'a> {
    pub exprs: Vec<Expr<'a>>,
}


// A struct initialization.
#[derive(Debug)]
pub struct StructInit<'a> {
    pub name: &'a str,
    pub fields: Vec<(&'a str, Expr<'a>)>,
}

// The atomic value of a primitive.
#[derive(Debug)]
pub enum Atom {
    Void,
    Bool(bool),
    Int(i64),
    Float(f64),
    Char(char),
    String(String),
}

// A call expression. Can only call builtins for now.
#[derive(Debug)]
pub struct Call<'a> {
    pub name: &'a str,
    pub args: Vec<Expr<'a>>,
}

// A builtin function name.
#[derive(Debug)]
pub enum BuiltIn {
    Clone,
    Spawn,
    Delete,
    Println,
    Print,
}

// A binary expression.
#[derive(Debug)]
pub struct BinExpr<'a> {
    pub left: Expr<'a>,
    pub op: BinOp,
    pub right: Expr<'a>,
}

// A binary operator.
#[derive(Copy, Clone, Debug)]
pub enum BinOp {
    Add, Sub, Mul, Div, Mod,
    And, Or, Xor, BitAnd, BitOr,
    Shl, Shr, Leq, Geq, Lt,
    Gt, Eq, Neq,
}

// An unary expression.
#[derive(Debug)]
pub struct UnExpr<'a> {
    pub op: UnOp,
    pub expr: Expr<'a>,
}

// An unary operator.
#[derive(Copy, Clone, Debug)]
pub enum UnOp {
    Pos, Neg, Not, BitNot,
}