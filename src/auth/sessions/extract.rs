use actix_web::HttpRequest;
use uuid::Uuid;

use crate::{
    SESSION_COOKIE_NAME,
    database::{DatabasePool, repositories, schemas::sessions::DbSession},
    error::Result,
};

// TODO: Add user retrieval by session via join to avoid 2 requests at a time
pub async fn extract_session_from_cookie(
    req: &HttpRequest,
    db: &DatabasePool,
) -> Result<Option<DbSession>> {
    match req.cookie(SESSION_COOKIE_NAME) {
        Some(s) => {
            let Ok(key) = Uuid::parse_str(s.value()) else {
                return Ok(None);
            };

            let session = repositories::sessions::fetch_by_id(&key, db).await?;
            Ok(session)
        }
        None => Ok(None),
    }
}
