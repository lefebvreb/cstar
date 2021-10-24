#![allow(unused)]

mod ast;

mod parser;
mod treewalker;

mod ecs;

use std::fs;
use std::path::Path;

use anyhow::Result;
use clap::{App, Arg};

fn main() -> Result<()> {
    let args = App::new("C* interpreter")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Benjamin Lefebvre")
        .about("An interpreter for the C* programming language.")
        .arg(Arg::with_name("source")
            .index(1)
            .value_name("SOURCE")
            .help("The path to the source file to be interpreted.")
            .required(true))
        .get_matches();

    let src = fs::read_to_string(args.value_of("source").unwrap())?;

    let ast = parser::parse_program(&src)?;

    treewalker::treewalk(&ast)?;

    Ok(())
}
