#[derive(Hash, Clone, Eq, PartialEq)]
pub struct Local {}

impl Local {
    #[must_use]
    pub fn new() -> Self {
        Local {}
    }
}
