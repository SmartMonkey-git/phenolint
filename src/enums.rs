use crate::json::Pointer;

#[derive(Debug, Default)]
pub struct Patch {
    rule_id: String,
    actions: Vec<PatchAction>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PatchAction {
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
