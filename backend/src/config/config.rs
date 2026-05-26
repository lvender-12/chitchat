use anyhow::{Context, Result};
use config::{Config, File, FileFormat};

use crate::config_model::ConfigModel;

pub fn load_config() -> Result<ConfigModel> {
    let conf: ConfigModel = Config::builder()
        .add_source(File::new("config/config.yaml", FileFormat::Yaml))
        .build()
        .context("failed to read config")?
        .try_deserialize()
        .context("failed to deserialize")?;

    Ok(conf)
}
