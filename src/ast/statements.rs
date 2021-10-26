use super::*;

/// A statement.
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

/// An if block.
pub struct If<'a> {
    pub cond: Expr<'a>,
    pub branch1: Block<'a>,
    pub branch2: Option<Block<'a>>,
}

/// A for loop.
pub struct For<'a> {
    pub init: Either<Expr<'a>, Declaration<'a>>,
    pub cond: Expr<'a>,
    pub incr: Expr<'a>,
    pub code: Block<'a>,
}

/// A while loop.
pub struct While<'a> {
    pub cond: Expr<'a>,
    pub code: Block<'a>,
}

/// A query loop.
pub struct Query<'a> {
    pub filters: Vec<Filter<'a>>,
    pub code: Block<'a>,
}

/// A code block.
pub struct Block<'a> {
    pub statements: Vec<Statement<'a>>,
}

/// A declaration.
pub struct Declaration<'a> {
    pub is_const: bool,
    pub ty: Type<'a>,
    pub name: &'a str,
    pub expr: Option<Expr<'a>>,
}