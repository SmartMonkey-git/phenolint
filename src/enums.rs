#[derive(Clone, Debug)]
pub enum Patch {
    Add {
        at: String,
        value: String,
    },
    Remove {
        at: String,
    },
    ///TODO: Unfolds to remove and add
    Move {
        from: String,
        to: String,
    },
    ///TODO: Converts to add
    Duplicate {
        from: String,
        to: String,
    },
}
