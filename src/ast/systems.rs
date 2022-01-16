use super::*;

// A system.
#[derive(Debug)]
pub struct System {
    pub filters: Vec<Filter>,
    pub code: Block,
}

// A single Filter.
#[derive(Debug)]
pub enum Filter {
    Entity(EntityFilter),
    Resource(Argument),
}

// An entity filter.
#[derive(Debug)]
pub struct EntityFilter {
    pub name: &'static str,
    pub args: Vec<Argument>,
}

// An argument to a System or function signature.
#[derive(Debug)]
pub struct Argument {
    pub ty: &'static str,
    pub name: &'static str,
}