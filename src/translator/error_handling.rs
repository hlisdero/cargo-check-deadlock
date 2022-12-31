/// Error message when the call stack in the translator is empty.
pub const EMPTY_CALL_STACK: &str =
    "BUG: The call stack should contain a function for the MIR visitor methods to work";

/// Format the error string used every time that an arc is created.
#[inline]
pub fn format_err_str_add_arc(start_place_name: &str, end_place_name: &str) -> String {
    format!(
        "BUG: Adding an arc from the {start_place_name} to the {end_place_name} should not fail"
    )
}
