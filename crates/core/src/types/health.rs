use crate::types::{DateTime, EnvName, Utc};

/// The result of a health check — the output of the [`Health`] driving port.
///
/// [`Health`]: crate::ports::Health
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HealthReport {
    pub environment: EnvName,
    pub version: &'static str,
    pub checked_at: DateTime<Utc>,
}
