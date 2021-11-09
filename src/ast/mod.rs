use crate::utils::*;

mod expressions;
pub use expressions::*;

mod statements;
pub use statements::*;

mod systems;
pub use systems::*;

mod types;
pub use types::*;

/// The main AST struct, representing a program.
#[derive(Default, Debug)]
pub struct AST<'a> {
    pub names: Map<'a, Name<'a>>,
    pub init: Vec<&'a str>,
    pub run: Vec<&'a str>,
}

// A name in the global namespace.
#[derive(Debug)]
pub enum Name<'a> {
    Static(Static<'a>),
    System(System<'a>),
    Component(StructDef<'a>),
    Resource(StructDef<'a>),
}

#[derive(Debug)]
pub struct Static<'a> {
    pub ty: Type<'a>,
    pub value: Expr<'a>,
}