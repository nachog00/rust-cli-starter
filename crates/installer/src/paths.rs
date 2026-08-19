use std::path::{Path, PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum PathsError {
    #[error("could not determine a config directory (XDG_CONFIG_HOME and HOME are unset)")]
    NoConfigDir,
}

/// Resolved filesystem locations for the application.
#[derive(Debug, Clone)]
pub struct Paths {
    config_dir: PathBuf,
}

impl Paths {
    /// Resolve paths from the environment (XDG-style):
    /// `$XDG_CONFIG_HOME/{{project-name}}`, falling back to
    /// `$HOME/.config/{{project-name}}`.
    pub fn resolve() -> Result<Self, PathsError> {
        let base = config_base().ok_or(PathsError::NoConfigDir)?;
        Ok(Self {
            config_dir: base.join("{{project-name}}"),
        })
    }

    /// Root the paths at an explicit directory. Handy for tests.
    pub fn with_config_dir(config_dir: PathBuf) -> Self {
        Self { config_dir }
    }

    pub fn config_dir(&self) -> &Path {
        &self.config_dir
    }

    pub fn config_file(&self) -> PathBuf {
        self.config_dir.join("config.toml")
    }
}

fn config_base() -> Option<PathBuf> {
    if let Some(xdg) = std::env::var_os("XDG_CONFIG_HOME")
        && !xdg.is_empty()
    {
        return Some(PathBuf::from(xdg));
    }
    std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".config"))
}
