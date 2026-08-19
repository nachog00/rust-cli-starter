use std::sync::Arc;

use {{crate_name}}_core::ports::Health;

/// The driving ports the CLI needs, injected by the binary's composition
/// root. Add a field per port as the application grows.
pub struct Ctx {
    pub health: Arc<dyn Health>,
}
