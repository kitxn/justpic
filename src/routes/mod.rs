use actix_web::web;

pub mod auth;
pub mod cards;
pub mod files;
pub mod users;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(files::config)
            .configure(cards::config)
            .configure(users::config)
            .configure(auth::config),
    );
}
