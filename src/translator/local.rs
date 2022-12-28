#[derive(Hash, Clone, Eq, PartialEq)]
pub struct Local {}

impl Local {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
