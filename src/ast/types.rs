use super::*;

/// Represents a type.
#[derive(PartialEq)]
pub enum Type<'a> {
    Primitive(Primitive),
    Component(&'a str),
    Resource(&'a str),
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