use std::collections::HashMap;

#[derive(Clone, PartialEq, Hash, Debug)]
pub struct Entity(u32);

pub struct World {
    entities: HashMap<Entity, Vec<()>>, 
}