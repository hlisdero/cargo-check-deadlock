//! Submodule for error handling functions and messages.

/// Handles the error case when adding an arc. Panic with a custom error message.
/// This error points to an error in the Petri net library or in the code and should not occur.
#[inline]
pub fn handle_err_add_arc(start_place_name: &str, end_place_name: &str) {
    panic!("BUG: Adding an arc from the {start_place_name} to the {end_place_name} should not fail")
}
