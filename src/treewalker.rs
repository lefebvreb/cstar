use anyhow::Result;
use pest::iterators::Pairs;

use crate::ast::AST;
use crate::parser::Rule;

/// Treewalks the AST, interpreting the code.
pub fn treewalk(pairs: &AST) -> Result<()> {
    todo!()
}