use std::collections::HashMap;

mod statements;
pub use statements::*;

mod systems;
pub use systems::*;

mod types;
pub use types::*;

pub type Map<'a, T> = HashMap<&'a str, T>;

pub struct AST<'a> {
    types: Map<'a, Type<'a>>,
    systems: Map<'a, System<'a>>, 
    init: Vec<&'a str>,
    run: Vec<&'a str>,
}