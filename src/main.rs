extern crate clap;
extern crate regex;
mod processing;

use clap::{App, SubCommand};
use processing::{Record, Records};
use regex::Regex;
use serde::Serialize;
use serde_json;
use std::fs;
use std::fs::File;
use std::io::{stdout, BufRead, BufReader, Result, Write};

fn main() -> Result<()> {
    let matches = app().get_matches();

    let stdout = stdout();
    let stdout = stdout.lock();

    if let Some(matches) = matches.subcommand_matches("import") {
        let input_file_name = matches.value_of("INPUT").unwrap();
        let input_file = File::open(input_file_name)?;
        let lines = BufReader::new(input_file).lines().filter_map(Result::ok);

        if let Some(output_file_name) = matches.value_of("OUTPUT") {
            let output_file = File::create(output_file_name)?;
            import(lines, output_file)?;
        } else {
            import(lines, stdout)?;
        }
    } else if let Some(matches) = matches.subcommand_matches("print") {
        let input_file_name = matches.value_of("INPUT").unwrap();
        let file = File::open(input_file_name)?;

        let records = Records::new(file);

        let regex = matches.value_of("REGEX").unwrap();
        let regex = Regex::new(regex).expect("Invalid regex");

        print(records, &regex, stdout)?;
    } else if let Some(matches) = matches.subcommand_matches("mark") {
        let input_file_name = matches.value_of("INPUT").unwrap();
        let input_file = File::open(input_file_name)?;

        let records = Records::new(input_file);

        let regex = matches.value_of("REGEX").unwrap();
        let regex = Regex::new(regex).expect("Invalid regex");

        match matches.value_of("OUTPUT") {
            Some("-") => mark(records, &regex, stdout)?,

            Some(output_file_name) => {
                let output_file = File::create(output_file_name)?;
                mark(records, &regex, output_file)?;
            }
            _ => {
                let output_file_name = format!(".{}.tmp", input_file_name);
                let output_file = File::create(&output_file_name)?;
                mark(records, &regex, output_file)?;
                fs::rename(output_file_name, input_file_name)?;
            }
        };
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
        .subcommand(
            SubCommand::with_name("mark")
                .about("mark matches")
                .arg_from_usage("<REGEX> -r --regex=<REGEX> 'regular expression'")
                .arg_from_usage("[OUTPUT] -o --output=[FILE] 'output file'")
                .arg_from_usage("<INPUT> 'input file'"),
        )
}

fn mark(records: impl Iterator<Item = Record>, regex: &Regex, mut out: impl Write) -> Result<()> {
    for mut r in records {
        r.add_match(regex);
        write_json(r, &mut out)?;
    }
    Ok(())
}

/// iterate over records and write to output only matches of given regex
fn print(records: impl Iterator<Item = Record>, regex: &Regex, mut out: impl Write) -> Result<()> {
    for record in records {
        let text = record.text;

        for m in regex.find_iter(&text) {
            writeln!(out, "{}", m.as_str())?;
        }
    }

    Ok(())
}

/// Read over lines and writes to ouput in Record format
fn import(input: impl Iterator<Item = String>, mut out: impl Write) -> Result<()> {
    for line in input {
        let record = Record {
            text: line.to_string(),
            spans: vec![],
        };
        write_json(record, &mut out)?;
    }

    Ok(())
}

fn write_json<T: Serialize>(value: T, mut out: impl Write) -> Result<()> {
    let bytes = serde_json::to_vec(&value)?;
    out.write_all(&bytes)?;
    writeln!(out)?;

    Ok(())
}
