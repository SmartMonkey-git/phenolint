use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LinterConfig {
    #[serde(rename = "rules")]
    pub rule_ids: Vec<String>,
    pub hpo_dir: Option<PathBuf>,
}

