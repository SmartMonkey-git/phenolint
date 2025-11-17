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

pub fn assert_lint_result(
    lint_result: LintResult,
    assert_settings: LintResultAssertSettings,
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
    assert_eq!(
        lint_result.report.patched_phenopacket, assert_settings.patched_phenopacket,
        "Patched phenopacket does not match expected value"
    );
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
