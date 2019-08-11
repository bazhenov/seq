extern crate clap;
extern crate regex;
mod processing;

use clap::{App, SubCommand};
use processing::{Record, Records};
use regex::Regex;
use serde_json;
use std::fs::File;
use std::io::{stdout, BufRead, BufReader, Read, Result, Write};

fn main() -> Result<()> {
    let matches = app().get_matches();

    let stdout = stdout();
    let stdout = stdout.lock();

    if let Some(matches) = matches.subcommand_matches("import") {
        let input_file_name = matches.value_of("INPUT").unwrap();
        let input_file = File::open(input_file_name)?;

        if let Some(output_file_name) = matches.value_of("OUTPUT") {
            let output_file = File::create(output_file_name)?;
            import(input_file, output_file)?;
        } else {
            import(input_file, stdout)?;
        }
    } else if let Some(matches) = matches.subcommand_matches("print") {
        let input_file_name = matches.value_of("INPUT").unwrap();
        let file = File::open(input_file_name)?;

        let records = Records::new(file);

        let regex = matches.value_of("REGEX").unwrap();
        let regex = Regex::new(regex).expect("Invalid regex");

        print(records, regex, stdout)?;
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
        .subcommand(
            SubCommand::with_name("print")
                .about("print matches")
                .arg_from_usage("[REGEX] -r --regex=[REGEX] 'regular expression'")
                .arg_from_usage("<INPUT> 'file to use'"),
        )
}

fn print(records: impl Iterator<Item = Record>, regex: Regex, mut out: impl Write) -> Result<()> {
    for record in records {
        let text = record.text;

        for m in regex.find_iter(&text) {
            writeln!(out, "{}", m.as_str())?;
        }
    }

    Ok(())
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
