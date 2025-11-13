use crate::diagnostics::specs::ReportSpecs;
use crate::json::Pointer;

#[derive(Debug, PartialEq)]
pub struct LintViolation {
    rule_id: String,
    report: ReportSpecs,
    at: Pointer,
}

impl LintViolation {
    pub fn new(rule_id: &str, report: ReportSpecs, at: Pointer) -> LintViolation {
        Self {
            rule_id: rule_id.to_string(),
            report,
            at,
        }
    }

    pub fn rule_id(&self) -> &str {
        &self.rule_id
    }
    pub fn report(&self) -> &ReportSpecs {
        &self.report
    }
}
