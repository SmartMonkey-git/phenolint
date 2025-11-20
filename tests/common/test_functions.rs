use crate::common::asserts::{LintResultAssertSettings, assert_lint_result};
use crate::common::construction::linter;
use gag::BufferRedirect;
use phenolint::traits::Lint;
use phenopackets::schema::v2::Phenopacket;
use std::env;
use std::io::Read;

pub fn run_rule_test(
    rule_id: &str,
    input: &Phenopacket,
    assert_settings: LintResultAssertSettings,
) {
    let mut stdout_buf = BufferRedirect::stdout().unwrap();
    let mut stderr_buf = BufferRedirect::stderr().unwrap();

    let mut linter = linter(vec![rule_id]);
    let phenostr = serde_json::to_string_pretty(&input).unwrap();
    let res = linter.lint(phenostr.as_str(), true, false);

    let mut stdout_output = String::new();
    let mut stderr_output = String::new();
    stdout_buf.read_to_string(&mut stdout_output).unwrap();
    stderr_buf.read_to_string(&mut stderr_output).unwrap();
    drop(stdout_buf);
    drop(stderr_buf);

    let output = if !stderr_output.is_empty() {
        stderr_output
    } else {
        stdout_output
    };

    if env::var("CI").is_err() {
        eprintln!("{}", output);
    }

    assert_lint_result(res, assert_settings, output);
}
