use crate::patches::enums::PatchInstruction;

#[derive(Debug, Default)]
#[allow(unused)]
pub struct Patch {
    actions: Vec<PatchInstruction>,
}
