use config::{Config, ConfigError, File, FileFormat};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Default)]
pub(crate) struct ConfigLoader;

impl ConfigLoader {
    pub fn load<'a, T: Serialize + Deserialize<'a>>(file_path: PathBuf) -> Result<T, ConfigError> {
        if let Some(ext) = file_path.extension() {
            let file_format = match ext.to_str() {
                Some("yaml") => Ok(FileFormat::Yaml),
                Some("yml") => Ok(FileFormat::Yaml),
                Some("json") => Ok(FileFormat::Json),
                Some("toml") => Ok(FileFormat::Toml),
                Some("ron") => Ok(FileFormat::Ron),
                _ => Err(ConfigError::NotFound(format!(
                    "File format not supported. File needs to end with .yaml, .json, .toml or .ron. {file_path:?}"
                ))),
            }?;

            let config = Config::builder()
                .add_source(File::new(file_path.to_str().unwrap(), file_format))
                .build()?;

            let config_struct: T = config.try_deserialize()?;
            Ok(config_struct)
        } else {
            Err(ConfigError::NotFound(format!(
                "Could not find file extension on path {file_path:?}"
            )))
        }
    }
}
