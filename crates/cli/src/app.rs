use clap::{Parser, Subcommand};

use crate::commands::status;

#[derive(Parser)]
#[command(name = "{{project-name}}", about = "{{description}}")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Write the default config file
    Setup,

    /// Remove the config directory
    Uninstall,

    /// Report application health
    Status(status::Args),
}
