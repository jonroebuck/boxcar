use std::sync::Arc;
use boxcar_core::registry::ServerRegistry;
use tokio::sync::RwLock;

/// Shared application state injected into every Axum handler.
/// The registry is behind an RwLock so adapters can be
/// registered at runtime without a restart.
#[derive(Clone)]
pub struct AppState {
    pub registry: Arc<RwLock<ServerRegistry>>,
}

impl AppState {
    pub fn new(registry: ServerRegistry) -> Self {
        Self {
            registry: Arc::new(RwLock::new(registry)),
        }
    }
}
