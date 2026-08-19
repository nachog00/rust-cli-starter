//! Composition root.
//!
//! The only place that names concrete adapters. It resolves paths, loads
//! config, constructs adapters, wires them into services, and dispatches
//! CLI commands. Everything else depends on ports, not implementations —
//! so swapping an adapter (e.g. a SQLite store for the in-memory one)
//! touches only this file.

use std::sync::Arc;

use anyhow::{Context, Result};
use clap::Parser;

use {{crate_name}}_cli::{Cli, Command, Ctx, commands};
use {{crate_name}}_core::ports::Clock;
use {{crate_name}}_core::types::{DateTime, Utc};
use {{crate_name}}_domain::services::HealthService;
use {{crate_name}}_installer::Paths;

/// Real system-clock adapter for the [`Clock`] driven port.
struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Command::Setup => run_setup(),
        Command::Uninstall => run_uninstall(),
        Command::Status(args) => with_ctx(|ctx| {
            commands::status::run(args, ctx);
            Ok(())
        }),
    }
}

/// Build the driving-port context and hand it to a command.
fn with_ctx(f: impl FnOnce(&Ctx) -> Result<()>) -> Result<()> {
    let paths = Paths::resolve().context("failed to resolve paths")?;
    let config = {{crate_name}}_config::load(&paths.config_file())
        .context("failed to load config (run `{{project-name}} setup` first)")?;

    let clock: Arc<dyn Clock> = Arc::new(SystemClock);
    let health = Arc::new(HealthService::new(config.environment, clock));

    let ctx = Ctx { health };
    f(&ctx)
}

fn run_setup() -> Result<()> {
    let paths = Paths::resolve().context("failed to resolve paths")?;
    let report = {{crate_name}}_installer::setup(&paths)?;
    for step in &report.steps {
        println!("{step}");
    }
    Ok(())
}

fn run_uninstall() -> Result<()> {
    let paths = Paths::resolve().context("failed to resolve paths")?;
    let report = {{crate_name}}_installer::uninstall(&paths)?;
    for step in &report.steps {
        println!("{step}");
    }
    Ok(())
}
