//! Module that defines the naming of places and transitions in the Petri net.
//!
//! The functions are grouped according to the structure they deal with, e.g., function,
//! basic block, statement, mutex, etc.
//! Functions used by several submodules should be defined here.
//!
//! All functions listed here should have an `#[inline]` attribute for performance reasons.
//! See the reference for more information:
//! <https://doc.rust-lang.org/stable/reference/attributes/codegen.html>

pub mod basic_block;
pub mod condvar;
pub mod function;
pub mod mutex;
pub mod program;
pub mod thread;

/// Sanitize the function name for the DOT format:
/// - Replace generic types "<T>" with "T".
/// - Replace lifetimes "'a" with simply "a".
/// - Replace double colons with underscores.
/// - Replace curly braces with underscores.
/// - Replace pound sign with underscores.
/// - Replace great-than and less-than sign with underscores.
/// - Replace spaces with underscores.
#[inline]
fn sanitize(function_name: &str) -> String {
    function_name
        .replace("<T>", "T")
        .replace("[T]", "T")
        .replace("<T, A>", "T_A")
        .replace("<'a>", "a")
        .replace("::", "_")
        .replace("Result_<T, E>", "Result")
        .replace(['{', '}', '[', ']', '#', '<', '>', ' '], "_") // Catch-all case
}
