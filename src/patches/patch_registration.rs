#![allow(dead_code)]
use crate::LinterContext;
use crate::error::{FromContextError, InitError};
use crate::patches::patch_registry::PatchRegistry;
use crate::patches::traits::{CompilePatches, RegisterablePatch};
use inventory;

pub struct PatchRegistration {
    pub(crate) rule_id: &'static str,
    pub(crate) factory:
        fn(context: &LinterContext) -> Result<Box<dyn RegisterablePatch>, FromContextError>,
}

inventory::collect!(PatchRegistration);
