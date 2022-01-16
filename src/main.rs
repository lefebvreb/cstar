#![allow(unused)]

mod ast;
mod ecs;
mod parser;
mod eval;
mod sources;
mod utils;

use std::path::Path;

use anyhow::Result;
use clap::{App, Arg};

fn main() -> Result<()> {
    // Parses the CLI arguments.
    let args = App::new("C* interpreter")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Benjamin Lefebvre")
        .about("An interpreter for the C* programming language.")
        .arg(Arg::with_name("source")
            .index(1)
            .value_name("SOURCE")
            .help("The path to the source file to be interpreted.")
            .required(true))
        .arg(Arg::with_name("ast")
            .long("ast")
            .help("Prints the AST of the source file and quits before evaluating it."))
        .get_matches();

    // Gets the source file's path.
    let path = Path::new(args.value_of("source").unwrap());

    // Parses the AST.
    let ast = parser::parse_program(path)?;

    // Prints the ast and exits if requested.
    if args.is_present("ast") {
        dbg!(&ast);
        return Ok(());
    }

    // Evaluates the AST.
    eval::eval(&ast)?;

    Ok(())
}
