use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::ast::EntityFilter;

use super::*;

// =============================================================== Command

// A command to be executed by the world.
#[derive(Debug)]
pub enum Command {
    SpawnEntity(Vec<Var>),
    DeleteEntity(Entity),
    NewResource(Var),
}

// =============================================================== EntityFilter impl

// For EntityFilter, hashes are just the references hashed as integers.
impl Hash for ast::EntityFilter {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize((self as *const Self) as usize);
    }
}

// For EntityFilter, compares are just the references compared as integers.
impl PartialEq for ast::EntityFilter {
    fn eq(&self, other: &Self) -> bool {
        self as *const Self == other as *const Self
    }
}

// Needed for hashmaps.
impl Eq for ast::EntityFilter {}

impl fmt::Display for EntityFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(arg) = self.args.first() {
            write!(f, "{}", arg.ty)?;
        }
        for arg in &self.args[1..] {
            write!(f, ", {}", arg.ty)?;
        }
        Ok(())
    }
}

// =============================================================== Entity

// An entity ID.
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Entity(pub u64);

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Entity({})", self.0)
    }
}

// =============================================================== World

#[derive(Debug)]
pub struct World {
    defs: &'static Map<Def>,
    // Counter for entites ids.
    counter: u64,
    // Map from resurces names to resources values.
    resources: Map<Var>,
    // Map from entities ID to their components.
    entities: HashMap<Entity, Map<Var>>,
    // Map from EntityFilters to the entities that match them.
    matches: HashMap<&'static EntityFilter, HashSet<Entity>>,
    // Map from entities ID to the filter they match.
    filters: HashMap<Entity, Vec<&'static EntityFilter>>,
}

// Public API.
impl World {
    // Creates a new world.
    pub fn new(defs: &'static Map<Def>) -> World {
        World {
            defs,
            counter: 0,
            resources: Map::default(),
            entities: HashMap::default(),
            matches: HashMap::default(),
            filters: HashMap::default(),
        }
    }

    // Gets the resource.
    pub fn get_resource(&self, name: &'static str) -> Result<Var> {
        self.resources.get(name).cloned().ok_or_else(|| anyhow!("Resource {} not found", name))
    }

    // Gets the named component of the given entity.
    pub fn get_component(&self, entity: Entity, name: &str) -> Result<Var> {
        self.entities.get(&entity)
            .ok_or_else(|| anyhow!("Entity {} not found.", entity))?
            .get(name).cloned()
            .ok_or_else(|| anyhow!("Component {} not found for {}.", name, entity))
    }

    // Executes the commands provided in the given vector of commands.
    pub fn do_commands(&mut self, commands: &mut Vec<Command>) -> Result<()> {
        for cmd in commands.drain(..) {
            match cmd {
                Command::SpawnEntity(components) => self.spawn_entity(components)?,
                Command::DeleteEntity(entity) => self.delete_entity(entity)?,
                Command::NewResource(res) => self.new_resource(res)?,
            }
        }

        Ok(())
    }

    // Filter entites by components they should hold. Returns an iterator over the entities that matches the filter.
    pub fn filter_entities(&mut self, filter: &'static EntityFilter, iter: &mut Vec<Entity>) -> Result<()> {
        // Empties the iterator.
        iter.clear();
        
        // Successful cache match.
        if let Some(matches) = self.matches.get(filter) {
            iter.extend(matches.iter().cloned());
            return Ok(());
        }

        // Check if the filter contains only components
        if filter.args.iter().any(|arg| !self.is_component(arg.ty)) {
            return Err(anyhow!("Filter contains non-component types."));
        }

        // It's a new filter, so we need to compute the entities it includes.
        let matches = self.entities.keys()
            .filter_map(|entity| self.matches(filter, entity).then(|| entity.clone()))
            .collect::<HashSet<_>>();
        
        // Set resulsts.
        iter.extend(matches.iter().cloned());

        // Cache the matches.
        for entity in &matches {
            self.filters.get_mut(&entity).unwrap().push(filter);
        }
        self.matches.insert(filter, matches);

        Ok(())
    }
}

// Private methods.
impl World {
    // Returns true if the given name refers to a component.
    fn is_component(&self, name: &str) -> bool {
        matches!(self.defs.get(name), Some(Def::Component(_)))
    }

    // Returns true if the given name refers to a resource.
    fn is_resource(&self, name: &str) -> bool {
        matches!(self.defs.get(name), Some(Def::Resource(_)))
    }

    // Returns true if the given entity matches the filter.
    fn matches(&self, filter: &EntityFilter, entity: &Entity) -> bool {
        let components = self.entities.get(&entity).unwrap();
        filter.args.iter().all(|arg| components.contains_key(arg.ty))
    }

    // Spawn the entity with the given components.
    fn spawn_entity(&mut self, components: Vec<Var>) -> Result<()> {
        // Check if the components are valid.
        let mut map = Map::default();

        for component in components {
            let name = component.struct_type()?;
            if !self.is_component(name) {
                return Err(anyhow!("Struct {} is not a component.", name));
            }
            map.insert(name, component);
        }

        // Add the entity to entities.
        let entity = Entity(self.counter);
        self.counter += 1;

        self.entities.insert(entity.clone(), map);

        // Update matches cache as well as filters cache.
        let matches = self.matches.keys()
            .filter(|filter| self.matches(filter, &entity))
            .map(|&filter| filter)
            .collect::<Vec<_>>();

        for &filter in &matches {
            self.matches.get_mut(filter).unwrap().insert(entity.clone());
        }
        self.filters.insert(entity, matches);

        Ok(())
    }

    // Delete the entity with the given ID.
    fn delete_entity(&mut self, entity: Entity) -> Result<()> {
        // Remove the entity from entities.
        self.entities.remove(&entity).ok_or_else(|| anyhow!("Entity {} not found.", entity))?;

        // Update matches cache and remove the corresponding entry in filters.
        let matches = self.filters.remove(&entity).unwrap();

        for &filter in &matches {
            self.matches.get_mut(filter).unwrap().remove(&entity);
        }

        Ok(())
    }

    // Creates a new resource with the given variable.
    fn new_resource(&mut self, res: Var) -> Result<()> {
        let name = res.struct_type()?;

        if !self.is_resource(name) {
            return Err(anyhow!("{} is not a resource.", name));
        }

        if self.resources.insert(name, res).is_some() {
            return Err(anyhow!("Resource {} already exists.", name));
        }
    
        Ok(())
    }
}

// Need to finish:
// - Printing entites in Var::fmt()
// - Cloning, Deleting, Spawning entites in eval_call()
// - Filtering and looping in eval_query(), eval_system()