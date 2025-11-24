use crate::common::asserts::{LintResultAssertSettings, assert_lint_result};
use crate::common::construction::build_linter;
use gag::BufferRedirect;
use phenolint::enums::InputTypes;
use phenolint::traits::Lint;
use phenopackets::schema::v2::Phenopacket;
use prost::Message;
use std::env;
use std::io::Read;

pub fn run_rule_test(
    rule_id: &str,
    input: &Phenopacket,
    assert_settings: LintResultAssertSettings,
) {
    let formats: Vec<InputTypes> = vec![InputTypes::Json, InputTypes::Yaml, InputTypes::Protobuf];

    for format in formats {
        let mut stdout_buf = BufferRedirect::stdout().unwrap();
        let mut stderr_buf = BufferRedirect::stderr().unwrap();

        let mut linter = build_linter(vec![rule_id]);

        let res = match format {
            InputTypes::Json => linter.lint(
                serde_json::to_string_pretty(&input).unwrap().as_str(),
                true,
                false,
            ),
            InputTypes::Yaml => {
                linter.lint(serde_yaml::to_string(&input).unwrap().as_str(), true, false)
            }
            InputTypes::Protobuf => {
                let mut buffer = Vec::new();
                input.encode(&mut buffer).unwrap();
                linter.lint(buffer.as_slice(), true, false)
            }
        };

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
            eprintln!("Testing {format}");
            eprintln!("----");
            eprintln!("{}", output);
        }

        assert_lint_result(res, &assert_settings, output, format);
    }
}
