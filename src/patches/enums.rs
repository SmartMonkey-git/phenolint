#![allow(dead_code)]

use crate::tree::pointer::Pointer;
use serde_json::{Value, from_value, json};

#[derive(Clone, Debug, PartialEq)]
pub enum PatchInstruction {
    Add { at: Pointer, value: Value },
    Remove { at: Pointer },
    Move { from: Pointer, to: Pointer },
    Duplicate { from: Pointer, to: Pointer },
}

impl PatchInstruction {
    pub fn to_json_patch(&self) -> json_patch::Patch {
        match self {
            PatchInstruction::Add { at, value } => {
                from_value(json!([{ "op": "add", "path": at.position(), "value": value }]))
                    .expect("Could not parse patch")
            }
            PatchInstruction::Remove { at } => {
                from_value(json!([{ "op": "remove", "path": at.position() }]))
                    .expect("Could not parse patch")
            }
            PatchInstruction::Move { from, to } => from_value(
                json!([{ "op": "move", "path": to.position(), "from": from.position() }]),
            )
            .expect("Could not parse patch"),
            PatchInstruction::Duplicate { from, to } => from_value(
                json!([{ "op": "move", "path": to.position(), "from": from.position() }]),
            )
            .expect("Could not parse patch"),
        }
    }
}
