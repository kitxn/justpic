use actix_web::cookie::{
    Cookie,
    time::{Duration, OffsetDateTime},
};

use crate::{SESSION_COOKIE_NAME, database::schemas::sessions::DbSession};

pub fn create_session_cookie<'a>(session: &DbSession) -> Cookie<'a> {
    let exp = OffsetDateTime::from_unix_timestamp(session.expires.timestamp())
        .unwrap_or(OffsetDateTime::now_utc() + Duration::days(28));

    crate::auth::cookie::create(SESSION_COOKIE_NAME, session.id.to_string(), exp)
}
