use actix_web::HttpRequest;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{
    SESSION_COOKIE_NAME, SESSION_LIFETIME,
    error::Error,
    repositories,
    sessions::models::Session,
    util::cookie::{self, parse_cookie},
};

/// Gets a potentially existing session
pub async fn try_extract_from_req(
    req: &HttpRequest,
    db: &SqlitePool,
) -> Result<Option<Session>, Error> {
    let Some(session_id_string) = parse_cookie(SESSION_COOKIE_NAME, req) else {
        return Ok(None);
    };

    let Ok(session_id) = Uuid::parse_str(&session_id_string) else {
        return Ok(None);
    };

    repositories::sessions::get_by_id(&session_id, db)
        .await
        .map_err(Error::from)
}

/// Gets the session from the cookie or returns an Unauthorized error.
pub async fn extract_from_req(req: &HttpRequest, db: &SqlitePool) -> Result<Session, Error> {
    match try_extract_from_req(req, db).await? {
        Some(s) => Ok(s),
        None => Err(Error::Unauthorized),
    }
}

pub fn generate_cookie<'a>(session: &'a Session) -> actix_web::cookie::Cookie<'a> {
    use actix_web::cookie::time::{Duration, OffsetDateTime};

    let exp = OffsetDateTime::from_unix_timestamp(session.expires().timestamp())
        .unwrap_or(OffsetDateTime::now_utc() + Duration::days(SESSION_LIFETIME as i64));

    cookie::create(SESSION_COOKIE_NAME, session.id().to_string(), exp)
}

pub fn generate_empty_auth_cookie<'a>() -> actix_web::cookie::Cookie<'a> {
    cookie::empty(SESSION_COOKIE_NAME)
}
