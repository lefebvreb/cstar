use std::fmt;

use super::*;

#[derive(Default, Debug)]
pub struct StructDef<'a> {
    pub fields: Map<'a, Type<'a>>,
}

// A primitive type.
#[derive(PartialEq, Eq, Debug)]
pub enum Type<'a>  {
    Void,
    Bool,
    Int,
    Float,
    Char,
    String,
    Entity,
    Struct(&'a str),
}

impl fmt::Display for Type<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Void => write!(f, "void"),
            Type::Bool => write!(f, "bool"),
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::Char => write!(f, "char"),
            Type::String => write!(f, "string"),
            Type::Entity => write!(f, "entity"),
            Type::Struct(name) => write!(f, "{}", name),
        }
    }
}