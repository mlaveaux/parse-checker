//!
//! `xtask` is a crate that can be used to enable `make`-like commands in cargo. These commands are then implemented in Rust.
//!

#![forbid(unsafe_code)]

use std::env;
use std::error::Error;
use std::process::ExitCode;

mod package;

fn main() -> Result<ExitCode, Box<dyn Error>> {
    let mut args = env::args();

    // Ignore the first argument (which should be xtask)
    args.next();

    // The name of the task
    let task = args.next();

    match task.as_deref() {
        Some("package") => package::package()?,
        Some(x) => {
            println!("Unknown task {x}");
            println!();
            print_help();
            return Ok(ExitCode::FAILURE);
        }
        _ => print_help(),
    }

    Ok(ExitCode::SUCCESS)
}

/// Print the help message.
fn print_help() {
    println!(
        "Available tasks: package"
    );
}
