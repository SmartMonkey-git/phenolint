use crate::config::config_loader::ConfigLoader;
use crate::error::InitError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct LinterConfig {
    #[serde(rename = "rules")]
    pub rule_ids: Vec<String>,
    pub hpo_dir: Option<PathBuf>,
}

impl TryFrom<PathBuf> for LinterConfig {
    type Error = InitError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        Ok(ConfigLoader::load(value)?)
    }
}
