//!
//! This crate provides the raw Rust bindings for the libraries of the
//! [mCRL2](https://mcrl2.org/) toolset.
//!
//! Every module mirrors the corresponding library of the mCRL2 toolset. Within
//! it a foreign function interface (FFI) is defined using the
//! [cxx](https://cxx.rs/) crate.
//! 

use std::sync::LazyLock;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("mcrl2-sys/cpp/print_ast.h");


        /// Prints the input mCRL2 specification as an abstract syntax tree (AST).
        fn print_ast_mcrl2(input: &str) -> Result<String>;

        /// Prints the input MCF specification as an abstract syntax tree (AST).
        fn print_ast_mcf(input: &str) -> Result<String>;
    }
}

static MCRL2_LOCK: LazyLock<std::sync::Mutex<()>> = LazyLock::new(|| std::sync::Mutex::new(()));

/// Prints the input mCRL2 specification as an abstract syntax tree (AST).
pub fn print_ast_mcrl2(input: &str) -> Result<String, cxx::Exception> {
    let _guard = MCRL2_LOCK.lock().unwrap();
    ffi::print_ast_mcrl2(input)
}

/// Prints the input MCF specification as an abstract syntax tree (AST).
pub fn print_ast_mcf(input: &str) -> Result<String, cxx::Exception> {
    let _guard = MCRL2_LOCK.lock().unwrap();
    ffi::print_ast_mcf(input)
}