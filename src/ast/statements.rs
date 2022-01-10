use super::*;

/// A statement.
#[derive(Debug)]
pub enum Statement<'a> {
    Assign(Assign<'a>),
    If(If<'a>),
    For(For<'a>),
    While(While<'a>),
    Query(Query<'a>),
    Block(Block<'a>),
    Expr(Expr<'a>),
    Break,
    Continue,
}

/// An if block.
#[derive(Debug)]
pub struct If<'a> {
    pub cond: Expr<'a>,
    pub branch1: Block<'a>,
    pub branch2: Option<Block<'a>>,
}

/// A for loop.
#[derive(Default, Debug)]
pub struct For<'a> {
    pub init: Option<Assign<'a>>,
    pub cond: Option<Expr<'a>>,
    pub incr: Option<Assign<'a>>,
    pub code: Block<'a>,
}

/// A while loop.
#[derive(Debug)]
pub struct While<'a> {
    pub cond: Expr<'a>,
    pub code: Block<'a>,
}

/// A query loop.
#[derive(Debug)]
pub struct Query<'a> {
    pub filters: Vec<Filter<'a>>,
    pub code: Block<'a>,
}

/// A code block.
#[derive(Default, Debug)]
pub struct Block<'a> {
    pub statements: Vec<Statement<'a>>,
}