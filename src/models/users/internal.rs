use crate::{
    auth::sessions::parse_session_key_from_cookie, models::users::responses::common::UserPublic,
    repositories,
};

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

    /// Extract user from HTTP request
    // TODO: Consider checking the session
    // lifetime when extracting only the user
    pub async fn from_request<'a, E>(
        req: &actix_web::HttpRequest,
        db_exec: E,
    ) -> crate::error::Result<Option<Self>>
    where
        E: sqlx::Executor<'a, Database = sqlx::sqlite::Sqlite>,
    {
        match parse_session_key_from_cookie(req)? {
            Some(key) => {
                let user = repositories::users::fetch_by_session_id(&key, db_exec).await?;

                Ok(user)
            }
            None => Ok(None),
        }
    }

    pub fn is_admin(&self) -> bool {
        self.role.is_admin()
    }

    // -- Mappers --
    pub fn to_public_model(self) -> UserPublic {
        UserPublic {
            id: self.id,
            username: self.username,
            role: self.role,
            created: self.created,
        }
    }
}
