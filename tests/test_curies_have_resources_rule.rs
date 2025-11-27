mod common;

use crate::common::asserts::LintResultAssertSettings;
use crate::common::test_functions::run_rule_test;
use common::construction::minimal_valid_phenopacket;
use phenopackets::schema::v2::core::{Individual, OntologyClass, PhenotypicFeature};

fn oc(id: impl ToString, label: impl ToString) -> Option<OntologyClass> {
    Some(OntologyClass {
        id: id.to_string(),
        label: label.to_string(),
    })
}

#[test]
fn test_rule() {
    let mut pp = minimal_valid_phenopacket();
    pp.subject = Some(Individual {
        id: "Jim001".into(),
        taxonomy: oc("NCBITaxon:9606", "Homo sapiens"),
        ..Default::default()
    });
    pp.phenotypic_features.push(PhenotypicFeature {
        r#type: oc("HP:0001250", "Seizure"),
        ..Default::default()
    });

    let rule_id = "INTER002";
    let assert_settings = LintResultAssertSettings {
        rule_id,
        n_violations: 2,
        patched_phenopacket: None,
        patches: vec![],
        message_snippets: vec![],
    };

    run_rule_test(rule_id, &pp, assert_settings);
}
