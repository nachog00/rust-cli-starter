use serde::Deserialize;

use {{crate_name}}_core::types::{EnvName, InvalidEnvName};

/// Validated application configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub environment: EnvName,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("failed to read config file: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to parse config: {0}")]
    Toml(#[from] toml::de::Error),
    #[error(transparent)]
    Environment(#[from] InvalidEnvName),
}

/// The on-disk shape, before parsing into core newtypes.
#[derive(Deserialize)]
struct RawConfig {
    environment: String,
}

impl Config {
    pub fn from_toml(input: &str) -> Result<Self, ConfigError> {
        let raw: RawConfig = toml::from_str(input)?;
        Ok(Self {
            environment: EnvName::parse(raw.environment)?,
        })
    }
}
