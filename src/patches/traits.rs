use crate::LinterContext;
use crate::diagnostics::LintViolation;
use crate::enums::Patch;
use crate::error::RuleInitError;
use crate::tree::node::Node;

pub trait RegisterablePatch: Send + Sync {
    fn compile_patches(&self, value: &Node, lint_violation: &LintViolation) -> Vec<Patch>;
    fn rule_id(&self) -> String;
}

pub trait PatchFromContext {
    fn from_context(context: &LinterContext) -> Result<Box<dyn RegisterablePatch>, RuleInitError>;
}

impl<T: CompilePatches + Send + RulePatch> RegisterablePatch for T {
    fn compile_patches(&self, value: &Node, lint_violation: &LintViolation) -> Vec<Patch> {
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
    fn compile_patches(&self, value: &Node, lint_violation: &LintViolation) -> Vec<Patch>;
}
