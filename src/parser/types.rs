use pest::iterators::Pairs;

use crate::ast;

use super::*;

/// Parses a structure definition.
pub fn parse_struct_def<'a>(mut pairs: Pairs<'a, Rule>) -> ast::StructDef<'a> {
    let mut def = ast::StructDef::default();
    
    while let Some(ty) = pairs.next() {
        let ident = pairs.next().unwrap();
        def.names.push(ident.as_str());
    }

    def
}