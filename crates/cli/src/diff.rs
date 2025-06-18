
use duct::cmd;
use std::error::Error;
use console::Style;
use similar::ChangeTag;
use similar::TextDiff;

/// Prints the AST of an mCRL2 specification or modal formula.
pub fn print_ast_2024(input: &str, mcf: bool) -> Result<String, Box<dyn Error>> {
    let mcrl2_path = which::which("mcrl2-2024")?;

    // Check if the executables exist
    if !mcrl2_path.exists() {
        return Err(format!(
            "Cannot find mcrl2-2024 executable at {}",
            mcrl2_path.display()
        )
        .into());
    }

    let tool = cmd!(mcrl2_path, input, if mcf { "--mcf" } else { "" })
        .stderr_capture()
        .stdout_capture()
        .run()?;

    Ok(String::from_utf8(tool.stdout)?)
}

/// Compare the ASTs of mCRL2 specifications or modal formulas between two versions.
pub fn diff(input: &str) -> Result<(), Box<dyn Error>> {
    let current_ast = mcrl2_sys::ffi::print_ast_mcrl2(input)?;
    let previous_ast = print_ast_2024(input, false)?;

    if current_ast != previous_ast {
        let diff = TextDiff::from_chars(
            &current_ast,
            &previous_ast,
        );

        for op in diff.ops() {
            for change in diff.iter_changes(op) {
                let (sign, style) = match change.tag() {
                    ChangeTag::Delete => ("-", Style::new().red()),
                    ChangeTag::Insert => ("+", Style::new().green()),
                    ChangeTag::Equal => (" ", Style::new()),
                };
                print!("{}{}", style.apply_to(sign).bold(), style.apply_to(change));
            }
        }

        Err("The ASTs of the mCRL2 specifications differ between the two versions.")?;
    }

    Ok(())
}

/// Compare the ASTs of mCRL2 specifications or modal formulas between two versions.
pub fn diff_mcf(input: &str) -> Result<(), Box<dyn Error>> {
    let current_ast = mcrl2_sys::ffi::print_ast_mcf(input)?;
    let previous_ast = print_ast_2024(input, true)?;

    if current_ast != previous_ast {
        let diff = TextDiff::from_chars(
            &current_ast,
            &previous_ast,
        );

        for op in diff.ops() {
            for change in diff.iter_changes(op) {
                let (sign, style) = match change.tag() {
                    ChangeTag::Delete => ("-", Style::new().red()),
                    ChangeTag::Insert => ("+", Style::new().green()),
                    ChangeTag::Equal => (" ", Style::new()),
                };
                print!("{}{}", style.apply_to(sign).bold(), style.apply_to(change));
            }
        }

        Err("The ASTs of the modal formula specifications differ between the two versions.")?;
    }

    Ok(())
}
