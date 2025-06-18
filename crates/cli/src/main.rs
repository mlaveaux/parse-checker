//! The main command line tool to check for mCRL2 specification and modal formula parsing differences.


use std::fs;
use std::path::Path;
use std::process::ExitCode;
use std::error::Error;

use clap::Parser;

// Import the other modules.
mod test_examples;
mod diff;

pub use diff::*;

#[derive(Parser)]
#[command(name = "parse-checker")]
#[command(about = "A tool that can be used to check whether mCRL2 specifications or modal formulas parse differently between the 2024 and 2025 release.")]
struct Cli {
    /// The path for the file to check.
    input: String,

    /// Whether to check mCRL2 specifications (default) or modal formulas.
    #[arg(short, long)]
    mcf: bool,
}

fn main() -> Result<ExitCode, Box<dyn Error>> {
    let cli = Cli::parse();

    let input_path = Path::new(&cli.input);
    if !input_path.exists() {
        return Err(format!("Cannot find file {}", cli.input))?;
    }

    let input = fs::read_to_string(input_path)?;

    if cli.mcf {
        diff_mcf(&input)?;
    } else {
        // Default to checking mCRL2 specifications
        diff(&input)?;
    }

    Ok(ExitCode::SUCCESS)
}
