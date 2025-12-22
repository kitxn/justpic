use actix_web::web;

pub mod auth;
pub mod cards;
pub mod files;
pub mod users;

pub mod docs;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(users::config)
            .configure(auth::config)
            .configure(files::config)
            .configure(cards::config),
    );
}
