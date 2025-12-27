use actix_web::{HttpResponse, web};

use crate::error::Result;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/files").route("/{id}", web::get().to(get_file_stream)));
}

/// Get a file by its ID
pub async fn get_file_stream(
    state: web::Data<crate::state::State>,
    id: web::Path<String>,
) -> Result<HttpResponse> {
    let storage = state.store();

    let file = storage.get(&id).await?;

    Ok(HttpResponse::Ok().streaming(file))
}
