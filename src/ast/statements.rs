use super::*;

pub enum Statement<'a> {
    If(If<'a>),
    For(For<'a>),
    While(While<'a>),
    Query(Query<'a>),
    Block(Block<'a>),
    Expr(Expr<'a>),
    Decl(Declaration<'a>),
    Break,
    Continue,
}

pub struct If<'a> {
    cond: Expr<'a>,
    branch1: Block<'a>,
    branch2: Option<Block<'a>>,
}

pub struct For<'a> {
    init: Either<Expr<'a>, Declaration<'a>>,
    cond: Expr<'a>,
    incr: Expr<'a>,
    code: Block<'a>,
}

pub struct While<'a> {
    cond: Expr<'a>,
    code: Block<'a>,
}

pub struct Query<'a> {
    filters: Vec<Filter<'a>>,
    code: Block<'a>,
}

pub struct Block<'a> {
    statements: Vec<Statement<'a>>,
}

pub struct Declaration<'a> {
    is_const: bool,
    ty: Type<'a>,
    name: &'a str,
    expr: Option<Expr<'a>>,
}