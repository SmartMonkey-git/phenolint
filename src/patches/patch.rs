use crate::helper::NonEmptyVec;
use crate::patches::enums::PatchInstruction;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Patch {
    instructions: Vec<PatchInstruction>,
}

impl Patch {
    pub fn new(instructions: NonEmptyVec<PatchInstruction>) -> Self {
        Self {
            instructions: instructions.into_vec(),
        }
    }

    pub fn instructions(&self) -> &[PatchInstruction] {
        &self.instructions
    }
}
