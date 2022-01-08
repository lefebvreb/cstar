#![allow(unused)]

mod ast;
mod ecs;
mod parser;
mod eval;
mod utils;

use std::fs;
use std::path::Path;

use anyhow::Result;
use clap::{App, Arg};

fn main() -> Result<()> {
    /// Parses the CLI arguments.
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

    /// Reads the source file's path.
    let path = args.value_of("source").unwrap();

    /// Reads the source code.
    let mut src = vec![fs::read_to_string(path)?];

    /// Parses the AST.
    let ast = parser::parse_program(&path, &mut src)?;
    // dbg!(&ast); // Prints the AST.

    /// Evaluates the AST.
    eval::eval(&ast)?;

    Ok(())
}
