use crate::diagnostics::LintViolation;
use crate::enums::Patch;
use crate::new::node::Node;
use crate::new::traits::{CompilePatches, RulePatch};
use std::collections::HashMap;

pub trait RegisterablePatch: Send + Sync {
    fn compile_patches(&self, value: &Node, lint_violation: &LintViolation) -> Vec<Patch>;
    fn rule_id(&self) -> String;
}

impl<T: CompilePatches + Send + RulePatch> RegisterablePatch for T {
    fn compile_patches(&self, value: &Node, lint_violation: &LintViolation) -> Vec<Patch> {
        CompilePatches::compile_patches(self, value, lint_violation)
    }

    fn rule_id(&self) -> String {
        Self::RULE_ID.to_string()
    }
}

pub struct PatchRegistry {
    patches: HashMap<String, Box<dyn RegisterablePatch>>,
}

impl PatchRegistry {
    pub fn new() -> Self {
        Self {
            patches: HashMap::new(),
        }
    }

    pub fn register<P: CompilePatches + RulePatch + 'static>(&mut self, rule_id: &str, patch: P) {
        self.patches.insert(rule_id.to_string(), Box::new(patch));
    }

    pub fn get_patches_for(
        &self,
        rule_id: &str,
        value: &Node,
        violation: &LintViolation,
    ) -> Vec<Patch> {
        if let Some(patch_compiler) = self.patches.get(rule_id) {
            patch_compiler.compile_patches(value, violation)
        } else {
            vec![]
        }
    }
}
/*
fn stuff() {
    // Usage:
    let mut registry = PatchRegistry::new();
    registry.register(crate::new::patches::curie_format_patch::CurieFormatPatch);
    let rule = CurieFormatRule::from_context(&LinterContext::default()).unwrap();
    // Later, get patches for CurieFormatRule
    let patches = registry.get_patches_for(&rule, &node, &violation);
}
*/
