use std::fs;
use std::path::Path;

use anyhow::Result;
use pest::Parser;
use pest_derive::Parser;

use crate::ast::AST;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct Grammar;

pub fn parse_program(src: &str) -> Result<AST> {
    todo!()
}