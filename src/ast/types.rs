use super::*;

/// Represents a type.
#[derive(PartialEq)]
pub enum Type<'a> {
    Primitive(Primitive),
    Composite(&'a str),
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
    System,
    Entity,
}

#[derive(Default)]
pub struct StructDef<'a> {
    pub names: Map<'a, Type<'a>>,
}