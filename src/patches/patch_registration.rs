#![allow(dead_code)]
use crate::LinterContext;
use crate::error::FromContextError;
use crate::patches::traits::RegisterablePatch;
use inventory;

pub struct PatchRegistration {
    pub(crate) rule_id: &'static str,
    pub(crate) factory:
        fn(context: &LinterContext) -> Result<Box<dyn RegisterablePatch>, FromContextError>,
}

inventory::collect!(PatchRegistration);
