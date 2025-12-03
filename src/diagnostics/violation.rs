use crate::helper::non_empty_vec::NonEmptyVec;
use crate::report::enums::ViolationSeverity;
use crate::tree::pointer::Pointer;

#[derive(Debug, PartialEq)]
pub struct LintViolation {
    severity: ViolationSeverity,
    rule_id: String,
    at: Vec<Pointer>,
}

impl LintViolation {
    pub fn new(
        severity: ViolationSeverity,
        rule_id: &str,
        at: NonEmptyVec<Pointer>,
    ) -> LintViolation {
        Self {
            severity,
            rule_id: rule_id.to_string(),
            at: at.into_vec(),
        }
    }

    pub fn severity(&self) -> &ViolationSeverity {
        &self.severity
    }

    pub fn rule_id(&self) -> &str {
        &self.rule_id
    }

    pub fn at(&self) -> &[Pointer] {
        &self.at
    }

    /// Will return the first pointer without an option
    ///
    /// This is guarantied to not panic, because `LintViolation` can only be initialized using `NonEmptyVec`
    pub fn first_at(&self) -> &Pointer {
        self.at.first().expect("At should never be empty")
    }
}
