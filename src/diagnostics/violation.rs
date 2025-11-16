use crate::tree::pointer::Pointer;

#[derive(Debug, PartialEq)]
pub struct LintViolation {
    // TODO: Add level of violation (Error, Warning ...)
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

    pub fn at(&self) -> &Pointer {
        &self.at
    }
}
