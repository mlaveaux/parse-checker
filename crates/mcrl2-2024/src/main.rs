//! The main command line tool to check for mCRL2 specification and modal formula parsing differences.

use std::io::{self, Read};
use std::process::ExitCode;
use std::error::Error;

use clap::Parser;

#[derive(Parser)]
#[command()]
#[command(name = "mcrl2-2024",
    author = "Maurice Laveaux",
    version,
    about = "Internal tool used to print the 2024 AST, use parse-checker instead!")]
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
