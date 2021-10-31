use pest::iterators::Pairs;

use crate::ast;

use super::Rule;

pub fn parse_system<'a>(mut pairs: Pairs<'a, Rule>) -> (&'a str, ast::Name<'a>) {
    todo!()
}