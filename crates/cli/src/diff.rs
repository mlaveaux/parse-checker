
use duct::cmd;
use std::error::Error;
use std::io::Write;
use std::io::stdout;
use console::Style;
use similar::ChangeTag;
use similar::TextDiff;

/// Prints the AST of an mCRL2 specification or modal formula.
pub fn print_ast_2024(input: &str, mcf: bool) -> Result<String, Box<dyn Error>> {    
    let mcrl2_path = which::which("mcrl2-2024")
        .or_else(|_| {
            // Try to find the executable in the same directory as the current executable
            std::env::current_exe()
                .map_err(|_e| which::Error::CannotFindBinaryPath)
                .and_then(|mut path| {
                    path.pop(); // Remove the executable name
                    
                    // Try removing "deps" directory if it's the last component
                    if path.file_name().and_then(|s| s.to_str()) == Some("deps") {
                        path.pop();
                    }

                    println!("{:?}", path);
                    path.push(if cfg!(windows) { "mcrl2-2024.exe" } else { "mcrl2-2024" });
                    if path.exists() {
                        Ok(path)
                    } else {
                        Err(which::Error::CannotFindBinaryPath)
                    }
                })
        })?;


    // Check if the executables exist
    if !mcrl2_path.exists() {
        return Err(format!(
            "Cannot find mcrl2-2024 executable at {}",
            mcrl2_path.display()
        )
        .into());
    }

    let mut arguments: Vec<String> = Vec::new();
    if mcf {
        arguments.push("--mcf".into());
    }

    let tool = cmd(mcrl2_path, arguments)
        .stdin_bytes(input)
        .stderr_capture()
        .stdout_capture()
        .run()?;

    Ok(String::from_utf8(tool.stdout)?)
}

fn print_diff(f: &mut impl Write, left: &str, right: &str) -> std::io::Result<()> {
    let diff = TextDiff::from_words(left, right);

    for op in diff.ops() {
        for change in diff.iter_changes(op) {
            let (sign, style) = match change.tag() {
                ChangeTag::Delete => ("-", Style::new().red()),
                ChangeTag::Insert => ("+", Style::new().green()),
                ChangeTag::Equal => (" ", Style::new()),
            };

            write!(f, "{}{}", style.apply_to(sign).bold(), style.apply_to(change))?;
        }
    }

    Ok(())
}

/// Compare the ASTs of mCRL2 specifications or modal formulas between two versions.
pub fn diff_mcrl2(input: &str) -> Result<(), Box<dyn Error>> {
    let current_ast = mcrl2_sys::print_ast_mcrl2(input)?;
    let previous_ast = print_ast_2024(input, false)?;

    if current_ast != previous_ast {
        print_diff( &mut stdout(), &current_ast,  &previous_ast)?;

        Err("The ASTs of the mCRL2 specifications differ between the two versions.")?;
    }

    Ok(())
}

/// Compare the ASTs of mCRL2 specifications or modal formulas between two versions.
pub fn diff_mcf(input: &str) -> Result<(), Box<dyn Error>> {
    let current_ast = mcrl2_sys::print_ast_mcf(input)?;
    let previous_ast = print_ast_2024(input, true)?;

    if current_ast != previous_ast {
        print_diff(&mut stdout(), &current_ast,  &previous_ast)?;

        Err("The ASTs of the modal formula specifications differ between the two versions.")?;
    }

    Ok(())
}
