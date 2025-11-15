#![allow(unused)]
use crate::LinterContext;
use crate::diagnostics::LintViolation;
use crate::enums::Patch;
use crate::error::FromContextError;
use crate::patches::patch_registration;
use crate::patches::traits::RulePatch;
use crate::patches::traits::{CompilePatches, PatchFromContext, RegisterablePatch};
use crate::tree::node::Node;
use phenolint_macros::register_patch;

#[register_patch(id = "CURIE001")]
struct CurieFormatPatch;

impl PatchFromContext for CurieFormatPatch {
    fn from_context(
        context: &LinterContext,
    ) -> Result<Box<dyn RegisterablePatch>, FromContextError> {
        Ok(Box::new(CurieFormatPatch))
    }
}

impl CompilePatches for CurieFormatPatch {
    fn compile_patches(&self, value: &Node, lint_violation: &LintViolation) -> Vec<Patch> {
        vec![]
    }
}
