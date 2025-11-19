use actix_web::{HttpResponse, get, web};

use crate::error::Result;

/// Get a file stream by its ID
#[utoipa::path(get, path = "/api/files/{id}", tag = "files")]
#[get("/{id}")]
pub async fn get_file_stream(
    state: web::Data<crate::state::State>,
    id: web::Path<String>,
) -> Result<HttpResponse> {
    let storage = state.store();

    let file = storage.get(&id).await?;

    Ok(HttpResponse::Ok().streaming(file))
}
