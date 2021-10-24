use super::*;

pub enum Statement<'a> {
    If(If<'a>),
    For(For<'a>),
    While(While<'a>),
    Query(Query<'a>),
    Block(Block<'a>),
}

pub struct If<'a> {
    condition: Expression<'a>,
    branch1: Block<'a>,
    branch2: Option<Block<'a>>,
}

pub struct Block<'a> {
    statements: Vec<Statement<'a>>,
}