use crate::LinterContext;
use crate::diagnostics::LintViolation;
use crate::patches::patch::Patch;
use crate::patches::patch_registration::PatchRegistration;
use crate::patches::traits::{CompilePatches, RegisterablePatch, RulePatch};
use crate::tree::node::Node;
use log::warn;
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

    pub fn with_enabled_patches(enabled_rules: &[String], context: &LinterContext) -> Self {
        let mut registry = HashMap::new();

        for registration in inventory::iter::<PatchRegistration> {
            if enabled_rules
                .iter()
                .any(|r_id| r_id == registration.rule_id)
            {
                match (registration.factory)(context) {
                    Ok(patch) => {
                        registry.insert(registration.rule_id.to_string(), patch);
                    }
                    Err(err) => warn!("Failed to register patch: {}", err),
                }
            }
        }

        PatchRegistry { patches: registry }
    }
}
