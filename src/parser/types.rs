use crate::ast;

use super::*;

pub fn parse_type<'a>(mut pairs: Pairs<'a, Rule>) -> ast::Type<'a> {
    let pair = pairs.next().unwrap();
    match pair.as_rule() {
        Rule::void_t => ast::Type::Void,
        Rule::bool_t => ast::Type::Bool,
        Rule::int_t => ast::Type::Int,
        Rule::float_t => ast::Type::Float,
        Rule::char_t => ast::Type::Char,
        Rule::string_t => ast::Type::String,
        Rule::system_t => ast::Type::System,
        Rule::ident => ast::Type::Struct(pair.as_str()),
        _ => unreachable!(),
    }
}

/// Parses a structure definition.
pub fn parse_struct_def<'a>(mut pairs: Pairs<'a, Rule>) -> Result<ast::StructDef<'a>> {
    let mut def = ast::StructDef::default();
    
    while let Some(ty) = pairs.next() {
        let name = pairs.next().unwrap().as_str();
        let ty = parse_type(ty.into_inner());
        if def.fields.insert(name, ty).is_some() {
            return Err(anyhow!("Duplicate field name: {}.", name));
        }
    }

    Ok(def)
}