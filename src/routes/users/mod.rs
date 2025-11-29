use actix_web::web;

pub mod fetch_by_username;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users").service(fetch_by_username::fetch_by_username));
}
