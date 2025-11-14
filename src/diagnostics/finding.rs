use crate::diagnostics::ReportSpecs;
use crate::diagnostics::violation::LintViolation;
use crate::enums::Patch;

#[derive(Debug)]
pub struct LintFinding {
    violation: LintViolation,
    report: ReportSpecs,
    patches: Vec<Patch>,
}

impl LintFinding {
    pub fn new(violation: LintViolation, report: ReportSpecs, patches: Vec<Patch>) -> Self {
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

    pub fn report(&self) -> &ReportSpecs {
        &self.report
    }
}
