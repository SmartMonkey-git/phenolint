mod common;
use crate::common::asserts::LintResultAssertSettings;
use crate::common::construction::minimal_valid_phenopacket;
use crate::common::test_functions::run_rule_test;
use phenolint::diagnostics::enums::PhenopacketData;
use phenolint::patches::enums::PatchInstruction::Add;
use phenolint::patches::patch::Patch;
use phenolint::tree::pointer::Pointer;
use phenopackets::schema::v2::core::{Diagnosis, Disease, Interpretation, OntologyClass};
use rstest::{fixture, rstest};
use serde_json::Value;

#[fixture]
fn disease_oc() -> OntologyClass {
    OntologyClass {
        id: "MONDO:0000252".to_string(),
        label: "inflammatory diarrhea".to_string(),
    }
}

#[rstest]
fn test_disease_consistency_rule(disease_oc: OntologyClass) {
    let mut pp = minimal_valid_phenopacket();

    let interpretation_id = "interpretation_123";

    pp.interpretations.push(Interpretation {
        id: interpretation_id.to_string(),
        diagnosis: Some(Diagnosis {
            disease: Some(disease_oc.clone()),
            genomic_interpretations: vec![],
        }),
        ..Default::default()
    });

    let mut patched = pp.clone();

    let disease = Disease {
        term: Some(disease_oc),
        ..Default::default()
    };

    patched.diseases.push(disease.clone());

    let rule_id = "INTER001";
    let assert_settings = LintResultAssertSettings {
        rule_id,
        n_violations: 1,
        patched_phenopacket: Some(PhenopacketData::Text(
            serde_json::to_string_pretty(&patched).unwrap(),
        )),
        patches: vec![Patch {
            instructions: vec![Add {
                at: Pointer::new("/diseases"),
                value: Value::Array(vec![serde_json::to_value(disease).unwrap()]),
            }],
        }],
        message_snippets: vec![interpretation_id, "disease"],
    };

    run_rule_test(rule_id, &pp, assert_settings);
}

#[rstest]
fn test_disease_consistency_rule_no_violation(disease_oc: OntologyClass) {
    let mut pp = minimal_valid_phenopacket();

    let interpretation_id = "interpretation_123";

    pp.interpretations.push(Interpretation {
        id: interpretation_id.to_string(),
        diagnosis: Some(Diagnosis {
            disease: Some(disease_oc.clone()),
            genomic_interpretations: vec![],
        }),

        ..Default::default()
    });

    pp.diseases.push(Disease {
        term: Some(disease_oc),
        ..Default::default()
    });

    let rule_id = "INTER001";
    let assert_settings = LintResultAssertSettings {
        rule_id,
        n_violations: 0,
        patched_phenopacket: None,
        patches: vec![],
        message_snippets: vec![],
    };

    run_rule_test(rule_id, &pp, assert_settings);
}
