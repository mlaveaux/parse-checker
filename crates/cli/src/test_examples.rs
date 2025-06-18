#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::print_ast_2024;

    #[test_case(include_str!("../../../examples/incorrect/Always eventually request_alt.mcf"), include_str!("../snapshot/Always eventually request_alt.mcf") ; "Always eventually request_alt.mcf")]
    #[test_case(include_str!("../../../examples/incorrect/Always eventually request.mcf"), include_str!("../snapshot/Always eventually request.mcf") ; "Always eventually request.mcf")]
    #[test_case(include_str!("../../../examples/incorrect/Bounded overtaking.mcf"), include_str!("../snapshot/Bounded overtaking.mcf") ; "Bounded overtaking.mcf")]
    #[test_case(include_str!("../../../examples/incorrect/Correctness.mcf"), include_str!("../snapshot/Correctness.mcf") ; "Correctness.mcf")]
    #[test_case(include_str!("../../../examples/incorrect/Eventual access if fair.mcf"), include_str!("../snapshot/Eventual access if fair.mcf") ; "Eventual access if fair.mcf")]
    #[test_case(include_str!("../../../examples/incorrect/Eventual access without coorperation.mcf"), include_str!("../snapshot/Eventual access without coorperation.mcf") ; "Eventual access without coorperation.mcf")]
    #[test_case(include_str!("../../../examples/incorrect/red_can_win.mcf"), include_str!("../snapshot/red_can_win.mcf") ; "red_can_win.mcf")]
    fn test_incorrect_example(input: &str, expected: &str) {
        let current_ast = mcrl2_sys::ffi::print_ast_mcf(input).expect("Failed to print AST for the current version.");
        assert_eq!(current_ast.trim(), expected.trim(), "The pretty printed AST does not match the expected output.");

        let previous_ast = print_ast_2024(input, true, false).expect("Failed to print AST for the current version.");
        assert_ne!(current_ast.trim(), previous_ast.trim(), "For the incorrect example, the ASTs should differ between the two versions.");
    }
}