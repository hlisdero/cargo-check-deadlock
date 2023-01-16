//! Module that define the naming of places and transitions in the Petri net.
//!
//! The functions are grouped according to the structure they deal with, e.g., function,
//! basic block, statement, mutex, etc.
//! Functions used by several submodules should be defined here.
//!
//! All functions listed here should have an `#[inline]` attribute for performance reasons.
//! See the reference for more information:
//! <https://doc.rust-lang.org/stable/reference/attributes/codegen.html>

pub mod basic_block;
pub mod function;
pub mod mutex;
pub mod program;
pub mod statement;
pub mod thread;

/// Sanitize the function name for the DOT format:
/// - Replace generic types "<T>" with "T".
/// - Replace double colons with underscores.
/// - Replace curly braces with underscores.
/// - Replace pound sign with underscores.
#[inline]
fn sanitize(function_name: &str) -> String {
    function_name
        .replace("<T>", "T")
        .replace("::", "_")
        .replace(['{', '}', '#'], "_")
}
