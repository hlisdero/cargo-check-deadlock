//! Submodule that defines the naming of basic places in the Petri net.
//! These places are present in every generated Petri net.

/// Label of the place that models the program start state.
pub const PROGRAM_START: &str = "PROGRAM_START";
/// Label of the place that models the normal program end state.
pub const PROGRAM_END: &str = "PROGRAM_END";
/// Label of the place that models the program end state after a `panic!`.
pub const PROGRAM_PANIC: &str = "PROGRAM_PANIC";
