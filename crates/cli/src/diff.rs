
use duct::cmd;
use std::error::Error;
use console::Style;
use similar::ChangeTag;
use similar::TextDiff;

/// Prints the AST of an mCRL2 specification or modal formula.
pub fn print_ast_2024(input: &str, mcf: bool, quantitative: bool) -> Result<String, Box<dyn Error>> {
    if !mcf && quantitative {
        return Err("quantitative is only applicable for modal formulas.".into());
    }

    let mcrl2_path = which::which("mcrl2-2024")?;

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

    if quantitative {
        arguments.push("--quantitative".into());
    }

    let tool = cmd(mcrl2_path, arguments)
        .stdin_bytes(input)
        .stderr_capture()
        .stdout_capture()
        .run()?;

    Ok(String::from_utf8(tool.stdout)?)
}

fn print_diff(left: &str, right: &str) -> String {
    let diff = TextDiff::from_chars(left, right);
    let mut output = String::new();

    for op in diff.ops() {
        for change in diff.iter_changes(op) {
            let (sign, style) = match change.tag() {
                ChangeTag::Delete => ("-", Style::new().red()),
                ChangeTag::Insert => ("+", Style::new().green()),
                ChangeTag::Equal => (" ", Style::new()),
            };
            output.push_str(&format!("{}{}\n", style.apply_to(sign).bold(), style.apply_to(change)));
        }
    }

    output
}

/// Compare the ASTs of mCRL2 specifications or modal formulas between two versions.
pub fn diff_mcrl2(input: &str) -> Result<(), Box<dyn Error>> {
    let current_ast = mcrl2_sys::ffi::print_ast_mcrl2(input)?;
    let previous_ast = print_ast_2024(input, false, false)?;

    if current_ast != previous_ast {
        print_diff(&current_ast,  &previous_ast);

        Err("The ASTs of the mCRL2 specifications differ between the two versions.")?;
    }

    Ok(())
}

/// Compare the ASTs of mCRL2 specifications or modal formulas between two versions.
pub fn diff_mcf(input: &str, quantitative: bool) -> Result<(), Box<dyn Error>> {
    let current_ast = mcrl2_sys::ffi::print_ast_mcf(input)?;
    let previous_ast = print_ast_2024(input, true, quantitative)?;

    if current_ast != previous_ast {
        print_diff(&current_ast,  &previous_ast);

        Err("The ASTs of the modal formula specifications differ between the two versions.")?;
    }

    Ok(())
}
