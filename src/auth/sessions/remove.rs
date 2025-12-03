use actix_web::cookie::Cookie;

use crate::SESSION_COOKIE_NAME;

pub fn remove_session_cookie<'a>() -> Cookie<'a> {
    crate::auth::cookie::remove(SESSION_COOKIE_NAME)
}
