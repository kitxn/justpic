use actix_web::cookie::{Cookie, time::OffsetDateTime};

pub fn create<'a>(key: &'a str, value: String, expires: OffsetDateTime) -> Cookie<'a> {
    Cookie::build(key, value)
        .path("/")
        .http_only(true)
        .expires(expires)
        .finish()
}

pub fn remove<'a>(key: &'a str) -> Cookie<'a> {
    Cookie::build(key, "")
        .path("/")
        .expires(OffsetDateTime::now_utc())
        .finish()
}

pub fn parse_cookie(key: &str, req: &actix_web::HttpRequest) -> Option<String> {
    req.cookie(key).map(|v| v.value().to_string())
}
