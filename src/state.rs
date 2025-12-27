use std::sync::Arc;

#[derive(Clone)]
pub struct State {
    inner: Arc<InnerState>,
}

impl State {
    pub fn new(
        database: crate::database::DatabasePool,
        storage: crate::storage::Storage,
        temp_storage: crate::storage::Storage,
    ) -> Self {
        State {
            inner: Arc::new(InnerState {
                database,
                storage,
                temp_storage,
            }),
        }
    }

    pub fn store(&self) -> &crate::storage::Storage {
        &self.inner.storage
    }

    pub fn db(&self) -> &crate::database::DatabasePool {
        &self.inner.database
    }

    pub fn temp_store(&self) -> &crate::storage::Storage {
        &self.inner.temp_storage
    }
}

pub struct InnerState {
    database: crate::database::DatabasePool,
    storage: crate::storage::Storage,
    temp_storage: crate::storage::Storage,
}
