use crate::{
    SESSION_COOKIE_NAME, SESSION_LIFETIME,
    error::Error,
    repositories,
    util::cookie::{self, parse_cookie},
};

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

    /// Extract user session from HTTP request
    pub async fn from_request<'a, E>(
        req: &actix_web::HttpRequest,
        db_exec: E,
    ) -> crate::error::Result<Option<Self>>
    where
        E: sqlx::Executor<'a, Database = sqlx::sqlite::Sqlite>,
    {
        match parse_cookie(SESSION_COOKIE_NAME, req) {
            Some(v) => {
                let Ok(session_id) = uuid::Uuid::parse_str(&v) else {
                    tracing::warn!("A session with an invalid key was received: {}", &v);
                    return Err(Error::Unauthorized);
                };

                let session = repositories::sessions::get_by_id(&session_id, db_exec).await?;
                Ok(session)
            }
            None => Ok(None),
        }
    }

    /// Check if the session has expired
    pub fn is_expired(&self) -> bool {
        self.expires < chrono::Utc::now()
    }

    pub fn as_cookie<'a>(&'a self) -> actix_web::cookie::Cookie<'a> {
        use actix_web::cookie::time::{Duration, OffsetDateTime};

        let exp = OffsetDateTime::from_unix_timestamp(self.expires.timestamp())
            .unwrap_or(OffsetDateTime::now_utc() + Duration::days(28));

        cookie::create(SESSION_COOKIE_NAME, self.id.to_string(), exp)
    }
}
