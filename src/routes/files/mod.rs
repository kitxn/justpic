use actix_web::web;

pub mod get;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/files").service(get::get_file_stream));
}
