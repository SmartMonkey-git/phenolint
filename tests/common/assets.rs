use crate::common::paths::json_phenopacket_path;
use phenopackets::schema::v2::Phenopacket;
use rstest::fixture;
use std::fs;
use std::path::PathBuf;

#[fixture]
pub fn json_phenopacket(json_phenopacket_path: PathBuf) -> Phenopacket {
    let phenostr = fs::read_to_string(json_phenopacket_path).unwrap();

    let pp: Phenopacket = serde_json::from_str(&phenostr).unwrap();
    pp
}
