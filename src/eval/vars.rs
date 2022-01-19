use std::fmt;

use super::*;

// A variable's value.
#[derive(Clone, Debug)]
pub enum Var {
    Void,
    Bool(bool),
    Int(i64),
    Float(f64),
    Char(char),
    String(String),
    Entity(ecs::Entity),
    List(Shared<Vec<Var>>),
    Struct(Shared<Struct>),
}

#[derive(Debug)]
pub struct Struct {
    pub name: &'static str,
    pub map: Map<Var>,
}

impl Var {
    // Returns the variable's type.
    pub fn get_type(&self) -> ast::Type {
        match self {
            Var::Void => ast::Type::Void,
            Var::Bool(_) => ast::Type::Bool,
            Var::Int(_) => ast::Type::Int,
            Var::Float(_) => ast::Type::Float,
            Var::Char(_) => ast::Type::Char,
            Var::String(_) => ast::Type::String,
            Var::List(_) => ast::Type::List,
            Var::Entity(_) => ast::Type::Entity,
            Var::Struct(s) => ast::Type::Struct(s.borrow().name),
        }
    }
}

impl fmt::Display for Var {
    // Prints a variable.
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
                let borrow = list.borrow();
                let mut iter = borrow.iter();
                if let Some(var) = iter.next() {
                    write!(f, "{}", var)?;
                }
                for var in iter {
                    write!(f, ", {}", var)?;
                }
                write!(f, "]")
            },
            Var::Struct(s) => {
                write!(f, "{{")?;
                let borrow = s.borrow();
                let mut iter = borrow.map.iter();
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

impl PartialEq for Var {
    // Compares two variables.
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Var::Bool(l), Var::Bool(r)) => l == r,
            (Var::Int(l), Var::Int(r)) => l == r,
            (Var::Float(l), Var::Float(r)) => l == r,
            (Var::Char(l), Var::Char(r)) => l == r,
            (Var::String(l), Var::String(r)) => l == r,
            (Var::Entity(l), Var::Entity(r)) => l == r,
            (Var::Struct(l), Self::Struct(r)) => l.borrow().name == r.borrow().name,
            _ => false,
        }
    }
}