use std::collections::HashMap;

#[derive(Clone, PartialEq, Hash, Debug)]
pub struct Entity(u32);

pub struct World {
    entities: HashMap<Entity, Vec<()>>, 
}

// Need to finish:
// - Printing entites in Var::Display
// - Cloning, Deleting, Spawnin entites in eval::call
// - Filtering in eval_query, eval_system