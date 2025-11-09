use crate::diagnostics::specs::ReportSpecs;

#[derive(Debug, PartialEq)]
pub struct LintViolation {
    rule_id: String,
    report: ReportSpecs,
}

impl LintViolation {
    pub fn new(rule_id: &str, report: ReportSpecs) -> LintViolation {
        Self {
            rule_id: rule_id.to_string(),
            report,
        }
    }

    pub fn rule_id(&self) -> &str {
        &self.rule_id
    }
    pub fn report(&self) -> &ReportSpecs {
        &self.report
    }
}
