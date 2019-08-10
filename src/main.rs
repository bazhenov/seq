extern crate clap;
mod processing;

use clap::{App, SubCommand};
use processing::Record;
use serde_json;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

fn main() {
    let matches = app().get_matches();

    if let Some(matches) = matches.subcommand_matches("import") {
        let file = matches.value_of("INPUT").unwrap();
        import(file);
    }
}

fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("sq")
        .author("Denis Bazhenov")
        .version("1.0")
        .about("sequence processing toolchain")
        .subcommand(
            SubCommand::with_name("import")
                .about("import examples in ndjson format")
                .args_from_usage("<INPUT> 'file to use'"),
        )
}

fn import(input: impl AsRef<Path>) -> Result<()> {
    let file = File::open(input)?;
    let buf = BufReader::new(file);

    for ref line in buf.lines() {
        println!("{:?}", line);
    }

    Ok(())
}
