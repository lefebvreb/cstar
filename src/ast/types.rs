use super::*;

/// Represents a type.
#[derive(PartialEq, Debug)]
pub enum Type<'a> {
    Primitive(Primitive),
    Composite(&'a str),
}

/// A primitive type.
#[derive(PartialEq, Debug)]
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

#[derive(Default, Debug)]
pub struct StructDef<'a> {
    pub names: Map<'a, Type<'a>>,
}