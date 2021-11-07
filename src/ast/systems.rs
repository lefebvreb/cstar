use super::*;

/// A system.
#[derive(Debug)]
pub struct System<'a> {
    pub filters: Vec<Filter<'a>>,
    pub code: Block<'a>,
}

/// A single Filter.
#[derive(Debug)]
pub enum Filter<'a> {
    Entity(&'a str, Vec<Argument<'a>>),
    Resource(Argument<'a>),
}

/// An argument to a System or function signature.
#[derive(Debug)]
pub struct Argument<'a> {
    pub is_const: bool,
    pub ty: Type<'a>,
    pub name: &'a str,
}