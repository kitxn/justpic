use actix_web::web;

pub mod register;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").service(register::register));
}
