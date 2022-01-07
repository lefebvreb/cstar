use anyhow::{anyhow, Error, Result};

use crate::ast;
use crate::ecs;
use crate::utils::*;

/// Holds definitions of function-like and struct-like objects.
pub struct Context<'a> {
    defs: Map<'a, Def<'a>>,
}

impl<'a> Context<'a> {
    /// Defines a new object in the context, retunrns an error if an
    /// object with the same name already exists.
    pub fn set_def(&mut self, name: &'a str, def: Def<'a>) -> Result<()> {
        self.defs.insert(name, def)
            .is_none().then(|| ())
            .ok_or_else(|| anyhow!("object with name '{}' already exists", name))
    }

    /// Returns the definition corresponding to the given name, or an
    /// error if no such definition exists.
    pub fn get_def(&self, name: &str) -> Result<Def> {
        self.defs.get(name)
            .ok_or_else(|| anyhow!("Definition {} does not exist", name))
            .map(Def::clone)
    }
}

impl Default for Context<'_> {
    /// Creates a new empty context object.
    fn default() -> Self {
        Self {
            defs: Map::new(),  
        }
    }
}

/// Holds all variables scopes.
pub struct Scope<'a> {
    vars: Vec<Map<'a, Var<'a>>>,
}

impl<'a> Scope<'a> {
    /// Nests another new empty scope.
    pub fn next(&mut self) {
        self.vars.push(Map::new());
    }

    /// Destroys the last created scope, freeing all of it's variables.
    pub fn back(&mut self) {
        self.vars.pop();
    }

    /// Adds a new variable to the current scope.
    pub fn set_var(&mut self, name: &'a str, var: Var<'a>) {
        self.vars.last_mut().unwrap().insert(name, var);
    }

    /// Gets a reference to a variable or constant from the current scope.
    pub fn get_var(&self, name: &str) -> Result<Var> {
        self.vars.iter().rev()
            .find_map(|scope| scope.get(name))
            .ok_or_else(|| anyhow!("Variable {} does not exist", name))
            .map(Var::clone)
    }

    /// Sets the value of a struct's field.
    pub fn update_path(&mut self, path: Vec<&'a str>, var: Var<'a>) -> Result<()> {
        if path.is_empty() {
            return Err(anyhow!("Path is empty"));
        }

        // Get struct in global space
        let name = path.first().unwrap();
        let mut map = self.vars.iter_mut().rev()
            .find(|scope| scope.get(name).is_some())
            .ok_or_else(|| anyhow!("No such struct {}", path.first().unwrap()))?;

        // Follow the path.
        for &name in path[..path.len()-1].iter() {
            // Get the entry corresponding to the next name.
            let entry = map.get_mut(name)
                .ok_or_else(|| anyhow!("Variable {} does not exist", name))?;

            // If the entry is a struct, follow the path.
            map = match entry {
                Var::Struct {val, ..} => val,
                _ => return Err(anyhow!("{} is not a struct", name)),
            };
        }

        let entry = map.get_mut(path.last().unwrap())
            .ok_or_else(|| anyhow!("Variable {} does not exist", name))?;
        
        *entry = var;

        Ok(())
    }
}

impl Default for Scope<'_> {
    /// Creates a new empty scope object.
    fn default() -> Self {
        Self {
            vars: vec![Map::new()],
        }
    }
}

/// A variable's value.
#[derive(Clone, Debug)]
pub enum Var<'a> {
    Void,
    Bool(bool),
    Int(i64),
    Float(f64),
    Char(char),
    String(String),
    Entity(ecs::Entity),
    System(&'a ast::System<'a>),
    Struct {
        typ: Type<'a>,
        val: Map<'a, Var<'a>>,
    },
}

impl<'a> Var<'a> {
    /// Gets the type of this variable
    pub fn get_type(&self) -> Type<'a> {
        match self {
            Var::Void => Type::Void,
            Var::Bool(_) => Type::Bool,
            Var::Int(_) => Type::Int,
            Var::Float(_) => Type::Float,
            Var::Char(_) => Type::Char,
            Var::String(_) => Type::String,
            Var::Entity(_) => Type::Entity,
            Var::System(_) => Type::System,
            Var::Struct { typ, .. } => typ.clone(),
        }
    }
}

/// A definition of a struct-like or function-like object.
#[derive(Clone, Debug)]
pub enum Def<'a> {
    System(&'a ast::System<'a>),
    Component(&'a ast::StructDef<'a>),
    Resource(&'a ast::StructDef<'a>),
}

/// The type of a variable.
#[derive(Clone, Debug)]
pub enum Type<'a> {
    Void,
    Bool,
    Int,
    Float,
    Char,
    String,
    Entity,
    System,
    Component(&'a ast::StructDef<'a>),
    Resource(&'a ast::StructDef<'a>),
}

impl PartialEq for Type<'_> {
    /// Tests for type equality. Struct-like types are only
    /// equal to themselves.
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Type::Component(a), Type::Component(b)) | (Type::Resource(a), Type::Resource(b)) => ref_eq(a, b),
            _ => matches!(self, other),
        }
    }
}