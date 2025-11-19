use actix_web::{HttpResponse, get};

use crate::{error::Result, models::base::message::MessageResponse};

#[utoipa::path(
    get, 
    path = "/api/ping", 
    tag = "system", 
    description = "Ping endpoint to check server operation",
    responses(
        (status = 200, body = MessageResponse), 
        (status = 403), 
        (status = 500))
    )
]
#[get("/ping")]
pub async fn ping() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(MessageResponse::new("pong")))
}
