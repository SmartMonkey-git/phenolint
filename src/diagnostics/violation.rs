use crate::diagnostics::specs::ReportSpecs;
use crate::json::Pointer;

#[derive(Debug, PartialEq)]
pub struct LintViolation {
    rule_id: String,
    at: Pointer,
}

impl LintViolation {
    pub fn new(rule_id: &str, at: Pointer) -> LintViolation {
        Self {
            rule_id: rule_id.to_string(),

            at,
        }
    }

    pub fn rule_id(&self) -> &str {
        &self.rule_id
    }
}
