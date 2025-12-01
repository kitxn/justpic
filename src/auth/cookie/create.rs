use actix_web::cookie::{Cookie, time::OffsetDateTime};

pub fn create<'a>(key: &'a str, value: String, expires: OffsetDateTime) -> Cookie<'a> {
    Cookie::build(key, value)
        .path("/")
        .http_only(true)
        .expires(expires)
        .finish()
}
