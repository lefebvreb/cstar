use pest::iterators::Pairs;

use crate::ast;

use super::Rule;

pub fn parse_statement<'a>(mut pairs: Pairs<'a, Rule>) -> ast::Statement<'a> {
    todo!()
}