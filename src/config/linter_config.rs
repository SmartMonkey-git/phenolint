use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LinterConfig {
    pub rule_ids: Vec<String>,
    pub fix: bool,
    pub hpo_dir: Option<PathBuf>,
}

