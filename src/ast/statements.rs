use super::*;

// A statement.
#[derive(Debug)]
pub enum Statement<'a> {
    Decl(Decl<'a>),
    If(If<'a>),
    For(For<'a>),
    While(While<'a>),
    Query(Query<'a>),
    Switch(Switch<'a>),
    Block(Block<'a>),
    Expr(Expr<'a>),
    Break,
    Continue,
    Return(Option<Expr<'a>>)
}

// A variable declaration.
#[derive(Debug)]
pub struct Decl<'a> {
    pub ident: &'a str,
    pub init: Option<Expr<'a>>,
}

// An if block.
#[derive(Debug)]
pub struct If<'a> {
    pub cond: Expr<'a>,
    pub branch1: Block<'a>,
    pub branch2: Option<Block<'a>>,
}

// A for loop.
#[derive(Debug)]
pub struct For<'a> {
    pub init: Either<Expr<'a>, Decl<'a>>,
    pub cond: Expr<'a>,
    pub incr: Expr<'a>,
    pub code: Block<'a>,
}

// A while loop.
#[derive(Debug)]
pub struct While<'a> {
    pub cond: Expr<'a>,
    pub code: Block<'a>,
}

// A query loop.
#[derive(Debug)]
pub struct Query<'a> {
    pub filters: Vec<Filter<'a>>,
    pub code: Block<'a>,
}

// A switch block.
#[derive(Debug)]
pub struct Switch<'a> {
    pub expr: Expr<'a>,
    pub cases: Vec<SwitchCase<'a>>,
    pub default: Block<'a>,
}

// A switch case.
#[derive(Debug)]
pub struct SwitchCase<'a> {
    pub val: Atom,
    pub block: Block<'a>,
}

// A code block.
#[derive(Default, Debug)]
pub struct Block<'a> {
    pub statements: Vec<Statement<'a>>,
}