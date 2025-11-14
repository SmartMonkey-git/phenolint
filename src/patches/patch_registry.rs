use crate::diagnostics::LintViolation;
use crate::enums::Patch;
use crate::patches::patch_registration::PatchRegistration;
use crate::patches::traits::{CompilePatches, RegisterablePatch, RulePatch};
use crate::tree::node::Node;
use std::collections::HashMap;

#[derive(Default)]
pub struct PatchRegistry {
    patches: HashMap<String, Box<dyn RegisterablePatch>>,
}

impl PatchRegistry {
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

    pub fn with_all_patches() -> Self {
        let mut registry = Self::default();

        for registration in inventory::iter::<PatchRegistration> {
            (registration.register)(&mut registry);
        }

        registry
    }
}
