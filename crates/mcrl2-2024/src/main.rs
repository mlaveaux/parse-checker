//! The main command line tool to check for mCRL2 specification and modal formula parsing differences.

use std::io::{self, Read};
use std::process::ExitCode;
use std::error::Error;

use clap::Parser;

#[derive(Parser)]
#[command(name = "mcrl2")]
#[command(about = "A tool that can be used to check whether mCRL2 specifications or modal formulas parse differently between the 2024 and 2025 release.")]
struct Cli {
    /// Whether to check mCRL2 specifications (default) or modal formulas.
    #[arg(short, long, default_value_t = false)]
    mcf: bool,
}

fn main() -> Result<ExitCode, Box<dyn Error>> {
    let cli = Cli::parse();

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    if cli.mcf {
        print!("{}", mcrl2_2024_sys::ffi::print_ast_mcf(&input)?);
    } else {
        // Default to checking mCRL2 specifications
        print!("{}", mcrl2_2024_sys::ffi::print_ast_mcrl2(&input)?);
    }

    Ok(ExitCode::SUCCESS)
}
