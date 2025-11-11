//! Responsible for working with application state and DI

use std::sync::Arc;

use crate::config::Configuration;

/// The application state contains
/// connections and configurations for the server
pub struct State {
    /// State wrapped in Arc for safe transfer between threads
    inner: Arc<StateInner>,
}

pub struct StateInner {
    /// Application configuration for receiving settings in endpoints
    config: Configuration,
}

impl State {
    /// Create a new application context
    pub fn new(config: Configuration) -> Self {
        let inner = StateInner { config };

        State {
            inner: Arc::new(inner),
        }
    }
}
