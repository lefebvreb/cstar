use std::fs;
use std::path::Path;

use anyhow::{Error, Result};
use pest::Parser;
use pest::iterators::Pairs;
use pest_derive::Parser;

use crate::ast::AST;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct Grammar;

/// Generates the Abstract Syntax Tree from the program's source code. 
pub fn parse_program(src: &str) -> Result<AST> {
    let pairs = Grammar::parse(Rule::program, src).map_err(|e| Error::msg(e))?;

    for pair in pairs {
        match pair.as_rule() {
            Rule::component => {

            },
            Rule::system => {

            },
            Rule::const_decl => {

            },
            _ => unreachable!(),
        }
    }

    todo!()
}