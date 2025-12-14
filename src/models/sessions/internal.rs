use crate::{
    SESSION_COOKIE_NAME, SESSION_LIFETIME,
    error::Error,
    repositories,
    util::cookie::{self, parse_cookie},
};

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

    /// Throw an access error if the session has expired
    pub fn throw_error_if_expired(&self) -> crate::error::Result<()> {
        if self.is_expired() {
            return Err(crate::error::Error::AccessDenied);
        }

        Ok(())
    }

    /// Extend the lifetime of a mutable session
    pub fn extend_life_time(&mut self, extend_for: u64) {
        self.expires = self.expires + chrono::Days::new(extend_for);
    }

    /// Create a cookie object based on the given session
    pub fn as_cookie<'a>(&'a self) -> actix_web::cookie::Cookie<'a> {
        use actix_web::cookie::time::{Duration, OffsetDateTime};

        let exp = OffsetDateTime::from_unix_timestamp(self.expires.timestamp())
            .unwrap_or(OffsetDateTime::now_utc() + Duration::days(SESSION_LIFETIME as i64));

        cookie::create(SESSION_COOKIE_NAME, self.id.to_string(), exp)
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

#[cfg(test)]
mod tests {
    use chrono::{Days, Utc};

    use crate::models::users::User;

    use super::*;

    fn create_session_for_test() -> Session {
        let user = User::new("john_doe", "hunter42!".to_string());

        Session::new(user.id_copy())
    }

    #[test]
    fn test_session_model_creation() {
        let user = User::new("john_doe", "hunter42!".to_string());

        let item = Session::new(user.id_copy());

        assert_eq!(
            item.owner_id(),
            user.id(),
            "The session owner ID must match the user ID"
        );

        assert!(
            !item.is_expired(),
            "The new session must not have already expired"
        );

        let time_delta = Utc::now().timestamp() - item.created.timestamp();
        assert!(
            (-1..=5).contains(&time_delta),
            "The creation time should roughly correspond to the current one"
        );
    }

    #[test]
    fn test_session_times() {
        let mut item = create_session_for_test();

        assert!(
            item.created < item.expires,
            "The session must not expire before it is created"
        );

        let expired_old = item.expires().timestamp();
        item.extend_life_time(5);

        assert!(
            expired_old < item.expires().timestamp(),
            "After life extension, its expiration time should be longer"
        );
    }

    #[test]
    fn test_session_expiration() {
        let mut item = create_session_for_test();
        item.expires = Utc::now() - Days::new(1);

        assert!(
            item.is_expired(),
            "A session that expired a day ago should be marked as expired"
        );

        assert!(
            matches!(item.throw_error_if_expired(), Err(Error::AccessDenied)),
            "If the session has expired, it should return an ACCESS_DENIED error"
        );

        assert_eq!(
            item.days_of_life_left(),
            -1,
            "For an expired session, the number of days remaining must be equal to -1"
        );
    }

    #[test]
    fn test_session_to_cookie() {
        let item = create_session_for_test();

        let cookie = item.as_cookie();

        assert_eq!(
            cookie.name(),
            SESSION_COOKIE_NAME,
            "The session cookie name must match the SESSION_COOKIE_NAME constant"
        );

        assert_eq!(
            cookie.value().to_string(),
            item.id.to_string(),
            "The session cookie value must match the session key"
        );
    }
}
