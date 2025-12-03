use actix_web::cookie::{Cookie, time::OffsetDateTime};

pub fn remove<'a>(key: &'a str) -> Cookie<'a> {
    Cookie::build(key, "")
        .path("/")
        .expires(OffsetDateTime::now_utc())
        .finish()
}
