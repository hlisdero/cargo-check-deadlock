/// Format the error string used every time that an arc is created.
#[inline]
pub fn format_err_str_add_arc(start_place_name: &str, end_place_name: &str) -> String {
    format!(
        "BUG: Adding an arc from the {start_place_name} to the {end_place_name} should not fail"
    )
}
