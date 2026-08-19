//! Installer adapter: resolve paths and manage on-disk integration points.
//!
//! `setup` writes the default config; `uninstall` removes it. Both return a
//! [`Report`] of human-readable steps for the CLI to print.

mod paths;

pub use paths::{Paths, PathsError};

use std::fs;

#[derive(Debug, thiserror::Error)]
pub enum InstallError {
    #[error(transparent)]
    Paths(#[from] PathsError),
    #[error("filesystem error: {0}")]
    Io(#[from] std::io::Error),
}

/// A record of what a step did, for user-facing output.
#[derive(Debug, Default)]
pub struct Report {
    pub steps: Vec<String>,
}

/// Write the default config into `paths`, unless one already exists.
pub fn setup(paths: &Paths) -> Result<Report, InstallError> {
    let mut report = Report::default();
    fs::create_dir_all(paths.config_dir())?;

    let cfg = paths.config_file();
    if cfg.exists() {
        report
            .steps
            .push(format!("config already present: {}", cfg.display()));
    } else {
        fs::write(&cfg, {{crate_name}}_config::DEFAULT_CONFIG)?;
        report
            .steps
            .push(format!("wrote default config: {}", cfg.display()));
    }
    Ok(report)
}

/// Remove the application's config directory.
pub fn uninstall(paths: &Paths) -> Result<Report, InstallError> {
    let mut report = Report::default();
    let dir = paths.config_dir();

    if dir.exists() {
        fs::remove_dir_all(dir)?;
        report.steps.push(format!("removed: {}", dir.display()));
    } else {
        report
            .steps
            .push(format!("nothing to remove at {}", dir.display()));
    }
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn setup_writes_loadable_config_and_is_idempotent() {
        let tmp = tempdir().unwrap();
        let paths = Paths::with_config_dir(tmp.path().join("{{project-name}}"));

        let first = setup(&paths).unwrap();
        assert!(paths.config_file().exists());
        assert!(first.steps[0].contains("wrote default config"));

        // The written config round-trips through the config adapter.
        let cfg = {{crate_name}}_config::load(&paths.config_file()).unwrap();
        assert_eq!(cfg.environment.as_str(), "dev");

        let second = setup(&paths).unwrap();
        assert!(second.steps[0].contains("already present"));
    }

    #[test]
    fn uninstall_removes_config_dir() {
        let tmp = tempdir().unwrap();
        let paths = Paths::with_config_dir(tmp.path().join("{{project-name}}"));
        setup(&paths).unwrap();

        uninstall(&paths).unwrap();
        assert!(!paths.config_dir().exists());
    }
}
