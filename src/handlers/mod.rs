use actix_web::web;

pub mod cards;
pub mod files;

pub mod docs;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(files::config)
            .configure(cards::config),
    );
}
