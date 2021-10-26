use pest::prec_climber::Operator;

/// An expression.
pub enum Expr<'a> {
    Atom(Atom),
    LValue(LValue<'a>),
    StructInit(StructInit<'a>),
    Call(Box<Call<'a>>),
    BinExpr(Box<BinExpr<'a>>),
    UnExpr(Box<UnExpr<'a>>),
    Assign(Box<Assign<'a>>),
}

/// A left-value, that can be assigned to.
pub enum LValue<'a> {
    Ident(&'a str),
    Access(Vec<&'a str>),
}

/// An assign expression.
pub struct Assign<'a> {
    pub lvalue: LValue<'a>,
    pub expr: Expr<'a>,
}

/// A struct initialization.
pub struct StructInit<'a> {
    pub ty: &'a str,
    pub fields: Vec<(&'a str, Expr<'a>)>,
}

/// The atomic value of a primitive.
pub enum Atom {
    Void,
    Bool(bool),
    Int(i64),
    Float(f32),
    Char(char),
    String(String),
}

/// A call expression. Can only call builtins for now.
pub struct Call<'a> {
    pub builtin: BuiltIn,
    pub args: Vec<Expr<'a>>,
}

/// A builtin function name.
pub enum BuiltIn {
    Clone,
    Spawn,
    Delete,
    Print,
}

/// A binary expression.
pub struct BinExpr<'a> {
    pub left: Expr<'a>,
    pub op: BinOp,
    pub right: Expr<'a>,
}

/// A binary operator.
pub enum BinOp {
    Add, Sub, Mul, Div, Mod,
    And, Or, Xor, BitAnd, BitOr,
    Shl, Shr, Leq, Geq, Lt,
    Gt, Eq, Neq,
}

/// An unary expression.
pub struct UnExpr<'a> {
    pub expr: Expr<'a>,
    pub op: UnOp,
}

/// An unary operator.
pub enum UnOp {
    Pos, Neg, Not, BitNot,
}