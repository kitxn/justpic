use actix_web::HttpRequest;
use uuid::Uuid;

use crate::{
    SESSION_COOKIE_NAME,
    database::{DatabasePool, repositories, schemas::sessions::DbSession},
    error::{Error, Result},
    models::users::User,
};

fn parse_session_key_from_cookie(req: &HttpRequest) -> Result<Option<Uuid>> {
    if let Some(s) = req.cookie(SESSION_COOKIE_NAME) {
        let Ok(key) = Uuid::parse_str(s.value()) else {
            tracing::warn!("A session with an invalid key was received: {}", s.value());
            return Err(Error::Unauthorized);
        };
        return Ok(Some(key));
    }
    Ok(None)
}

pub async fn extract_session_from_cookie(
    req: &HttpRequest,
    db: &DatabasePool,
) -> Result<Option<DbSession>> {
    match parse_session_key_from_cookie(req)? {
        Some(key) => {
            let session = repositories::sessions::fetch_by_id(&key, db).await?;
            Ok(session)
        }
        None => Ok(None),
    }
}

pub async fn extract_user_from_cookie(
    req: &HttpRequest,
    db: &DatabasePool,
) -> Result<Option<User>> {
    match parse_session_key_from_cookie(req)? {
        Some(key) => {
            let user = repositories::users::fetch_by_session_id(&key, db).await?;
            Ok(user)
        }
        None => Ok(None),
    }
}
