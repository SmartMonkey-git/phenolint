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
