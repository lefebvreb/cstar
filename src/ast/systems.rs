use super::*;

/// A system.
pub struct System<'a> {
    filters: Vec<Filter<'a>>,
    code: Block<'a>,
}

/// A single Filter.
pub enum Filter<'a> {
    Entity(&'a str, Vec<Argument<'a>>),
    Resource(Argument<'a>),
}

/// An argument to a System or function signature.
pub struct Argument<'a> {
    is_const: bool,
    ty: Type<'a>,
    name: &'a str,
}