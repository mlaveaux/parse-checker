//! The main command line tool to check for mCRL2 specification and modal formula parsing differences.

#![forbid(unsafe_code)]

use std::fmt::Write;
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
#[command(about = "A tool that can be used to check whether mCRL2 specifications or modal formulas parse differently between the 202407.0 and 202507.0 release.")]
struct Cli {
    /// The path for the file to check, 
    input: String,

    /// Whether to check mCRL2 specifications (default) or modal formulas.
    #[arg(long)]
    mcf: bool,

    /// Prints the parse tree of the input file.
    #[arg(long)]
    print: bool,

    /// Prints the parse tree of the input file using the 2024 release of mCRL2.
    #[arg(long)]
    print_2024: bool,

    /// Prints the parse tree indented (whenever it is printed).
    #[arg(short, long)]
    indented: bool,
}

fn main() -> Result<ExitCode, Box<dyn Error>> {
    let mut cli = Cli::parse();

    let input_path = Path::new(&cli.input);
    if !input_path.exists() {
        return Err(format!("Cannot find file {}", cli.input))?;
    }

    if let Some(ext) = input_path.extension() {
        // Detect input format, otherwise use the as specificed by the user.
        if  ext == "mcf" {
            // If the file has a .mcf extension, we assume it's a modal formula. This is also what the toolset does.
            cli.mcf = true;
        } else if ext == "mcrl2" {
            cli.mcf = false;
        }
    }

    let input = fs::read_to_string(input_path)?;

    if cli.print || cli.print_2024 {
        let ast = print(&cli, &input).map_err(|e| format!("Error: {}", e))?;

        if cli.indented {
            print!("{}", PrintIndented(&ast));
        } else {
            print!("{}", ast);
        }
    }

    
    if cli.mcf {
        diff_mcf(&input)?;
    } else {
        // Default to checking mCRL2 specifications
        diff_mcrl2(&input)?;
    }

    Ok(ExitCode::SUCCESS)
}

fn print(cli: &Cli, input: &str) -> Result<String, Box<dyn Error>> {    
    // If the user wants to print the parse tree, print it depending on the specified options.
    if cli.print {
        if cli.mcf {
            Ok(mcrl2_sys::print_ast_mcf(&input)?)
        } else {
            Ok(mcrl2_sys::print_ast_mcrl2(&input)?)
        }
    } else {
        print_ast_2024(&input, cli.mcf)
    }
}

struct PrintIndented<'a>(&'a str);

impl std::fmt::Display for PrintIndented< '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        print_indented(f, self.0)
    }
}

fn print_indented(writer: &mut impl Write, s: &str) -> Result<(), std::fmt::Error> {
    let mut indent_level = 0;
    let mut chars = s.chars().peekable();
    
    while let Some(ch) = chars.next() {
        match ch {
            '(' => {
                write!(writer, "{}", ch)?;
                indent_level += 1;
                writeln!(writer)?;
                write!(writer, "{}", "  ".repeat(indent_level))?;
            }
            ')' => {
                if indent_level > 0 {
                    indent_level -= 1;
                }
                writeln!(writer)?;
                write!(writer, "{}{}", "  ".repeat(indent_level), ch)?;
            }
            '\n' | '\r' => {
                // Skip existing newlines to avoid double spacing
            }
            _ => {
                write!(writer, "{}", ch)?;
            }
        }
    }
    writeln!(writer)?;
    Ok(())
}