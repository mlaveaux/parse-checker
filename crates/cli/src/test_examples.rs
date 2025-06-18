#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case(include_str!("../../../examples/red_can_win.mcf"), include_str!("../snapshot/red_can_win.mcf") ; "red_can_win.mcf")]
    fn test_example(input: &str, expected: &str) {
        let current_ast = mcrl2_sys::ffi::print_ast_mcf(input).expect("Failed to print AST for the current version.");

        assert_eq!(current_ast.trim(), expected.trim(), "The pretty printed AST does not match the expected output.");
    }
}