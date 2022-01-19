use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

use crate::ast::EntityFilter;

use super::*;

// For EntityFilter, hash and equality are done on references.
impl Hash for ast::EntityFilter {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize((self as *const Self) as usize);
    }
}
impl PartialEq for ast::EntityFilter {
    fn eq(&self, other: &Self) -> bool {
        self as *const Self == other as *const Self
    }
}

// An entity ID.
#[derive(Clone, Hash, PartialEq, Debug)]
pub struct Entity(pub usize);

#[derive(Debug)]
pub struct World {
    defs: &'static Map<Def>,
    // Map from resurces names to resources values.
    resources: Map<Var>,
    // Map from entities ID to their components.
    entities: HashMap<Entity, Map<Var>>, 
    // Map from EntityFilters to the components they refer to.
    filters: HashMap<EntityFilter, Vec<&'static str>>,
    // Map from EntityFilters to the entities that match them.
    matches: HashMap<EntityFilter, HashSet<Entity>>,
}

impl World {
    pub fn new(defs: &'static Map<Def>) -> World {
        todo!()
    }

    pub fn new_entity(&mut self, components: Vec<Var>) -> Result<Entity> {
        todo!()
    }

    pub fn delete_entity(&mut self, entity: Entity) -> Result<()> {
        todo!()
    }

    pub fn new_resource(&mut self, val: Var) -> Result<()> {
        todo!()
    }

    /*pub fn filter_entities(&self, filter: &EntityFilter) -> impl Iterator<Item = Entity> {
        todo!()
    }*/
}

// Need to finish:
// - Printing entites in Var::fmt()
// - Cloning, Deleting, Spawning entites in eval_call()
// - Filtering and looping in eval_query(), eval_system()