use std::fs;
use std::path::Path;

use anyhow::{Error, Result};
use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;

use crate::ast::{self, Map};

/// The grammar of our language.
#[derive(Parser)]
#[grammar = "grammar.pest"]
struct Grammar;

/// Generates the Abstract Syntax Tree from the program's source code. 
pub fn parse_program<'a>(path: &str, src: &'a mut Vec<String>) -> Result<ast::AST<'a>> {
    // Parses the source code.
    let pairs = Grammar::parse(Rule::program, &src[0])?;

    let mut ast = ast::AST::default();

    for pair in pairs {
        match pair.as_rule() {
            Rule::element => parse_element(pair, &mut ast)?,
            Rule::init => ast.init = parse_ident_list(pair),
            Rule::run => ast.run = parse_ident_list(pair),
            _ => unreachable!(),
        }
    }

    Ok(ast)
}

/// Parses an element.
fn parse_element<'a>(pair: Pair<'a, Rule>, ast: &mut ast::AST<'a>) -> Result<()> {
    todo!()
}

/// Parses a list of identifiers.
fn parse_ident_list<'a>(pair: Pair<'a, Rule>) -> Vec<&'a str> {
    pair.into_inner().map(|pair| pair.as_str()).collect()
}