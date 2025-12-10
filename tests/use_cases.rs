use phenolint::LinterContext;
use phenolint::phenolint::Phenolint;
use phenolint::traits::Lint;
use serde_json::json;
use std::path::PathBuf;
use std::sync::OnceLock;

static PHENOLINT: OnceLock<Phenolint> = OnceLock::new();

#[test]
fn lint_minimal_valid_phenopacket() {
    let phenolint = PHENOLINT.get_or_init(init_phenolint);

    let value = json!({
        "id": "phenopacket",
    });
    let payload_str = serde_json::to_string(&value).unwrap();

    // TODO: is it absolutely necessary for `lint` to take `&mut self`?
    let result = phenolint.lint(payload_str.as_str(), true, true);

    // TODO: assert properly
    eprintln!("{:?}", result);
}

fn init_phenolint() -> Phenolint {
    let context = LinterContext::new(Some(PathBuf::from("tests/assets/hp.toy.json")));
    let rule_ids = vec!["CURIE001".into(), "INTER001".into()];
    Phenolint::new(context, rule_ids)
}
