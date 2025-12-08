use actix_web::web;

pub mod fetch_by_id;
pub mod fetch_by_username;
pub mod fetch_me;

pub mod change_me_password;
pub mod change_me_username;

pub mod delete;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(fetch_me::fetch_me)
            .service(fetch_by_username::fetch_by_username)
            .service(change_me_password::change_me_password)
            .service(change_me_username::change_me_username),
    );
}
