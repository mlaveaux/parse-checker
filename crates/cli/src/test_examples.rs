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
    #[test_case(include_str!("../../../examples/incorrect/Eventual access.mcf"), include_str!("../snapshot/Eventual access.mcf") ; "Eventual access.mcf")]
    #[test_case(include_str!("../../../examples/incorrect/eventually_comm.mcf"), include_str!("../snapshot/eventually_comm.mcf") ; "eventually comm.mcf")]
    #[test_case(include_str!("../../../examples/incorrect/minimal_walking_distance.mcf"), include_str!("../snapshot/minimal_walking_distance.mcf") ; "minimal_walking_distance.mcf")]
    #[test_case(include_str!("../../../examples/incorrect/mutual exclusion.mcf"), include_str!("../snapshot/mutual exclusion.mcf") ; "mutual exclusion.mcf")]
    #[test_case(include_str!("../../../examples/incorrect/prop9.mcf"), include_str!("../snapshot/prop9.mcf") ; "prop9.mcf")]
    #[test_case(include_str!("../../../examples/incorrect/Reachable.mcf"), include_str!("../snapshot/Reachable.mcf") ; "Reachable.mcf")]
    #[test_case(include_str!("../../../examples/incorrect/red_can_win.mcf"), include_str!("../snapshot/red_can_win.mcf") ; "red_can_win.mcf")]
    #[test_case(include_str!("../../../examples/incorrect/request_can_eventually_enter.mcf"), include_str!("../snapshot/request_can_eventually_enter.mcf") ; "request_can_eventually_enter.mcf")]
    #[test_case(include_str!("../../../examples/incorrect/rule4.mcf"), include_str!("../snapshot/rule4.mcf") ; "rule4.mcf")]
    #[test_case(include_str!("../../../examples/incorrect/rule789.mcf"), include_str!("../snapshot/rule789.mcf") ; "rule789.mcf")]
    #[test_case(include_str!("../../../examples/incorrect/starvation freemdom.mcf"), include_str!("../snapshot/starvation freemdom.mcf") ; "starvation freemdom.mcf")]
    #[test_case(include_str!("../../../examples/incorrect/white_can_win.mcf"), include_str!("../snapshot/white_can_win.mcf") ; "white_can_win.mcf")]
    fn test_incorrect_example(input: &str, expected: &str) {
        let current_ast = mcrl2_sys::print_ast_mcf(input).expect("Failed to print AST for the current version.");
        assert_eq!(current_ast.trim(), expected.trim(), "The pretty printed AST does not match the expected output.");

        let previous_ast = print_ast_2024(input, true).expect("Failed to print AST for the 2024 version.");
        assert_ne!(current_ast.trim(), previous_ast.trim(), "For the incorrect example, the ASTs should differ between the two versions.");
    }
}