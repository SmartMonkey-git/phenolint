use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct LinterConfig {
    #[serde(rename = "rules")]
    pub rule_ids: Vec<String>,
    pub hpo_dir: Option<PathBuf>,
}
