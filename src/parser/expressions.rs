use pest::iterators::Pairs;

use crate::ast;

use super::*;

pub fn parse_expr<'a>(mut pairs: Pairs<'a, Rule>) -> ast::Expr<'a> {
    todo!()
}