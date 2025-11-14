#![allow(dead_code)]
use crate::patches::patch_registry::PatchRegistry;
use inventory;

pub struct PatchRegistration {
    pub(crate) rule_id: &'static str,
    pub(crate) register: fn(&mut PatchRegistry),
}

inventory::collect!(PatchRegistration);
