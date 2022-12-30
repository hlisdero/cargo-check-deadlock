//! Auxiliary structure to keep track of the local variables in the code.
//!
//! The virtual memory stores a mapping between the `rustc_middle::mir::Local`
//! and the `Local` defined in our code.
//! This is needed to know if a variable contains a mutex.
//!
//! More info:
//! <https://rustc-dev-guide.rust-lang.org/mir/index.html#mir-data-types>
use crate::translator::local::Local;
use std::collections::HashMap;

/// TODO: Local is a an mere index to the vector in body. This is not unique!!!
/// <https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/mir/struct.Body.html#structfield.local_decls>
#[derive(Default)]
pub struct VirtualMemory {
    locals: HashMap<rustc_middle::mir::Local, Local>,
}

impl VirtualMemory {
    /// Creates a new virtual memory with an empty mapping.
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets the local variable structure corresponding to the MIR local.
    pub fn get_local_variable(&self, local: &rustc_middle::mir::Local) -> Option<&Local> {
        self.locals.get(&local)
    }
}
