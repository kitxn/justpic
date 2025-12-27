use serde::Deserialize;
use utoipa::ToSchema;

use crate::{SESSION_LIFETIME, utils::validation::Validatable};

/// Internal model for the user entity
#[derive(sqlx::FromRow)]
pub struct Session {
    id: uuid::Uuid,

    owner_id: uuid::Uuid,

    agent: Option<String>,

    created: chrono::DateTime<chrono::Utc>,
    expires: chrono::DateTime<chrono::Utc>,
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

    /// Check if the session has expired
    pub fn is_expired(&self) -> bool {
        self.expires < chrono::Utc::now()
    }

    /// Extend the lifetime of a mutable session
    pub fn extend_life_time(&mut self, extend_for: u64) {
        self.expires = self.expires + chrono::Days::new(extend_for);
    }

    /// The number of days remaining until the session expires
    pub fn days_of_life_left(&self) -> i16 {
        if self.is_expired() {
            return -1;
        }

        let time_left = self.expires - chrono::Utc::now();
        time_left.num_days() as i16
    }
}

// -- DTO --
#[derive(Debug, Deserialize, ToSchema)]
pub struct SessionCreateRequest {
    #[schema(example = "john_doe")]
    pub username: String,

    #[schema(example = "hunter42!")]
    pub password: String,
}

impl Validatable for SessionCreateRequest {
    fn validate(&self) -> Result<(), crate::error::Error> {
        if self.username.len() > 42 {
            return Err(crate::error::Error::Validation {
                field: "username",
                message: "Incorrect username length (3-42 characters required)",
            });
        }

        if self.password.len() > 72 {
            return Err(crate::error::Error::Validation {
                field: "password",
                message: "Incorrect password length (8-72 characters required)",
            });
        }

        Ok(())
    }
}
