use crate::patches::enums::PatchInstruction;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Patch {
    pub instructions: Vec<PatchInstruction>,
}

impl Patch {
    pub fn new(instructions: Vec<PatchInstruction>) -> Self {
        Self { instructions }
    }
}
