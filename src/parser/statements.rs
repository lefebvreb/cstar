use pest::iterators::Pairs;

use crate::ast;
use crate::utils::*;

use super::*;

// Parses a statement.
pub fn parse_statement(mut pairs: Pairs<'static, Rule>) -> ast::Statement {
    let pair = pairs.next().unwrap();

    match pair.as_rule() {
        Rule::decl => ast::Statement::Decl(parse_decl(pair.into_inner())),
        Rule::if_ => ast::Statement::If(parse_if(pair.into_inner())),
        Rule::for_ => ast::Statement::For(parse_for(pair.into_inner())),
        Rule::while_ => ast::Statement::While(parse_while(pair.into_inner())),
        Rule::query => ast::Statement::Query(parse_query(pair.into_inner())),
        Rule::switch => ast::Statement::Switch(parse_switch(pair.into_inner())),
        Rule::block => ast::Statement::Block(parse_block(pair.into_inner())),
        Rule::expr => ast::Statement::Expr(parse_expr(pair.into_inner())),
        Rule::break_ => ast::Statement::Break,
        Rule::continue_ => ast::Statement::Continue,
        Rule::return_ => ast::Statement::Return(parse_return(pair.into_inner())),
        _ => unreachable!(),
    }
}

// Parses a declaration.
pub fn parse_decl(mut pairs: Pairs<'static, Rule>) -> ast::Decl {
    ast::Decl {
        ident: pairs.next().unwrap().as_str(),
        init: pairs.next().map(|pair| parse_expr(pair.into_inner())),
    }
}

// Parses a block.
pub fn parse_block(mut pairs: Pairs<'static, Rule>) -> ast::Block {
    ast::Block {
        statements: pairs.map(|pair| parse_statement(pair.into_inner())).collect()
    }
}

// Parses a if.
pub fn parse_if(mut pairs: Pairs<'static, Rule>) -> ast::If {
    ast::If {
        cond: parse_expr(pairs.next().unwrap().into_inner()),
        branch1: parse_block(pairs.next().unwrap().into_inner()),
        branch2: pairs.next().map(|pair| parse_block(pair.into_inner())),
    }
}

// Parses a for.
pub fn parse_for(mut pairs: Pairs<'static, Rule>) -> ast::For {
    ast::For {
        init: {
            let pair = pairs.next().unwrap();
            match pair.as_rule() {
                Rule::expr => Either::Left(parse_expr(pair.into_inner())),
                Rule::decl => Either::Right(parse_decl(pair.into_inner())),
                _ => unreachable!(),
            }
        },
        cond: parse_expr(pairs.next().unwrap().into_inner()),
        incr: parse_expr(pairs.next().unwrap().into_inner()),
        code: parse_block(pairs.next().unwrap().into_inner())
    }
}

// Parses a while loop.
pub fn parse_while(mut pairs: Pairs<'static, Rule>) -> ast::While {
    ast::While {
        cond: parse_expr(pairs.next().unwrap().into_inner()),
        code: parse_block(pairs.next().unwrap().into_inner()),
    }
}

// Parses a query loop.
pub fn parse_query(mut pairs: Pairs<'static, Rule>) -> ast::Query {
    ast::Query {
        filters: parse_filter_list(pairs.next().unwrap().into_inner()),
        code: parse_block(pairs.next().unwrap().into_inner()),
    }
}

// Parses a switch block.
pub fn parse_switch(mut pairs: Pairs<'static, Rule>) -> ast::Switch {
    let expr = parse_expr(pairs.next().unwrap().into_inner());
    let mut cases = Vec::new();

    while let Some(pair) = pairs.next() {
        match pair.as_rule() {
            Rule::atom => cases.push(ast::SwitchCase {
                val: parse_atom(pair.into_inner()),
                block: parse_block(pairs.next().unwrap().into_inner()),
            }),
            Rule::block => return ast::Switch {
                expr,
                cases,
                default: parse_block(pair.into_inner()),
            },
            _ => unreachable!(),
        }
    }

    unreachable!()
}

// Parses a switch block.
pub fn parse_case(mut pairs: Pairs<'static, Rule>) -> ast::SwitchCase {
    ast::SwitchCase {
        val: parse_atom(pairs.next().unwrap().into_inner()),
        block: parse_block(pairs.next().unwrap().into_inner()),
    }
}

// Parses a return statement.
pub fn parse_return(mut pairs: Pairs<'static, Rule>) -> Option<ast::Expr> {
    pairs.next().map(|pair| parse_expr(pair.into_inner()))
}
