#pragma once

#include "rust/cxx.h"

#include "mcrl2/lps/parse.h"
#include "mcrl2/process/process_specification.h"
#include "mcrl2/modal_formula/parse.h"
#include "mcrl2/modal_formula/state_formula_specification.h"

#include <iostream>

inline
rust::String print_ast_mcrl2(rust::Str text) {
    mcrl2::process::process_specification spec = mcrl2::process::parse_process_specification(static_cast<std::string>(text));

    std::stringstream result;
    result << mcrl2::process::pp(spec, false) << std::endl;
    return result.str();
}

inline
rust::String print_ast_mcf(rust::Str text) {
    mcrl2::lps::stochastic_specification lpsspec;

    mcrl2::state_formulas::parse_state_formula_options options;
    options.type_check = false;
    options.translate_regular_formulas = false;
    options.translate_user_notation = false;
    options.resolve_name_clashes = false;
    options.check_monotonicity = false;
    mcrl2::state_formulas::state_formula_specification formspec = mcrl2::state_formulas::parse_state_formula_specification(static_cast<std::string>(text), lpsspec, false, options);

    std::stringstream result;
    result << mcrl2::state_formulas::pp(formspec, false) << std::endl;
    return result.str();
}

inline
rust::String print_ast_quantitative_mcf(rust::Str text) {
    mcrl2::lps::stochastic_specification lpsspec;

    mcrl2::state_formulas::parse_state_formula_options options;
    options.type_check = false;
    options.translate_regular_formulas = false;
    options.translate_user_notation = false;
    options.resolve_name_clashes = false;
    options.check_monotonicity = false;
    mcrl2::state_formulas::state_formula_specification formspec = mcrl2::state_formulas::parse_state_formula_specification(static_cast<std::string>(text), lpsspec, true, options);

    std::stringstream result;
    result << mcrl2::state_formulas::pp(formspec, false) << std::endl;
    return result.str();
}