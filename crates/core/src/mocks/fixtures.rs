//! Small constructors for use in tests, keeping test bodies terse.

use crate::types::{DateTime, EnvName, Utc};

/// A fixed, arbitrary instant (2023-11-14T22:13:20Z) for deterministic tests.
pub fn instant() -> DateTime<Utc> {
    DateTime::from_timestamp(1_700_000_000, 0).expect("valid timestamp")
}

/// Build an [`EnvName`], panicking on invalid input. Tests only.
pub fn env(name: &str) -> EnvName {
    EnvName::parse(name).expect("valid env name")
}
