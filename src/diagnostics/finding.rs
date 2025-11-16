use crate::diagnostics::violation::LintViolation;
use crate::patches::patch::Patch;
use crate::report::specs::ReportSpecs;

#[derive(Debug)]
pub struct LintFinding {
    violation: LintViolation,
    report: Option<ReportSpecs>,
    patches: Vec<Patch>,
}

impl LintFinding {
    pub fn new(violation: LintViolation, report: Option<ReportSpecs>, patches: Vec<Patch>) -> Self {
        Self {
            violation,
            report,
            patches,
        }
    }

    pub fn violation(&self) -> &LintViolation {
        &self.violation
    }

    pub fn patch(&self) -> &[Patch] {
        self.patches.as_ref()
    }

    pub fn report(&self) -> Option<&ReportSpecs> {
        self.report.as_ref()
    }
}
