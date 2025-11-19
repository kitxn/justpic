use actix_web::{HttpResponse, get};

use crate::error::Result;

#[utoipa::path(
    get, 
    path = "/api/ping", 
    tag = "system", 
    description = "Ping endpoint to check server operation",
    responses(
        (status = 200, description = "Server is online")
    )
)]
#[get("/ping")]
pub async fn ping() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}
