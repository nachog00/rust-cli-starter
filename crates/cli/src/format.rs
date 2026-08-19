//! Presentation helpers: turn core types into terminal output. Keeping this
//! separate from command logic makes both easy to change independently.

use {{crate_name}}_core::types::HealthReport;

pub fn health(report: &HealthReport) -> String {
    format!(
        "environment: {}\nversion:     {}\nchecked at:  {}",
        report.environment,
        report.version,
        report.checked_at.to_rfc3339(),
    )
}
