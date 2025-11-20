#![allow(dead_code)]
use phenolint::diagnostics::enums::PhenopacketData;
use phenolint::error::LintResult;
use phenolint::patches::patch::Patch;
use pretty_assertions::assert_eq;

pub struct LintResultAssertSettings<'a> {
    pub rule_id: &'a str,
    pub n_violations: usize,
    pub patched_phenopacket: Option<PhenopacketData>,
    pub patches: Vec<Patch>,
    pub message_snippets: Vec<&'a str>,
}

impl<'a> LintResultAssertSettings<'a> {
    /// Create a new builder for LintResultAssertSettings
    pub fn builder(rule_id: &'a str) -> LintResultAssertSettingsBuilder<'a> {
        LintResultAssertSettingsBuilder::new(rule_id)
    }
}

/// Builder for LintResultAssertSettings with a fluent API
#[derive(Debug, Clone)]
pub struct LintResultAssertSettingsBuilder<'a> {
    rule_id: &'a str,
    n_violations: Option<usize>,
    patched_phenopacket: Option<PhenopacketData>,
    patches: Vec<Patch>,
    message_snippets: Vec<&'a str>,
}

impl<'a> LintResultAssertSettingsBuilder<'a> {
    /// Create a new builder with the required rule_id
    pub fn new(rule_id: &'a str) -> Self {
        Self {
            rule_id,
            n_violations: None,
            patched_phenopacket: None,
            patches: Vec::new(),
            message_snippets: Vec::new(),
        }
    }

    /// Set the expected number of violations
    pub fn violations(mut self, n: usize) -> Self {
        self.n_violations = Some(n);
        self
    }

    /// Expect no violations (convenience method)
    pub fn no_violations(mut self) -> Self {
        self.n_violations = Some(0);
        self
    }

    /// Expect exactly one violation (convenience method)
    pub fn one_violation(mut self) -> Self {
        self.n_violations = Some(1);
        self
    }

    /// Set the expected patched phenopacket
    pub fn patched(mut self, phenopacket: PhenopacketData) -> Self {
        self.patched_phenopacket = Some(phenopacket);
        self
    }

    /// Add a single patch to expect
    pub fn patch(mut self, patch: Patch) -> Self {
        self.patches.push(patch);
        self
    }

    /// Set all expected patches at once
    pub fn patches(mut self, patches: Vec<Patch>) -> Self {
        self.patches = patches;
        self
    }

    /// Add a single message snippet to check for
    pub fn message_snippet(mut self, snippet: &'a str) -> Self {
        self.message_snippets.push(snippet);
        self
    }

    /// Set all message snippets at once
    pub fn messages(mut self, snippets: Vec<&'a str>) -> Self {
        self.message_snippets = snippets;
        self
    }

    /// Add multiple message snippets
    pub fn with_messages(mut self, snippets: &[&'a str]) -> Self {
        self.message_snippets.extend_from_slice(snippets);
        self
    }

    /// Build the final LintResultAssertSettings
    ///
    /// # Panics
    /// Panics if `n_violations` was not set
    pub fn build(self) -> LintResultAssertSettings<'a> {
        LintResultAssertSettings {
            rule_id: self.rule_id,
            n_violations: self.n_violations.expect("n_violations must be set"),
            patched_phenopacket: self.patched_phenopacket,
            patches: self.patches,
            message_snippets: self.message_snippets,
        }
    }

    /// Build with a default of 0 violations if not specified
    pub fn build_or_default(self) -> LintResultAssertSettings<'a> {
        LintResultAssertSettings {
            rule_id: self.rule_id,
            n_violations: self.n_violations.unwrap_or(0),
            patched_phenopacket: self.patched_phenopacket,
            patches: self.patches,
            message_snippets: self.message_snippets,
        }
    }
}

pub fn assert_lint_result(
    lint_result: LintResult,
    assert_settings: &LintResultAssertSettings,
    console_messages: String,
) {
    if let Some(err) = lint_result.error {
        eprintln!("Unexpected error during linting: {}", err);
    }
    assert_eq!(
        lint_result.report.violations().len(),
        assert_settings.n_violations,
        "Expected '{}' violations found '{}'",
        assert_settings.n_violations,
        lint_result.report.violations().len()
    );
    for violation in lint_result.report.violations() {
        assert_eq!(
            violation.rule_id(),
            assert_settings.rule_id,
            "Violation rule_id '{}' does not match expected rule_id '{}'",
            violation.rule_id(),
            assert_settings.rule_id
        );
    }
    //assert_eq!(
    //    lint_result.report.patched_phenopacket, assert_settings.patched_phenopacket,
    //    "Patched phenopacket does not match expected value"
    //);
    assert_eq!(
        lint_result.report.patches(),
        assert_settings.patches.iter().collect::<Vec<&Patch>>(),
        "Patches do not match expected patches"
    );

    let mut message_snippets = assert_settings.message_snippets.to_vec();
    message_snippets.push(assert_settings.rule_id);

    for ms in message_snippets {
        assert!(
            console_messages.contains(ms),
            "Console output does not contain expected snippet: '{}'",
            ms
        );
    }
}
