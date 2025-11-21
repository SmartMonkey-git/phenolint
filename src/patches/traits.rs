use crate::LinterContext;
use crate::diagnostics::LintViolation;
use crate::error::FromContextError;
use crate::patches::patch::Patch;
use crate::tree::node::DynamicNode;

pub trait RegisterablePatch: Send + Sync {
    fn compile_patches(
        &self,
        full_node: &DynamicNode,
        lint_violation: &LintViolation,
    ) -> Vec<Patch>;
    fn rule_id(&self) -> String;
}

pub trait PatchFromContext {
    fn from_context(
        context: &LinterContext,
    ) -> Result<Box<dyn RegisterablePatch>, FromContextError>;
}

impl<T: CompilePatches + Send + RulePatch> RegisterablePatch for T {
    fn compile_patches(&self, value: &DynamicNode, lint_violation: &LintViolation) -> Vec<Patch> {
        CompilePatches::compile_patches(self, value, lint_violation)
    }

    fn rule_id(&self) -> String {
        Self::RULE_ID.to_string()
    }
}

pub trait RulePatch: PatchFromContext + RegisterablePatch + CompilePatches {
    const RULE_ID: &'static str;
}

/// Tries to compile patches for a given rule.
pub trait CompilePatches: Send + Sync {
    fn compile_patches(&self, node: &DynamicNode, lint_violation: &LintViolation) -> Vec<Patch>;
}
