use pest::iterators::Pairs;

use crate::ast;
use crate::utils::*;

use super::*;

/// Parses a statement.
pub fn parse_statement<'a>(mut pairs: Pairs<'a, Rule>) -> ast::Statement<'a> {
    let pair = pairs.next().unwrap();

    match pair.as_rule() {
        Rule::if_ => ast::Statement::If(parse_if(pair.into_inner())),
        Rule::for_ => ast::Statement::For(parse_for(pair.into_inner())),
        Rule::while_ => ast::Statement::While(parse_while(pair.into_inner())),
        Rule::query => ast::Statement::Query(parse_query(pair.into_inner())),
        Rule::block => ast::Statement::Block(parse_block(pair.into_inner())),
        Rule::expr => ast::Statement::Expr(parse_expr(pair.into_inner())),
        Rule::decl => ast::Statement::Decl(parse_declaration(pair.into_inner())),
        Rule::break_ => ast::Statement::Break,
        Rule::continue_ => ast::Statement::Continue,
        _ => unreachable!(),
    }
}

/// Parses a block.
pub fn parse_block<'a>(mut pairs: Pairs<'a, Rule>) -> ast::Block<'a> {
    ast::Block {
        statements: pairs.map(|pair| parse_statement(pair.into_inner())).collect()
    }
}

/// Parses a declaration.
pub fn parse_declaration<'a>(mut pairs: Pairs<'a, Rule>) -> ast::Decl<'a> {
    let mut pair = pairs.next().unwrap();

    let is_const = matches!(pair.as_rule(), Rule::const_);
    if !is_const {
        pair = pairs.next().unwrap();
    }
    let ty = parse_type(pair.into_inner());
    let name = pairs.next().unwrap().as_str();
    let init = pairs.next().map(|pair| parse_expr(pair.into_inner()));

    ast::Decl {is_const, ty, name, init}
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

    match pair.as_rule() {
        Rule::expr => {
            res.init = Some(Either::Left(parse_expr(pair.into_inner())));
            pair = pairs.next().unwrap();
        },
        Rule::decl => {
            res.init = Some(Either::Right(parse_declaration(pair.into_inner())));
            pair = pairs.next().unwrap();
        },
        _ => (),
    }

    match pair.as_rule() {
        Rule::expr => {
            res.cond = Some(parse_expr(pair.into_inner()));
            pair = pairs.next().unwrap();
        },
        _ => (),
    }

    match pair.as_rule() {
        Rule::expr => {
            res.incr = Some(parse_expr(pair.into_inner()));
            pair = pairs.next().unwrap();
        },
        _ => (),
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