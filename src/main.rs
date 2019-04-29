use docopt::Docopt;
use serde::Deserialize;

const USAGE: &str = "
Add.

Adds two numbers <x> and <y>.

Usage:
    add <x> <y>
    add (-h | --help)

Options:
  -h --help     Show this screen.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_x: f64,
    arg_y: f64,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    println!("{}", args.arg_x + args.arg_y);
}
