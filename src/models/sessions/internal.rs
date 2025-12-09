use crate::SESSION_LIFETIME;

/// Internal model for the user entity
#[derive(sqlx::FromRow)]
pub struct Session {
    pub(super) id: uuid::Uuid,

    pub(super) owner_id: uuid::Uuid,

    pub(super) agent: Option<String>,

    pub(super) created: chrono::DateTime<chrono::Utc>,
    pub(super) expires: chrono::DateTime<chrono::Utc>,
}

impl Session {
    // -- Getters --
    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn owner_id(&self) -> &uuid::Uuid {
        &self.owner_id
    }

    pub fn agent(&self) -> Option<&String> {
        self.agent.as_ref()
    }

    pub fn created(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.created
    }

    pub fn expires(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.expires
    }

    // -- Features --
    pub fn new(owner_id: uuid::Uuid) -> Self {
        let id = uuid::Uuid::new_v4();

        let created = chrono::Utc::now();
        let expires = created + chrono::Days::new(SESSION_LIFETIME);

        Session {
            id,
            owner_id,
            agent: None, //temp
            created,
            expires,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.expires >= chrono::Utc::now()
    }
}
