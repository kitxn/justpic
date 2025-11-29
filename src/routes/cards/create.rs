use actix_web::HttpResponse;

use crate::error::Result;

/// Create new card endpoint
pub async fn create() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}
