use super::*;

// A system.
#[derive(Debug)]
pub struct System<'a> {
    pub filters: Vec<Filter<'a>>,
    pub code: Block<'a>,
}

// A single Filter.
#[derive(Debug)]
pub enum Filter<'a> {
    Entity(EntityFilter<'a>),
    Resource(Argument<'a>),
}

// An entity filter.
#[derive(Debug)]
pub struct EntityFilter<'a> {
    pub name: &'a str,
    pub args: Vec<Argument<'a>>,
}

// An argument to a System or function signature.
#[derive(Debug)]
pub struct Argument<'a> {
    pub ty: &'a str,
    pub name: &'a str,
}