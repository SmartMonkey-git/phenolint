use crate::enums::Patch;
use crate::report::linting_violation::LintingViolation;
use crate::report::owned_report::OwnedReport;

#[derive(Clone, Debug)]
pub struct LintReportInfo {
    violation: LintingViolation,
    patch: Option<Patch>,
}

impl LintReportInfo {
    pub fn new(rule_id: &str, report: OwnedReport, patch: Option<Patch>) -> Self {
        Self {
            violation: LintingViolation::new(rule_id, report),
            patch,
        }
    }

    pub fn violation(&self) -> &LintingViolation {
        &self.violation
    }
    pub fn patch(&self) -> Option<&Patch> {
        self.patch.as_ref()
    }
}
