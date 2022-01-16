use crate::utils::*;

mod expressions;
pub use expressions::*;

mod statements;
pub use statements::*;

mod systems;
pub use systems::*;

mod types;
pub use types::*;

// The main AST struct, representing a program.
#[derive(Default, Debug)]
pub struct AST {
    pub names: Map<Name>,
    pub init: Vec<&'static str>,
    pub run: Vec<&'static str>,
}

// A name in the global namespace.
#[derive(Debug)]
pub enum Name {
    Function(Function),
    System(System),
    Component(StructDef),
    Resource(StructDef),
    Struct(StructDef),
}

#[derive(Debug)]
pub struct Function {
    pub args: Vec<&'static str>,
    pub body: Block,
}