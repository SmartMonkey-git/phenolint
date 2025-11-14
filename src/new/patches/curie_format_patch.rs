use crate::LinterContext;
use crate::diagnostics::LintViolation;
use crate::enums::Patch;
use crate::error::RuleInitError;
use crate::new::node::Node;
use crate::new::patches::patch_registry::{PatchFromContext, RegisterablePatch};
use crate::new::traits::CompilePatches;
use crate::new::traits::RulePatch;
use phenolint_macros::register_patch;

#[register_patch(id = "CURIE001")]
struct CurieFormatPatch;

impl PatchFromContext for CurieFormatPatch {
    fn from_context(context: &LinterContext) -> Result<Box<dyn RegisterablePatch>, RuleInitError> {
        Ok(Box::new(CurieFormatPatch))
    }
}

impl CompilePatches for CurieFormatPatch {
    fn compile_patches(&self, value: &Node, lint_violation: &LintViolation) -> Vec<Patch> {
        println!("Reached compilation of CurieFormatPatch");
        vec![]
    }
}
