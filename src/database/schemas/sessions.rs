use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;

use crate::SESSION_LIFETIME;

#[derive(Debug, FromRow)]
pub struct DbSession {
    pub(crate) id: uuid::Uuid,

    pub(crate) owner_id: uuid::Uuid,

    pub(crate) agent: Option<String>,

    pub(crate) created: DateTime<Utc>,
    pub(crate) expires: DateTime<Utc>,
}

impl DbSession {
    pub fn new(owner_id: uuid::Uuid, agent: Option<String>) -> Self {
        let expired = Utc::now() + chrono::Days::new(SESSION_LIFETIME);
        let id = uuid::Uuid::new_v4();

        DbSession {
            id,
            owner_id,
            agent,
            created: Utc::now(),
            expires: expired,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.expires >= Utc::now()
    }

    pub fn id(&self) -> uuid::Uuid {
        self.id
    }

    pub fn owner_id(&self) -> uuid::Uuid {
        self.owner_id
    }
}
