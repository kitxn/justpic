//! Responsible for working with application state and DI

use std::sync::Arc;

use crate::config::Configuration;

/// The application state contains
/// connections and configurations for the server
///
/// Contains:
/// - config: [`Configuration`]
#[derive(Clone)]
pub struct State {
    /// State wrapped in Arc for safe transfer between threads
    inner: Arc<InnerState>,
}

impl State {
    /// Create a new application context
    pub fn new(
        config: Configuration,
        database: crate::database::DatabasePool,
        storage: crate::storage::Storage,
        temp_storage: crate::storage::Storage,
    ) -> Self {
        let inner = InnerState {
            config,
            database,
            storage,
            temp_storage,
        };

        State {
            inner: Arc::new(inner),
        }
    }

    pub fn store(&self) -> &crate::storage::Storage {
        &self.inner.storage
    }

    pub fn db(&self) -> &crate::database::DatabasePool {
        &self.inner.database
    }

    pub fn cfg(&self) -> &Configuration {
        &self.inner.config
    }

    pub fn temp_store(&self) -> &crate::storage::Storage {
        &self.inner.temp_storage
    }
}

pub struct InnerState {
    /// Application configuration for receiving settings in endpoints
    config: Configuration,

    database: crate::database::DatabasePool,

    storage: crate::storage::Storage,

    temp_storage: crate::storage::Storage,
}
