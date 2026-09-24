//! Command-line entry point for Lithra.

use clap::Parser;

/// Reproducible systems and images, described in a language you can read.
#[derive(Parser)]
#[command(version)]
struct Cli {}

fn main() {
    let Cli {} = Cli::parse();
    println!("{}", lithra::version_line());
}
