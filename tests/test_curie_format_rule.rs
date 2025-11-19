mod common;
use crate::common::asserts::LintResultAssertSettings;
use crate::common::construction::minimal_valid_phenopacket;
use crate::common::test_functions::run_rule_test;
use phenopackets::schema::v2::core::{OntologyClass, PhenotypicFeature};
use rstest::rstest;

#[rstest]
fn test_curie_format_rule() {
    let mut pp = minimal_valid_phenopacket();

    pp.phenotypic_features = vec![
        PhenotypicFeature {
            r#type: Some(OntologyClass {
                id: "invalid_id:31nm".to_string(),
                label: "some pf".to_string(),
            }),
            ..Default::default()
        },
        PhenotypicFeature {
            r#type: Some(OntologyClass {
                id: "HP:0002090".to_string(),
                label: "Pneumonia".to_string(),
            }),
            ..Default::default()
        },
    ];

    let rule_id = "CURIE001";
    let assert_settings = LintResultAssertSettings {
        rule_id,
        n_violations: 1,
        patched_phenopacket: None,
        patches: vec![],
        message_snippets: vec!["invalid_id:31nm", "formatted", "CURIE"],
    };

    run_rule_test(rule_id, &pp, assert_settings);
}
