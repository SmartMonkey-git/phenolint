use crate::LinterContext;
use crate::diagnostics::LintViolation;
use crate::error::FromContextError;
use crate::patches::enums::PatchInstruction;
use crate::patches::patch::Patch;
use crate::patches::patch_registration::PatchRegistration;
use crate::patches::traits::RulePatch;
use crate::patches::traits::{CompilePatches, PatchFromContext, RegisterablePatch};
use crate::tree::pointer::Pointer;
use crate::tree::traits::Node;
use phenolint_macros::register_patch;
use phenopackets::schema::v2::core::{Disease, OntologyClass};
use serde_json::Value;

#[register_patch(id = "INTER001")]
struct DiseaseConsistencyPatch;

impl PatchFromContext for DiseaseConsistencyPatch {
    fn from_context(_: &LinterContext) -> Result<Box<dyn RegisterablePatch>, FromContextError> {
        Ok(Box::new(Self))
    }
}

impl CompilePatches for DiseaseConsistencyPatch {
    fn compile_patches(&self, value: &dyn Node, lint_violation: &LintViolation) -> Vec<Patch> {
        let oc: OntologyClass = serde_json::from_value(
            value
                .value_at(lint_violation.at().first().expect("Should have pointer."))
                .unwrap()
                .as_ref()
                .clone(),
        )
        .unwrap();

        let disease: Value = serde_json::to_value(Disease {
            term: Some(oc),
            ..Default::default()
        })
        .unwrap();

        let instruction = PatchInstruction::Add {
            at: Pointer::at_root().down("diseases").clone(),
            value: Value::Array(vec![disease]),
        };

        vec![Patch::new(vec![instruction])]
    }
}
