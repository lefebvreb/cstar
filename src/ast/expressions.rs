use pest::prec_climber::Operator;

pub enum Expr<'a> {
    Atom(Atom),
    LValue(LValue<'a>),
    StructInit(StructInit<'a>),
    Call(Box<Call<'a>>),
    BinExpr(Box<BinExpr<'a>>),
    UnExpr(Box<UnExpr<'a>>),
    Assign(Box<Assign<'a>>),
}

pub enum LValue<'a> {
    Ident(&'a str),
    Access(Vec<&'a str>),
}

pub struct Assign<'a> {
    ident: &'a str,
    expr: Expr<'a>,
}

pub struct StructInit<'a> {
    ty: &'a str,
    fields: Vec<(&'a str, Expr<'a>)>,
}

pub enum Atom {
    Void,
    Bool(bool),
    Int(i64),
    Float(f32),
    Char(char),
    String(String),
}

pub struct Call<'a> {
    builtin: BuiltIn,
    args: Vec<Expr<'a>>,
}

pub enum BuiltIn {
    Clone,
    Spawn,
    Delete,
    Print,
}

pub struct BinExpr<'a> {
    left: Expr<'a>,
    op: BinOp,
    right: Expr<'a>,
}

pub enum BinOp {
    Add, Sub, Mul, Div, Mod,
    And, Or, Xor, BitAnd, BitOr,
    Shl, Shr, Leq, Geq, Lt,
    Gt, Eq, Neq,
}

pub struct UnExpr<'a> {
    expr: Expr<'a>,
    op: UnOp,
}

pub enum UnOp {
    Pos, Neg, Not, BitNot,
}