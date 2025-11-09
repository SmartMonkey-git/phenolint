use crate::diagnostics::specs::ReportSpecs;
use crate::diagnostics::violation::LintViolation;
use crate::enums::Patch;

#[derive(Debug)]
pub struct LintFinding {
    violation: LintViolation,
    patch: Option<Patch>,
}

impl LintFinding {
    pub fn new(rule_id: &str, report: ReportSpecs, patch: Option<Patch>) -> Self {
        Self {
            violation: LintViolation::new(rule_id, report),
            patch,
        }
    }

    pub fn violation(&self) -> &LintViolation {
        &self.violation
    }
    pub fn patch(&self) -> Option<&Patch> {
        self.patch.as_ref()
    }
}
