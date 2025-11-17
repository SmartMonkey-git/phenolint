use crate::patches::enums::PatchInstruction;

#[derive(Debug, Default, PartialEq)]
pub struct Patch {
    pub instructions: Vec<PatchInstruction>,
}

impl Patch {
    pub fn new(instructions: Vec<PatchInstruction>) -> Self {
        Self { instructions }
    }
}
