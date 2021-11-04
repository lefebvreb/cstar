use pest::iterators::Pairs;

use crate::ast;

use super::*;

pub fn parse_statement<'a>(mut pairs: Pairs<'a, Rule>) -> ast::Statement<'a> {
    todo!()
}

pub fn parse_block<'a>(mut pairs: Pairs<'a, Rule>) -> ast::Block<'a> {
    todo!()
}