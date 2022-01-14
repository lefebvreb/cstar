use std::cell::RefCell;
use std::fmt;

use anyhow::{anyhow, Error, Result};

use crate::ast;
use crate::ecs;
use crate::utils::*;

// Holds definitions of function-like and struct-like objects.
pub struct Context<'a> {
    defs: Map<'a, Def<'a>>,
}

impl<'a> Context<'a> {
    // Defines a new object in the context, retunrns an error if an
    // object with the same name already exists.
    pub fn set_def(&mut self, name: &'a str, def: Def<'a>) -> Result<()> {
        self.defs.insert(name, def)
            .is_none().then(|| ())
            .ok_or_else(|| anyhow!("object with name '{}' already exists", name))
    }

    // Returns the definition corresponding to the given name, or an
    // error if no such definition exists.
    pub fn get_def(&self, name: &str) -> Result<Def> {
        self.defs.get(name)
            .ok_or_else(|| anyhow!("Definition {} does not exist", name))
            .map(Def::clone)
    }
}

impl Default for Context<'_> {
    // Creates a new empty context object.
    fn default() -> Self {
        Self {
            defs: Map::new(),  
        }
    }
}

// Holds all variables scopes.
pub struct Scope<'a> {
    vars: RefCell<Vec<Map<'a, Var<'a>>>>,
}

impl<'a> Scope<'a> {
    // Nests another new empty scope.
    pub fn next(&self) {
        self.vars.borrow_mut().push(Map::new());
    }

    // Destroys the last created scope, freeing all of it's variables.
    pub fn back(&self) {
        self.vars.borrow_mut().pop();
    }

    // Adds a new variable to the topmost scope.
    pub fn new_var(&self, name: &'a str, val: Var<'a>) {
        self.vars.borrow_mut().last_mut().unwrap().insert(name, val);
    }

    // Runs a closure on a requested variable.
    pub fn get_var(&self, path: &[&'a str], index: &[usize], reader: impl FnOnce(&Var<'a>) -> Result<Var<'a>>) -> Result<Var<'a>> {
        self.mutate_var(path, index, |var| reader(var))
    }

    // Runs a mutable closure on a requested variable.
    pub fn mutate_var(&self, path: &[&'a str], index: &[usize], mutator: impl FnOnce(&mut Var<'a>) -> Result<Var<'a>>) -> Result<Var<'a>> {
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

        for i in index {
            match var {
                Var::List(list) => var = list.get_mut(*i).ok_or_else(|| anyhow!("Out of bound index: {}.", i))?,
                _ => return Err(anyhow!("Cannot access index non-list variable.")),
            }
        }

        mutator(var)
    }
}

impl Default for Scope<'_> {
    // Creates a new empty scope object.
    fn default() -> Self {
        Self {
            vars: RefCell::new(vec![Map::new()]),
        }
    }
}

// A variable's value.
#[derive(Clone, Debug)]
pub enum Var<'a> {
    Void,
    Bool(bool),
    Int(i64),
    Float(f64),
    Char(char),
    String(String),
    List(Vec<Var<'a>>),
    Entity(ecs::Entity),
    Struct {
        name: &'a str,
        map: Map<'a, Var<'a>>,
    },
}

impl<'a> Var<'a> {
    // Returns the variable's type.
    pub fn get_type(&self) -> ast::Type<'a> {
        match self {
            Var::Void => ast::Type::Void,
            Var::Bool(_) => ast::Type::Bool,
            Var::Int(_) => ast::Type::Int,
            Var::Float(_) => ast::Type::Float,
            Var::Char(_) => ast::Type::Char,
            Var::String(_) => ast::Type::String,
            Var::List(_) => ast::Type::List,
            Var::Entity(_) => ast::Type::Entity,
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
            Var::List(list) => {
                write!(f, "[")?;
                let mut iter = list.iter();
                if let Some(var) = iter.next() {
                    write!(f, "{}", var)?;
                }
                for var in iter {
                    write!(f, ", {}", var)?;
                }
                write!(f, "]")
            },
            Var::Struct {map, ..} => {
                write!(f, "{{")?;
                let mut iter = map.iter();
                if let Some((name, var)) = iter.next() {
                    write!(f, "{}: {}", name, var)?;
                }
                for (name, var) in iter {
                    write!(f, ", {}: {}", name, var)?;
                }
                write!(f, "}}")
            },
        }
    }
}

impl PartialEq for Var<'_> {
    // Compares two variables.
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Var::Bool(l), Var::Bool(r)) => l == r,
            (Var::Int(l), Var::Int(r)) => l == r,
            (Var::Float(l), Var::Float(r)) => l == r,
            (Var::Char(l), Var::Char(r)) => l == r,
            (Var::String(l), Var::String(r)) => l == r,
            (Var::Entity(l), Var::Entity(r)) => l == r,
            (Var::Struct {name: l_name, ..}, Self::Struct {name: r_name, ..}) => l_name == r_name,
            _ => false,
        }
    }
}

// A definition of a struct-like or function-like object.
#[derive(Clone, Debug)]
pub enum Def<'a> {
    Function(&'a ast::Function<'a>),
    System(&'a ast::System<'a>),
    Component(&'a ast::StructDef<'a>),
    Resource(&'a ast::StructDef<'a>),
    Struct(&'a ast::StructDef<'a>),
}

// The result of the evaluation of a statement.
#[derive(Debug)]
pub enum Flow<'a> {
    Ok,
    Break,
    Continue,
    Return(Var<'a>),
}