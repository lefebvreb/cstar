use std::fmt;

use super::*;

#[derive(Default, Debug)]
pub struct StructDef {
    pub fields: Map<Type>,
}

// A primitive type.
#[derive(PartialEq, Eq, Debug)]
pub enum Type  {
    Void,
    Bool,
    Int,
    Float,
    Char,
    String,
    List,
    Entity,
    Struct(&'static str),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Void => write!(f, "void"),
            Type::Bool => write!(f, "bool"),
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::Char => write!(f, "char"),
            Type::String => write!(f, "string"),
            Type::List => write!(f, "list"),
            Type::Entity => write!(f, "entity"),
            Type::Struct(name) => write!(f, "{}", name),
        }
    }
}