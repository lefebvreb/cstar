use std::collections::HashMap;

mod expressions;
pub use expressions::*;

mod statements;
pub use statements::*;

mod systems;
pub use systems::*;

mod types;
pub use types::*;

/// Either a T or an U value.
pub enum Either<T, U> {
    Left(T),
    Right(U),
}

/// Shorter name for a HashMap with &str as keys.
pub type Map<'a, T> = HashMap<&'a str, T>;

/// The main AST struct, representing a program.
pub struct AST<'a> {
    types: Map<'a, Type<'a>>,
    systems: Map<'a, System<'a>>, 
    init: Vec<&'a str>,
    run: Vec<&'a str>,
}