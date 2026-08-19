//! Value types. One module per type; invariants enforced at construction.

mod env_name;
mod health;

pub use env_name::{EnvName, InvalidEnvName};
pub use health::HealthReport;

// Re-exported so downstream crates get a single, consistent chrono surface.
pub use chrono::{DateTime, Utc};
