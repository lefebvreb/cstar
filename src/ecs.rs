use std::collections::HashMap;

#[derive(Clone, PartialEq, Hash, Debug)]
pub struct Entity(u32);

pub struct World {
    entities: HashMap<Entity, Vec<()>>, 
}

// Need to finish:
// - Printing entites in Var::fmt()
// - Cloning, Deleting, Spawning entites in eval_call()
// - Filtering and looping in eval_query(), eval_system()