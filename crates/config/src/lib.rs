//! Config adapter: read a TOML file into a typed, validated [`Config`].
//!
//! The raw file shape is deserialized with serde, then parsed into core
//! newtypes — so an invalid `environment` fails here, at the boundary,
//! rather than deep in the domain.

mod config;

pub use config::{Config, ConfigError};

use std::path::Path;

/// The default config written by `{{project-name}} setup`.
pub const DEFAULT_CONFIG: &str = "# {{project-name}} configuration\nenvironment = \"dev\"\n";

/// Load and parse the config file at `path`.
pub fn load(path: &Path) -> Result<Config, ConfigError> {
    let raw = std::fs::read_to_string(path)?;
    Config::from_toml(&raw)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_default_config() {
        let cfg = Config::from_toml(DEFAULT_CONFIG).unwrap();
        assert_eq!(cfg.environment.as_str(), "dev");
    }

    #[test]
    fn rejects_invalid_environment() {
        let err = Config::from_toml("environment = \"Not Valid\"\n").unwrap_err();
        assert!(matches!(err, ConfigError::Environment(_)));
    }

    #[test]
    fn rejects_missing_field() {
        assert!(Config::from_toml("").is_err());
    }
}
