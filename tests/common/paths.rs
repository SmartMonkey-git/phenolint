use rstest::fixture;
use std::path::PathBuf;

#[fixture]
pub fn assets_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("assets")
}

#[fixture]
pub fn hpo_dir(assets_dir: PathBuf) -> PathBuf {
    assets_dir.join("hpo.toy.json")
}

#[fixture]
pub fn json_phenopacket_path(assets_dir: PathBuf) -> PathBuf {
    assets_dir.join("phenopacket.json")
}

#[fixture]
pub fn yaml_phenopacket_path(assets_dir: PathBuf) -> PathBuf {
    assets_dir.join("phenopacket.yaml")
}

#[fixture]
pub fn protobufphenopacket_path(assets_dir: PathBuf) -> PathBuf {
    assets_dir.join("phenopacket.pb")
}
