use serde::Deserialize;
use utoipa::ToSchema;

use crate::{users::types::UserRole, util::validation::Validatable};

/// Internal model for the user entity
#[derive(sqlx::FromRow)]
pub struct User {
    id: uuid::Uuid,

    username: String,
    password: String,

    role: UserRole,

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

    pub fn role(&self) -> &UserRole {
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
            role: UserRole::Regular,
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

/// A public user model stripped of all private fields
#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
pub struct UserPublic {
    pub id: uuid::Uuid,

    #[schema(example = "john_doe")]
    pub username: String,

    #[schema(value_type = String, example = "regular")]
    pub role: UserRole,

    pub created: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UserCreateRequest {
    #[schema(example = "john_doe")]
    pub username: String,

    #[schema(example = "hunter42!")]
    pub password: String,

    #[schema(example = "hunter42!")]
    pub password_confirmation: String,
}

impl Validatable for UserCreateRequest {
    fn validate(&self) -> Result<(), crate::error::Error> {
        if self.password != self.password_confirmation {
            return Err(crate::error::Error::Validation {
                field: "password_confirmation",
                message: "The passwords do not match",
            });
        }

        if !(3..42).contains(&self.username.len()) {
            return Err(crate::error::Error::Validation {
                field: "username",
                message: "Incorrect username length (3-42 characters required)",
            });
        }

        // TODO: Add password complexity checking
        if !(8..72).contains(&self.password.len()) {
            return Err(crate::error::Error::Validation {
                field: "password",
                message: "Incorrect password length (8-72 characters required)",
            });
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UserChangePasswordRequest {
    #[schema(example = "hunter42!")]
    pub old_password: String,

    #[schema(example = "hunter52!")]
    pub new_password: String,
}

impl Validatable for UserChangePasswordRequest {
    fn validate(&self) -> Result<(), crate::error::Error> {
        if !(8..72).contains(&self.new_password.len()) {
            return Err(crate::error::Error::Validation {
                field: "new_password",
                message: "Incorrect password length (8-72 characters required)",
            });
        }

        if self.old_password == self.new_password {
            return Err(crate::error::Error::Validation {
                field: "new_password",
                message: "The new password must not be the same as the old one",
            });
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UserChangeUsernameRequest {
    #[schema(example = "new_john_doe")]
    pub username: String,
}

impl Validatable for UserChangeUsernameRequest {
    fn validate(&self) -> Result<(), crate::error::Error> {
        if !(3..42).contains(&self.username.len()) {
            return Err(crate::error::Error::Validation {
                field: "username",
                message: "Incorrect username length (3-42 characters required)",
            });
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserDeleteRequest {
    #[schema(example = "hunter42!")]
    pub password: String,
}

impl Validatable for UserDeleteRequest {
    fn validate(&self) -> Result<(), crate::error::Error> {
        if !(8..72).contains(&self.password.len()) {
            return Err(crate::error::Error::Validation {
                field: "password",
                message: "Incorrect password length (8-72 characters required)",
            });
        }

        Ok(())
    }
}
