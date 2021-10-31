use pest::iterators::Pairs;

use crate::ast;

use super::Rule;

/// Parses a structure definition.
pub fn parse_struct_def<'a>(mut pairs: Pairs<'a, Rule>) -> ast::StructDef<'a> {
    let mut def = ast::StructDef::default();
    
    while let Some(ty) = pairs.next() {
        let ty = parse_type(ty.into_inner());
        let ident = pairs.next().unwrap();
        def.names.insert(ident.as_str(), ty);
    }

    def
}

/// Parses a type.
pub fn parse_type<'a>(mut pairs: Pairs<'a, Rule>) -> ast::Type<'a> {
    let pair = pairs.next().unwrap();
    
    match pair.as_rule() {
        Rule::primitive => parse_primitive(pair.into_inner()),
        Rule::ident => ast::Type::Composite(pair.as_str()),
        _ => unreachable!(),
    }
}

/// Parses a primitive type.
pub fn parse_primitive<'a>(mut pairs: Pairs<'a, Rule>) -> ast::Type<'a> {
    ast::Type::Primitive(match pairs.next().unwrap().as_rule() {
        Rule::void_t => ast::Primitive::Void,
        Rule::bool_t => ast::Primitive::Bool,
        Rule::int_t => ast::Primitive::Int,
        Rule::float_t => ast::Primitive::Float,
        Rule::char_t => ast::Primitive::Char,
        Rule::string_t => ast::Primitive::String,
        Rule::system_t => ast::Primitive::System,
        Rule::system_t => ast::Primitive::Entity,
        _ => unreachable!(),
    })
}