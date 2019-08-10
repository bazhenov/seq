extern crate clap;
mod processing;

use clap::{App, SubCommand};
use processing::Record;
use serde_json;
use std::fs::File;
use std::io::{stdout, BufRead, BufReader, Read, Result, Write};

fn main() -> Result<()> {
    let matches = app().get_matches();

    if let Some(matches) = matches.subcommand_matches("import") {
        let input_file_name = matches.value_of("INPUT").unwrap();
        let input_file = File::open(input_file_name)?;

        if let Some(output_file_name) = matches.value_of("OUTPUT") {
            let output_file = File::create(output_file_name)?;
            import(input_file, output_file)?;
        } else {
            let stdout = stdout();
            import(input_file, stdout.lock())?;
        }
    }

    Ok(())
}

fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("sq")
        .author("Denis Bazhenov")
        .version("1.0")
        .about("sequence processing toolchain")
        .subcommand(
            SubCommand::with_name("import")
                .about("import examples in ndjson format")
                .arg_from_usage("[OUTPUT] -o --output=[FILE] 'output file'")
                .arg_from_usage("<INPUT> 'file to use'"),
        )
}

/// Reads input line by line and writes it to ouput in Record format
fn import(input: impl Read, mut out: impl Write) -> Result<()> {
    let buf = BufReader::new(input);

    for line in buf.lines().filter_map(Result::ok) {
        let record = Record {
            text: line.to_string(),
            spans: vec![],
        };
        let bytes = serde_json::to_vec(&record)?;
        out.write_all(&bytes)?;
        writeln!(out)?;
    }

    Ok(())
}
