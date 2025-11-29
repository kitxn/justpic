use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct User {
    pub(crate) id: uuid::Uuid,

    pub(crate) username: String,
    pub(crate) password: String,

    pub(crate) role: UserRole,

    pub(crate) created: DateTime<Utc>,
}

impl User {
    pub fn new(username: String, password: String) -> Self {
        User {
            id: uuid::Uuid::new_v4(),
            username: username.to_lowercase(),
            password,
            role: UserRole::Regular,
            created: Utc::now(),
        }
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn role(&self) -> &UserRole {
        &self.role
    }
}

#[derive(Debug, serde::Serialize, sqlx::Type, derive_more::Display)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    #[sqlx(rename = "regular")]
    #[display("regular")]
    Regular,

    #[sqlx(rename = "admin")]
    #[display("admin")]
    Admin,
}
