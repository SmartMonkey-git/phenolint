use crate::LinterContext;
use crate::error::FromContextError;
use crate::patches::traits::RegisterablePatch;
use inventory;

pub struct PatchRegistration {
    pub rule_id: &'static str,
    pub factory:
        fn(context: &LinterContext) -> Result<Box<dyn RegisterablePatch>, FromContextError>,
}

inventory::collect!(PatchRegistration);
