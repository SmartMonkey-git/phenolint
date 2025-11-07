use crate::enums::Patch;
use crate::report::lint_violation::LintViolation;
use crate::report::owned_report::OwnedReport;

#[derive(Clone, Debug)]
pub struct LintFinding {
    violation: LintViolation,
    patch: Option<Patch>,
}

impl LintFinding {
    pub fn new(rule_id: &str, report: OwnedReport, patch: Option<Patch>) -> Self {
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
