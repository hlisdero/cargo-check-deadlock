/// Error message when the call stack in the translator is empty.
pub const EMPTY_CALL_STACK: &str =
    "BUG: The call stack should contain a function for the MIR visitor methods to work";

/// Handles the error case when adding an arc. Panic with a custom error message.
/// This error points to an error in the library or in the code and should not occur.
pub fn handle_err_add_arc(start_place_name: &str, end_place_name: &str) {
    panic!("BUG: Adding an arc from the {start_place_name} to the {end_place_name} should not fail")
}
