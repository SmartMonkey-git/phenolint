use crate::patches::enums::PatchInstruction;

#[derive(Debug, Default)]
pub struct Patch {
    pub instructions: Vec<PatchInstruction>,
}
