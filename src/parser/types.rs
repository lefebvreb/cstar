use pest::iterators::Pairs;

use crate::ast;

use super::*;

/// Parses a structure definition.
pub fn parse_struct_def<'a>(mut pairs: Pairs<'a, Rule>) -> ast::StructDef<'a> {
    let mut def = ast::StructDef::default();
    
    while let Some(ident) = pairs.next() {
        def.names.insert(ident.as_str());
    }

    def
}