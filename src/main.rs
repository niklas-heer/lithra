//! Command-line entry point for Lithra.

use std::io::{self, Write};
use std::process::ExitCode;

const USAGE: &str = "\
Reproducible systems and images, described in a language you can read.

Usage: lithra [OPTIONS]

Options:
  -h, --help     Print help
  -V, --version  Print version
";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .as_slice()
    {
        [] | ["-h" | "--help"] => print(USAGE),
        ["-V" | "--version"] => print(&format!("lithra {}\n", env!("CARGO_PKG_VERSION"))),
        [unknown, ..] => {
            let _ = writeln!(
                io::stderr(),
                "error: unexpected argument '{unknown}'\n\n{USAGE}"
            );
            ExitCode::from(2)
        }
    }
}

fn print(text: &str) -> ExitCode {
    match io::stdout().write_all(text.as_bytes()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}
