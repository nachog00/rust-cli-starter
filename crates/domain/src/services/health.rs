use std::sync::Arc;

use {{crate_name}}_core::ports::{Clock, Health};
use {{crate_name}}_core::types::{EnvName, HealthReport};

/// Reports application health. Depends on the [`Clock`] driven port so the
/// reported timestamp is deterministic under test.
pub struct HealthService {
    environment: EnvName,
    clock: Arc<dyn Clock>,
}

impl HealthService {
    pub fn new(environment: EnvName, clock: Arc<dyn Clock>) -> Self {
        Self { environment, clock }
    }
}

impl Health for HealthService {
    fn report(&self) -> HealthReport {
        HealthReport {
            environment: self.environment.clone(),
            version: env!("CARGO_PKG_VERSION"),
            checked_at: self.clock.now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use {{crate_name}}_core::mocks::FixedClock;
    use {{crate_name}}_core::mocks::fixtures::{env, instant};

    #[test]
    fn report_carries_injected_env_and_clock() {
        let clock = Arc::new(FixedClock::new(instant()));
        let svc = HealthService::new(env("dev"), clock);

        let report = svc.report();

        assert_eq!(report.environment, env("dev"));
        assert_eq!(report.checked_at, instant());
        assert_eq!(report.version, env!("CARGO_PKG_VERSION"));
    }
}
