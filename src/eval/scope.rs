use anyhow::{anyhow, Error, Result};

use crate::ast;
use crate::ecs;
use crate::utils::*;

/// A struct holding all values of the program, including
/// struct and functions definitions, as well as variables.
/// Variables scopes may be nested, and are stored in a stack,
/// definitions may not.
pub struct Scope<'a> {
    defs: Map<'a, Definition<'a>>,
    vars: Vec<Map<'a, Variable<'a>>>,
}

impl<'a> Scope<'a> {
    /// Creates a new empty scope object.
    pub fn new() -> Self {
        Self {
            defs: Map::new(),
            vars: vec![Map::new()],
        }
    }

    /// Nests another new empty variable scope.
    pub fn next(&mut self) {
        self.vars.push(Map::new());
    }

    /// Destroys the last created variable scope, freeing all
    /// of it's variables.
    pub fn back(&mut self) {
        self.vars.pop();
    }

    /// Adds a new definition to the current scope.
    pub fn add_def(&mut self, name: &'a str, def: Definition<'a>) -> Result<()> {
        if self.defs.contains_key(name) {
            return Err(anyhow!("Definition already exists: {}", name));
        }
        self.defs.insert(name, def);
        Ok(())
    }

    /// Gets a referecne to a definition by its name.
    pub fn get_def(&self, name: &str) -> Result<&Definition> {
        self.defs.get(name).ok_or_else(|| anyhow!("Definition {} does not exist", name))
    }

    /// Adds a new variable to the current scope.
    pub fn add_var(&mut self, name: &'a str, var: Variable<'a>) {
        self.vars.last_mut().unwrap().insert(name, var);
    }

    /// Gets a reference to a variable or constant from the current scope.
    pub fn get_var(&self, name: &str) -> Result<&Variable> {
        self.vars.iter().rev()
            .find_map(|scope| scope.get(name))
            .ok_or_else(|| anyhow!("Variable {} does not exist", name))
    }

    /// Gets a mutable reference to a variable from the current scope.
    pub fn get_var_mut(&'a mut self, name: &str) -> Result<&'a mut Variable> {
        self.vars.iter_mut().rev()
            .find_map(|scope| scope.get_mut(name))
            .ok_or_else(|| anyhow!("Variable {} does not exist or is readonly", name))
    }
}

/// A variable's value.
#[derive(Debug)]
pub enum Variable<'a> {
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
        val: Map<'a, Variable<'a>>,
    },
}

impl<'a> Variable<'a> {
    /// Gets the type of this variable
    pub fn get_type(&self) -> Type<'a> {
        match self {
            Variable::Void => Type::Void,
            Variable::Bool(_) => Type::Bool,
            Variable::Int(_) => Type::Int,
            Variable::Float(_) => Type::Float,
            Variable::Char(_) => Type::Char,
            Variable::String(_) => Type::String,
            Variable::Entity(_) => Type::Entity,
            Variable::System(_) => Type::System,
            Variable::Struct { typ, .. } => typ.clone(),
        }
    }
}

/// A definition of a struct-like or function-like object.
#[derive(Debug)]
pub enum Definition<'a> {
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