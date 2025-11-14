use crate::diagnostics::LintViolation;
use crate::enums::Patch;
use crate::new::node::Node;
use crate::new::traits::CompilePatches;
use phenolint_macros::register_patch;

#[register_patch("CURIE001")]
struct CurieFormatPatch;

impl CompilePatches for CurieFormatPatch {
    fn compile_patches(&self, value: &Node, lint_violation: &LintViolation) -> Vec<Patch> {
        todo!()
    }
}
