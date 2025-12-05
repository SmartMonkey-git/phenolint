use crate::diagnostics::violation::LintViolation;
use crate::patches::patch::Patch;

#[derive(Debug)]
pub struct LintFinding {
    violation: LintViolation,
    patches: Vec<Patch>,
}

impl LintFinding {
    pub fn new(violation: LintViolation, patches: Vec<Patch>) -> Self {
        Self { violation, patches }
    }

    pub fn violation(&self) -> &LintViolation {
        &self.violation
    }

    pub fn patch(&self) -> &[Patch] {
        self.patches.as_ref()
    }
}
