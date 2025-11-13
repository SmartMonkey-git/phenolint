use crate::diagnostics::LintViolation;
use crate::diagnostics::finding::LintFinding;
use crate::enums::{Patch, PatchAction};

#[derive(Debug, Default)]
pub struct LintReport {
    pub patched_phenopacket: Option<String>,
    findings: Vec<LintFinding>,
}

impl LintReport {
    pub fn new() -> LintReport {
        LintReport {
            patched_phenopacket: None,
            findings: Vec::new(),
        }
    }

    pub fn findings(&self) -> &[LintFinding] {
        &self.findings
    }
    pub fn violations(&'_ self) -> Vec<&LintViolation> {
        self.findings.iter().map(|i| i.violation()).collect()
    }

    pub fn patches(&self) -> Vec<&Patch> {
        self.findings.iter().flat_map(|lf| lf.patch()).collect()
    }

    pub fn ambiguous_patches(&self) -> Vec<&Patch> {
        self.findings
            .iter()
            .filter(|lf| lf.ambiguous_patches())
            .flat_map(|lf_filtered| lf_filtered.patch())
            .collect()
    }

    pub fn unambiguous_patches(&self) -> Vec<&Patch> {
        self.findings
            .iter()
            .filter(|lf| !lf.ambiguous_patches())
            .flat_map(|lf_filtered| lf_filtered.patch())
            .collect()
    }

    pub fn push_finding(&mut self, finding: LintFinding) {
        self.findings.push(finding);
    }

    pub fn extend_finding(&mut self, findings: Vec<LintFinding>) {
        self.findings.extend(findings);
    }

    pub fn has_violations(&self) -> bool {
        !self.findings.is_empty()
    }

    pub fn has_patches(&self) -> bool {
        for info in &self.findings {
            if !info.patch().is_empty() {
                return true;
            }
        }
        false
    }
}
