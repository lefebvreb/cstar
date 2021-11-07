use pest::prec_climber::Operator;

use super::*;

/// An expression.
#[derive(Debug)]
pub enum Expr<'a> {
    Atom(Atom),
    LValue(LValue<'a>),
    StructInit(StructInit<'a>),
    Cast(Box<Cast<'a>>),
    Call(Call<'a>),
    BinExpr(Box<BinExpr<'a>>),
    UnExpr(Box<UnExpr<'a>>),
    Assign(Box<Assign<'a>>),
}

/// A left-value, that can be assigned to.
#[derive(Debug)]
pub enum LValue<'a> {
    Ident(&'a str),
    Access(Vec<&'a str>),
}

/// An assign expression.
#[derive(Debug)]
pub struct Assign<'a> {
    pub lvalue: LValue<'a>,
    pub expr: Expr<'a>,
}

/// A struct initialization.
#[derive(Debug)]
pub struct StructInit<'a> {
    pub name: &'a str,
    pub fields: Vec<(&'a str, Expr<'a>)>,
}

/// The atomic value of a primitive.
#[derive(Debug)]
pub enum Atom {
    Void,
    Bool(bool),
    Int(i64),
    Float(f64),
    Char(char),
    String(String),
}

/// A call expression. Can only call builtins for now.
#[derive(Debug)]
pub struct Call<'a> {
    pub builtin: BuiltIn,
    pub args: Vec<Expr<'a>>,
}

/// A cast expression.
#[derive(Debug)]
pub struct Cast<'a> {
    pub ty: Type<'a>,
    pub expr: Expr<'a>,
}

/// A builtin function name.
#[derive(Debug)]
pub enum BuiltIn {
    Clone,
    Spawn,
    Delete,
    Print,
}

/// A binary expression.
#[derive(Debug)]
pub struct BinExpr<'a> {
    pub left: Expr<'a>,
    pub op: BinOp,
    pub right: Expr<'a>,
}

/// A binary operator.
#[derive(Debug)]
pub enum BinOp {
    Add, Sub, Mul, Div, Mod,
    And, Or, Xor, BitAnd, BitOr,
    Shl, Shr, Leq, Geq, Lt,
    Gt, Eq, Neq,
}

/// An unary expression.
#[derive(Debug)]
pub struct UnExpr<'a> {
    pub op: UnOp,
    pub expr: Expr<'a>,
}

/// An unary operator.
#[derive(Debug)]
pub enum UnOp {
    Pos, Neg, Not, BitNot,
}