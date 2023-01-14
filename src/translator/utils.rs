//! Submodule for miscellaneous utility functions.

/// Convert the `Place` directly to a `Local`.
/// <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/syntax/struct.Place.html#method.as_local>
///
/// # Panics
///
/// If `place` is not a simple local variable without projections, then the function panics.
/// <https://rustc-dev-guide.rust-lang.org/mir/index.html#mir-data-types>
pub fn place_to_local(place: &rustc_middle::mir::Place) -> rustc_middle::mir::Local {
    place
        .as_local()
        .expect("BUG: The place should be a local variable with no projections")
}
