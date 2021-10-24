use super::*;

/// Represents a type.
#[derive(PartialEq)]
pub enum Type<'a> {
    Primitive(Primitive),
    Component(Struct<'a>),
    Resource(Struct<'a>),
}

/// A primitive type.
#[derive(PartialEq)]
pub enum Primitive {
    Void,
    Bool,
    Int,
    Float,
    Char,
    String,
}

/// A struct-like type.
pub struct Struct<'a> {
    names: Map<'a, Type<'a>>,
}

impl<'a> PartialEq for Struct<'a> {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}