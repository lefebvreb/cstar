use pest::iterators::Pairs;

use crate::ast;
use crate::utils::*;

use super::*;

/// Parses a statement.
pub fn parse_statement<'a>(mut pairs: Pairs<'a, Rule>) -> ast::Statement<'a> {
    let pair = pairs.next().unwrap();

    match pair.as_rule() {
        Rule::assign => ast::Statement::Assign(parse_assign(pair.into_inner())),
        Rule::if_ => ast::Statement::If(parse_if(pair.into_inner())),
        Rule::for_ => ast::Statement::For(parse_for(pair.into_inner())),
        Rule::while_ => ast::Statement::While(parse_while(pair.into_inner())),
        Rule::query => ast::Statement::Query(parse_query(pair.into_inner())),
        Rule::block => ast::Statement::Block(parse_block(pair.into_inner())),
        Rule::expr => ast::Statement::Expr(parse_expr(pair.into_inner())),
        Rule::break_ => ast::Statement::Break,
        Rule::continue_ => ast::Statement::Continue,
        _ => unreachable!(),
    }
}

/// Parses an assignement.
pub fn parse_assign<'a>(mut pairs: Pairs<'a, Rule>) -> ast::Assign<'a> {
    ast::Assign {
        lvalue: parse_lvalue(pairs.next().unwrap().into_inner()),
        expr: parse_expr(pairs.next().unwrap().into_inner()),
    }
}

/// Parses a block.
pub fn parse_block<'a>(mut pairs: Pairs<'a, Rule>) -> ast::Block<'a> {
    ast::Block {
        statements: pairs.map(|pair| parse_statement(pair.into_inner())).collect()
    }
}

/// Parses a if.
pub fn parse_if<'a>(mut pairs: Pairs<'a, Rule>) -> ast::If<'a> {
    ast::If {
        cond: parse_expr(pairs.next().unwrap().into_inner()),
        branch1: parse_block(pairs.next().unwrap().into_inner()),
        branch2: pairs.next().map(|pair| parse_block(pair.into_inner())),
    }
}

/// Parses a for.
pub fn parse_for<'a>(mut pairs: Pairs<'a, Rule>) -> ast::For<'a> {
    let mut res = ast::For::default();

    let mut pair = pairs.next().unwrap();

    if let Rule::assign = pair.as_rule() {
        res.init = Some(parse_assign(pair.into_inner()));
        pair = pairs.next().unwrap();
    }

    if let Rule::expr = pair.as_rule() {
        res.cond = Some(parse_expr(pair.into_inner()));
        pair = pairs.next().unwrap();
    }

    if let Rule::assign = pair.as_rule() {
        res.incr = Some(parse_assign(pair.into_inner()));
        pair = pairs.next().unwrap();
    }

    res.code = parse_block(pair.into_inner());

    res
}

/// Parses a while loop.
pub fn parse_while<'a>(mut pairs: Pairs<'a, Rule>) -> ast::While<'a> {
    ast::While {
        cond: parse_expr(pairs.next().unwrap().into_inner()),
        code: parse_block(pairs.next().unwrap().into_inner()),
    }
}

/// Parses a query loop.
pub fn parse_query<'a>(mut pairs: Pairs<'a, Rule>) -> ast::Query<'a> {
    ast::Query {
        filters: parse_filter_list(pairs.next().unwrap().into_inner()),
        code: parse_block(pairs.next().unwrap().into_inner()),
    }
}