use super::*;

/// A system.
pub struct System<'a> {
    pub filters: Vec<Filter<'a>>,
    pub code: Block<'a>,
}

/// A single Filter.
pub enum Filter<'a> {
    Entity(&'a str, Vec<Argument<'a>>),
    Resource(Argument<'a>),
}

/// An argument to a System or function signature.
pub struct Argument<'a> {
    pub is_const: bool,
    pub ty: Type<'a>,
    pub name: &'a str,
}