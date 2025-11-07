use crate::enums::Patch;
use crate::report::linting_violation::LintingViolation;

#[derive(Clone, Debug)]
pub struct LintReportInfo {
    violation: LintingViolation,
    patch: Option<Patch>,
}

impl LintReportInfo {
    pub fn new(violation: LintingViolation, patch: Option<Patch>) -> Self {
        Self { violation, patch }
    }

    pub fn violation(&self) -> &LintingViolation {
        &self.violation
    }
    pub fn patch(&self) -> Option<&Patch> {
        self.patch.as_ref()
    }
}
