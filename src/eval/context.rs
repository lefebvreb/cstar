use std::cell::RefCell;
use std::fmt;

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
    vars: RefCell<Vec<Map<'a, Var<'a>>>>,
}

impl<'a> Scope<'a> {
    /// Nests another new empty scope.
    pub fn next(&self) {
        self.vars.borrow_mut().push(Map::new());
    }

    /// Destroys the last created scope, freeing all of it's variables.
    pub fn back(&self) {
        self.vars.borrow_mut().pop();
    }

    /// Adds a new variable to the topmost scope.
    pub fn new_var(&self, name: &'a str, val: Var<'a>) {
        self.vars.borrow_mut().last_mut().unwrap().insert(name, val);
    }

    /// Updates a variable in the current scope.
    pub fn set_var(&self, name: &'a str, val: Var<'a>) -> Result<()> {
        let mut vars = self.vars.borrow_mut();
        let var = vars.iter_mut().rev()
            .find_map(|scope| scope.get_mut(name))
            .ok_or_else(|| anyhow!("Variable {} is not declared is current scope", name))?;

        if matches!(var, Var::Struct {..}) {
            return Err(anyhow!("Cannot update a struct variable."));
        }

        *var = val;
        Ok(())
    }

    /// Gets a copy of a variable from the current scope.
    pub fn get_var(&self, name: &'a str) -> Result<Var<'a>> {
        self.vars.borrow().iter().rev()
            .find_map(|scope| scope.get(name))
            .ok_or_else(|| anyhow!("Variable {} does not exist in current scope.", name))
            .map(Var::clone)
    }

    /// Gets a copy of the variable at the specified path.
    pub fn get_path(&self, path: &[&'a str]) -> Result<Var<'a>> {
        let vars = self.vars.borrow();
        let mut var = vars.iter().rev()
            .find_map(|scope| scope.get(path[0]))
            .ok_or_else(|| anyhow!("Variable {} does not exist in current scope.", path[0]))?;

        for ident in &path[1..] {
            match var {
                Var::Struct {map, ..} => var = map.get(ident).ok_or_else(|| anyhow!("Field {} does not exist in struct {}.", ident, var))?,
                _ => return Err(anyhow!("Cannot access field {} of non-struct variable.", ident)),
            }
        }

        Ok(var.clone())
    }

    /// Updates the value at the given path.
    pub fn set_path(&self, path: &[&'a str], val: Var<'a>) -> Result<()> {
        let mut vars = self.vars.borrow_mut();
        let mut var = vars.iter_mut().rev()
            .find_map(|scope| scope.get_mut(path[0]))
            .ok_or_else(|| anyhow!("Variable {} does not exist in current scope.", path[0]))?;

        for ident in &path[1..] {
            match var {
                Var::Struct {map, ..} => var = map.get_mut(ident).ok_or_else(|| anyhow!("Field {} does not exist.", ident))?,
                _ => return Err(anyhow!("Cannot access field {} of non-struct variable.", ident)),
            }
        }

        if matches!(var, Var::Struct {..}) {
            return Err(anyhow!("Cannot update a struct variable."));
        }

        if val.get_type() != var.get_type() {
            return Err(anyhow!("Type mismatch: cannot assign {} to {}.", val.get_type(), var.get_type()));
        }

        *var = val;
        Ok(())
    }
}

impl Default for Scope<'_> {
    /// Creates a new empty scope object.
    fn default() -> Self {
        Self {
            vars: RefCell::new(vec![Map::new()]),
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
        name: &'a str,
        map: Map<'a, Var<'a>>,
    },
}

impl<'a> Var<'a> {
    /// Returns the variable's type.
    pub fn get_type(&self) -> ast::Type<'a> {
        match self {
            Var::Void => ast::Type::Void,
            Var::Bool(_) => ast::Type::Bool,
            Var::Int(_) => ast::Type::Int,
            Var::Float(_) => ast::Type::Float,
            Var::Char(_) => ast::Type::Char,
            Var::String(_) => ast::Type::String,
            Var::Entity(_) => ast::Type::System,
            Var::System(_) => ast::Type::System,
            Var::Struct {name, ..} => ast::Type::Struct(name),
        }
    }
}

impl<'a> fmt::Display for Var<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Var::Void => write!(f, "void"),
            Var::Bool(b) => write!(f, "{}", b),
            Var::Int(i) => write!(f, "{}", i),
            Var::Float(x) => write!(f, "{}", x),
            Var::Char(c) => write!(f, "{}", c),
            Var::String(s) => write!(f, "{}", s),
            Var::Entity(e) => todo!(),
            Var::System(sys) => write!(f, "System at {:p}", &sys),
            Var::Struct {map, ..} => {
                write!(f, "{{")?;
                let mut iter = map.iter();
                if let Some((name, var)) = iter.next() {
                    write!(f, "\"{}\": {}", name, var)?;
                }
                for (name, var) in iter {
                    write!(f, ", \"{}\": {}", name, var)?;
                }
                write!(f, "}}")
            },
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

/// The result of the evaluation of a statement.
#[derive(Debug)]
pub enum Flow<'a> {
    Ok,
    Break,
    Continue,
    Return(Var<'a>),
}