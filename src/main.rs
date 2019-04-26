use docopt::Docopt;
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_writer_pretty};
use std::error::Error;
use std::fs::File;

const USAGE: &str = "
Add.

Usage:
    add <input> <output>
    add (-h | --help)

Options:
  -h --help     Show this screen.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_input: String,
    arg_output: String,
}

#[derive(Debug, Deserialize)]
struct InputFormat {
    x: f64,
    y: f64,
}

#[derive(Debug, Serialize)]
struct OutputFormat {
    sum: f64,
}

fn main() -> Result<(), Box<Error>> {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    let input = File::open(args.arg_input)?;
    let input: InputFormat = from_reader(input)?;
    let output = File::create(args.arg_output)?;
    to_writer_pretty(
        output,
        &OutputFormat {
            sum: input.x + input.y,
        },
    )?;
    Ok(())
}
