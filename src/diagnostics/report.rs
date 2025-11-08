use crate::diagnostics::LintViolation;
use crate::diagnostics::finding::LintFinding;
use crate::enums::Patch;

#[derive(Clone, Debug, Default)]
pub struct LintReport {
    pub patched_phenopacket: Option<String>,
    pub findings: Vec<LintFinding>,
}

impl LintReport {
    pub fn new() -> LintReport {
        LintReport {
            patched_phenopacket: None,
            findings: Vec::new(),
        }
    }

    pub fn violations(&'_ self) -> Vec<&LintViolation> {
        self.findings.iter().map(|i| i.violation()).collect()
    }

    pub fn patches(&self) -> Vec<&Patch> {
        self.findings.iter().filter_map(|lri| lri.patch()).collect()
    }

    pub fn push_finding(&mut self, finding: LintFinding) {
        self.findings.push(finding);
    }

    pub fn extend_finding(&mut self, findings: &[LintFinding]) {
        self.findings.extend(findings.iter().cloned());
    }

    pub fn has_violations(&self) -> bool {
        !self.findings.is_empty()
    }

    pub fn has_patches(&self) -> bool {
        for info in &self.findings {
            if info.patch().is_some() {
                return true;
            }
        }
        false
    }
}
