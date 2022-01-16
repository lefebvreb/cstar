use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Error, Result};
use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;

use crate::ast;
use crate::sources::Sources;
use crate::utils::*;

mod expressions;
use expressions::*;

mod statements;
use statements::*;

mod systems;
use systems::*;

mod types;
use types::*;

// The grammar of our language.
#[derive(Parser)]
#[grammar = "grammar.pest"]
struct Grammar;

// Generates the Abstract Syntax Tree from the program's source code. 
pub fn parse_program<'a>(path: &Path) -> Result<ast::AST<'a>> {
    let mut src = Sources::default();

    let mut pairs = Grammar::parse(Rule::program, src.add(path)?.unwrap())?
        .next().unwrap().into_inner();

    let mut ast = ast::AST::default();

    for pair in pairs {
        match pair.as_rule() {
            Rule::include => parse_module(pair.into_inner(), &mut ast, &mut src)?,
            Rule::element => {
                let (name, element) = parse_element(pair.into_inner())?;
                ast.names.insert(name, element);
            }
            Rule::init => ast.init = parse_ident_list(pair.into_inner()),
            Rule::run => ast.run = parse_ident_list(pair.into_inner()),
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(ast)
}

// Parse a module file.
fn parse_module<'a>(mut pairs: Pairs<'a, Rule>, ast: &mut ast::AST<'a>, src: &mut Sources) -> Result<()> {
    let path = parse_string(pairs.next().unwrap().as_str());
    
    if let Some(file) = src.add(Path::new(&path))? {
        let mut pairs = Grammar::parse(Rule::module, file)?
            .next().unwrap().into_inner();

        for pair in pairs {
            match pair.as_rule() {
                Rule::include => parse_module(pair.into_inner(), ast, src)?,
                Rule::element => {
                    let (name, element) = parse_element(pair.into_inner())?;
                    ast.names.insert(name, element);
                }
                Rule::EOI => (),
                _ => unreachable!(),
            }
        }
    }

    Ok(())
    /*let mut pairs = pairs.peekable();

    for pair in pairs {
        match pair.as_rule() {
            Rule::element => {
                let (name, element) = parse_element(pair.into_inner())?;
                module.names.insert(name, element);
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(module)*/
}

// Parses an include directive.
fn parse_include<'a>(mut pairs: Pairs<'a, Rule>) -> PathBuf {
    PathBuf::from(parse_string(pairs.next().unwrap().as_str()))
}

// Parses an element.
fn parse_element<'a>(mut pairs: Pairs<'a, Rule>) -> Result<(&'a str, ast::Name<'a>)> {
    let element = pairs.next().unwrap();

    match element.as_rule() {
        Rule::component => parse_component(element.into_inner()),
        Rule::resource => parse_resource(element.into_inner()),
        Rule::struct_ => parse_struct(element.into_inner()),
        Rule::function => Ok(parse_function(element.into_inner())),
        Rule::system => Ok(parse_system(element.into_inner())),
        _ => unreachable!(),
    }
}

// Parses a componenet definition.
fn parse_component<'a>(mut pairs: Pairs<'a, Rule>) -> Result<(&'a str, ast::Name<'a>)> {
    let name = pairs.next().unwrap().as_str();
    let def = parse_struct_def(pairs.next().unwrap().into_inner())?;
    Ok((name, ast::Name::Component(def)))
}

// Parses a resource definition.
fn parse_resource<'a>(mut pairs: Pairs<'a, Rule>) -> Result<(&'a str, ast::Name<'a>)> {
    let name = pairs.next().unwrap().as_str();
    let def = parse_struct_def(pairs.next().unwrap().into_inner())?;
    Ok((name, ast::Name::Resource(def)))
}

// Parses a componenet definition.
fn parse_struct<'a>(mut pairs: Pairs<'a, Rule>) -> Result<(&'a str, ast::Name<'a>)> {
    let name = pairs.next().unwrap().as_str();
    let def = parse_struct_def(pairs.next().unwrap().into_inner())?;
    Ok((name, ast::Name::Struct(def)))
}

// Parses a function definition.
fn parse_function<'a>(mut pairs: Pairs<'a, Rule>) -> (&'a str, ast::Name<'a>) {
    let name = pairs.next().unwrap().as_str();
    let mut args = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::ident => args.push(pair.as_str()),
            Rule::block => return (name, ast::Name::Function(ast::Function {
                args, body: parse_block(pair.into_inner()),
            })),
            _ => unreachable!(),
        }
    };

    unreachable!();
}

// Parses a list of identifiers.
fn parse_ident_list<'a>(pairs: Pairs<'a, Rule>) -> Vec<&'a str> {
    pairs.map(|pair| pair.as_str()).collect()
}