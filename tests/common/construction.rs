use crate::common::paths::{assets_dir, hpo_dir};
use phenolint::LinterContext;
use phenolint::phenolint::Phenolint;
use phenopackets::schema::v2::Phenopacket;
use phenopackets::schema::v2::core::MetaData;

pub fn linter(rules: Vec<&str>) -> Phenolint {
    let context = LinterContext::new(Some(hpo_dir(assets_dir())));
    let rules: Vec<String> = rules.into_iter().map(|s| s.to_string()).collect();
    Phenolint::new(context, rules)
}

#[allow(unused)]
pub fn minimal_valid_phenopacket() -> Phenopacket {
    Phenopacket {
        id: "cohort-1-patient-1".to_string(),
        meta_data: Some(MetaData {
            created: Some(prost_types::Timestamp {
                seconds: 50,
                nanos: 2,
            }),
            created_by: "Test-Suite".to_string(),
            submitted_by: "Test-Suite".to_string(),
            phenopacket_schema_version: "2".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    }
}
