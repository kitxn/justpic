//! Responsible for working with application state and DI

use std::sync::Arc;

use crate::config::Configuration;

/// The application state contains
/// connections and configurations for the server
///
/// Contains:
/// - config: [`Configuration`]
#[derive(Debug, Clone)]
pub struct State {
    /// State wrapped in Arc for safe transfer between threads
    inner: Arc<InnerState>,
}

#[derive(Debug)]
pub struct InnerState {
    /// Application configuration for receiving settings in endpoints
    config: Configuration,

    database: justpic_database::DatabasePool,

    storage: justpic_storage::storage::Storage,
}

impl State {
    /// Create a new application context
    pub fn new(
        config: Configuration,
        database: justpic_database::DatabasePool,
        storage: justpic_storage::storage::Storage,
    ) -> Self {
        let inner = InnerState {
            config,
            database,
            storage,
        };

        State {
            inner: Arc::new(inner),
        }
    }
}
