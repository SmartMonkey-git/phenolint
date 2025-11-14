use crate::RuleFromContext;
use crate::diagnostics::LintViolation;
use crate::enums::{Patch, PatchAction};
use crate::json::Pointer;
use crate::new::node::Node;
use crate::new::patches::patch_registry::{PatchFromContext, RegisterablePatch};

pub trait Spanning {
    fn span(&self, ptr: &Pointer) -> Option<(usize, usize)>;
}

pub trait ParsableNode<N> {
    fn parse(node: &Node) -> Option<N>;
}

pub trait RulePatch: PatchFromContext + RegisterablePatch + CompilePatches {
    const RULE_ID: &'static str;
}

/// Tries to compile patches for a given rule.
pub trait CompilePatches: Send + Sync {
    fn compile_patches(&self, value: &Node, lint_violation: &LintViolation) -> Vec<Patch>;
}
