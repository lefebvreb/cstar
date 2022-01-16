use pest::prec_climber::Operator;

use super::*;

// An expression.
#[derive(Debug)]
pub enum Expr {
    Assign(Box<Assign>),
    Ternary(Box<Ternary>),
    Atom(Atom),
    LValue(LValue),
    ListInit(ListInit),
    StructInit(StructInit),
    Call(Call),
    BinExpr(Box<BinExpr>),
    UnExpr(Box<UnExpr>),
}

// A left-value, that can be assigned to.
#[derive(Default, Debug)]
pub struct Index {
    pub exprs: Vec<Expr>,
}

// A left-value, that can be assigned to.
#[derive(Debug)]
pub struct LValue {
    pub name: &'static str,
    pub first_index: Index,
    pub path: Vec<(&'static str, Index)>,
}

// A ternary expression.
#[derive(Debug)]
pub struct Ternary {
    pub cond: Expr,
    pub branch1: Expr,
    pub branch2: Expr,
}

// An assign expression.
#[derive(Debug)]
pub struct Assign {
    pub lvalue: LValue,
    pub expr: Expr,
}

// A list initialization.
#[derive(Debug)]
pub struct ListInit {
    pub exprs: Vec<Expr>,
}


// A struct initialization.
#[derive(Debug)]
pub struct StructInit {
    pub name: &'static str,
    pub fields: Vec<(&'static str, Expr)>,
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
pub struct Call {
    pub name: &'static str,
    pub args: Vec<Expr>,
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
pub struct BinExpr {
    pub left: Expr,
    pub op: BinOp,
    pub right: Expr,
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
pub struct UnExpr {
    pub op: UnOp,
    pub expr: Expr,
}

// An unary operator.
#[derive(Copy, Clone, Debug)]
pub enum UnOp {
    Pos, Neg, Not, BitNot,
}