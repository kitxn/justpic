use actix_web::web;

pub mod login;
pub mod logout;
pub mod register;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(register::register)
            .service(login::login)
            .service(logout::logout),
    );
}
