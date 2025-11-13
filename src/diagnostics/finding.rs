use crate::diagnostics::specs::ReportSpecs;
use crate::diagnostics::violation::LintViolation;
use crate::enums::{Patch, PatchAction};

#[derive(Debug)]
pub struct LintFinding {
    violation: LintViolation,
    has_ambiguous_patches: bool,
    patches: Vec<Patch>,
}

impl LintFinding {
    pub fn new(
        rule_id: &str,
        report: ReportSpecs,
        has_ambiguous_patches: bool,
        patches: Vec<Patch>,
    ) -> Self {
        Self {
            violation: LintViolation::new(rule_id, report),
            has_ambiguous_patches,
            patches,
        }
    }

    pub fn violation(&self) -> &LintViolation {
        &self.violation
    }

    pub fn ambiguous_patches(&self) -> bool {
        self.has_ambiguous_patches
    }

    pub fn patch(&self) -> &[Patch] {
        self.patches.as_ref()
    }
}
