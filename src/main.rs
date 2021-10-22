#![allow(unused)]

use std::fs;

use anyhow::{Error, Result};
use clap::{App, Arg};

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct Grammar;

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

    let src_path = args.value_of("source").unwrap();

    let text = fs::read_to_string(src_path)?;

    let parse = Grammar::parse(Rule::program, &text)?;

    println!("{}", parse);

    Ok(())
}
