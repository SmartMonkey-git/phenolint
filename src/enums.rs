use crate::rules::utils::json_cursor::Pointer;

#[derive(Clone, Debug)]
pub enum Patch {
    Add {
        at: Pointer,
        value: String,
    },
    Remove {
        at: Pointer,
    },
    ///TODO: Unfolds to remove and add
    Move {
        from: Pointer,
        to: Pointer,
    },
    ///TODO: Converts to add
    Duplicate {
        from: Pointer,
        to: Pointer,
    },
}
