use actix_web::HttpRequest;
use uuid::Uuid;

use crate::{
    SESSION_COOKIE_NAME,
    error::{Error, Result},
};

pub fn parse_session_key_from_cookie(req: &HttpRequest) -> Result<Option<Uuid>> {
    if let Some(s) = req.cookie(SESSION_COOKIE_NAME) {
        let Ok(key) = Uuid::parse_str(s.value()) else {
            tracing::warn!("A session with an invalid key was received: {}", s.value());
            return Err(Error::Unauthorized);
        };

        return Ok(Some(key));
    }
    Ok(None)
}
