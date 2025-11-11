//! Responsible for working with application state and DI

use std::sync::Arc;

use crate::config::Configuration;

/// The application state contains
/// connections and configurations for the server
pub struct State {
    /// Application configuration for receiving settings in endpoints
    config: Arc<Configuration>,
}

impl State {
    /// Create a new application context
    pub fn new(config: Configuration) -> Self {
        let config = Arc::new(config);

        State { config }
    }
}
