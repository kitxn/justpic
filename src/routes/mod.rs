use actix_web::web;

pub mod ping;

pub mod cards;
pub mod files;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(ping::ping)
            .configure(files::config),
    );
}
