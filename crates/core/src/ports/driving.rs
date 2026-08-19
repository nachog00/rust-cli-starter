use crate::types::HealthReport;

/// Inbound port: report application health.
///
/// Implemented by the domain (`HealthService`) and consumed by delivery
/// mechanisms through `Arc<dyn Health>`. Callers never name the concrete
/// service — only the binary's composition root does.
pub trait Health: Send + Sync {
    fn report(&self) -> HealthReport;
}
