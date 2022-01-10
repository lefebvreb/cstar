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
    //vars: Vec<Map<'a, Var<'a>>>,
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

    /// Adds a new variable to the current scope.
    pub fn set_var(&self, name: &'a str, var: Var<'a>) {
        self.vars.borrow_mut().last_mut().unwrap().insert(name, var);
    }

    /// Gets a reference to a variable or constant from the current scope.
    pub fn get_var(&self, name: &str) -> Result<Var> {
        self.vars.borrow().iter().rev()
            .find_map(|scope| scope.get(name))
            .ok_or_else(|| anyhow!("Variable {} does not exist in current scope.", name))
            .map(Var::clone)
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
    Struct(Map<'a, Var<'a>>),
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
            Var::System(sys) => todo!(),
            Var::Struct(st) => {
                writeln!(f, "{{")?;
                for (name, var) in st.iter() {
                    writeln!(f, "\t{}: {}", name, var)?;
                }
                writeln!(f, "}}")
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
#[derive(PartialEq, Eq, Debug)]
pub enum StmtRes {
    Ok,
    Break,
    Continue,
}