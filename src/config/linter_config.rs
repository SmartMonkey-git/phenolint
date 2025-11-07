use crate::config::config_loader::ConfigLoader;
use config::ConfigError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct LinterConfig {
    #[serde(rename = "rules")]
    pub rule_ids: Vec<String>,
    pub hpo_dir: Option<PathBuf>,
    #[serde(default)]
    patch: bool,
    #[serde(default)]
    quiet: bool,
}

impl TryFrom<PathBuf> for LinterConfig {
    type Error = ConfigError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        ConfigLoader::load(value)
    }
}
