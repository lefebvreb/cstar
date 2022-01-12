use std::fs;
use std::path::Path;

use anyhow::{anyhow, Error, Result};
use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;

use crate::ast;
use crate::utils::*;

mod expressions;
use expressions::*;

mod statements;
use statements::*;

mod systems;
use systems::*;

mod types;
use types::*;

/// The grammar of our language.
#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
struct Grammar;

/// Generates the Abstract Syntax Tree from the program's source code. 
pub fn parse_program<'a>(path: &str, src: &'a mut Vec<String>) -> Result<ast::AST<'a>> {
    let pairs = Grammar::parse(Rule::program, &src[0])?
        .next().unwrap().into_inner();

    let mut ast = ast::AST::default();

    for element in pairs {
        match element.as_rule() {
            Rule::element => {
                let (name, element) = parse_element(element.into_inner())?;
                ast.names.insert(name, element);
            },
            Rule::init => ast.init = parse_ident_list(element.into_inner()),
            Rule::run => ast.run = parse_ident_list(element.into_inner()),
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(ast)
}

/// Parses an element.
fn parse_element<'a>(mut pairs: Pairs<'a, Rule>) -> Result<(&'a str, ast::Name<'a>)> {
    let element = pairs.next().unwrap();

    match element.as_rule() {
        Rule::component => parse_component(element.into_inner()),
        Rule::resource => parse_resource(element.into_inner()),
        Rule::system => Ok(parse_system(element.into_inner())),
        _ => unreachable!(),
    }
}

/// Parses a componenet definition.
fn parse_component<'a>(mut pairs: Pairs<'a, Rule>) -> Result<(&'a str, ast::Name<'a>)> {
    let name = pairs.next().unwrap().as_str();
    let def = parse_struct_def(pairs.next().unwrap().into_inner())?;
    Ok((name, ast::Name::Component(def)))
}

/// Parses a resource definition.
fn parse_resource<'a>(mut pairs: Pairs<'a, Rule>) -> Result<(&'a str, ast::Name<'a>)> {
    let name = pairs.next().unwrap().as_str();
    let def = parse_struct_def(pairs.next().unwrap().into_inner())?;
    Ok((name, ast::Name::Resource(def)))
}

/// Parses a list of identifiers.
fn parse_ident_list<'a>(pairs: Pairs<'a, Rule>) -> Vec<&'a str> {
    pairs.map(|pair| pair.as_str()).collect()
}