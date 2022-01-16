use super::*;

// A statement.
#[derive(Debug)]
pub enum Statement {
    Decl(Decl),
    If(If),
    For(For),
    While(While),
    Query(Query),
    Switch(Switch),
    Block(Block),
    Expr(Expr),
    Break,
    Continue,
    Return(Option<Expr>)
}

// A variable declaration.
#[derive(Debug)]
pub struct Decl {
    pub ident: &'static str,
    pub init: Option<Expr>,
}

// An if block.
#[derive(Debug)]
pub struct If {
    pub cond: Expr,
    pub branch1: Block,
    pub branch2: Option<Block>,
}

// A for loop.
#[derive(Debug)]
pub struct For {
    pub init: Either<Expr, Decl>,
    pub cond: Expr,
    pub incr: Expr,
    pub code: Block,
}

// A while loop.
#[derive(Debug)]
pub struct While {
    pub cond: Expr,
    pub code: Block,
}

// A query loop.
#[derive(Debug)]
pub struct Query {
    pub filters: Vec<Filter>,
    pub code: Block,
}

// A switch block.
#[derive(Debug)]
pub struct Switch {
    pub expr: Expr,
    pub cases: Vec<SwitchCase>,
    pub default: Block,
}

// A switch case.
#[derive(Debug)]
pub struct SwitchCase {
    pub val: Atom,
    pub block: Block,
}

// A code block.
#[derive(Default, Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
}