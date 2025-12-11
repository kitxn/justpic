use crate::models::users::responses::common::UserPublic;

/// Internal model for the user entity
#[derive(sqlx::FromRow)]
pub struct User {
    id: uuid::Uuid,

    username: String,
    password: String,

    role: super::role::UserRole,

    created: chrono::DateTime<chrono::Utc>,
}

impl User {
    // -- Getters --
    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn id_copy(&self) -> uuid::Uuid {
        self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn role(&self) -> &super::role::UserRole {
        &self.role
    }

    pub fn created(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.created
    }

    // -- Features --
    pub fn new(username: &str, password_hash: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            username: username.to_lowercase(),
            password: password_hash,
            role: super::role::UserRole::Regular,
            created: chrono::Utc::now(),
        }
    }

    /// Is the user an administrator?
    pub fn is_admin(&self) -> bool {
        self.role.is_admin()
    }

    // -- Mappers --
    /// Make the entity public by removing sensitive fields
    pub fn to_public_model(self) -> UserPublic {
        UserPublic {
            id: self.id,
            username: self.username,
            role: self.role,
            created: self.created,
        }
    }
}
