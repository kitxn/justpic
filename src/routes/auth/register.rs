use actix_web::{
    HttpResponse, post,
    web::{self, Json},
};

use crate::{error::Result, models::auth::register::{RegisterRequestData, RegisterResponseData}};

/// Register new user endpoint
#[utoipa::path(
    post, 
    path = "/api/auth/register", 
    tag = "auth", 
    request_body = RegisterRequestData,
    responses(
        (
            status = 200, 
            body = RegisterResponseData,
            description = "Successful registration"),
        (
            status = 400, 
            description = "Invalid data for registration"
        )
    )
)]
#[post("/register")]
pub async fn register(
    state: web::Data<crate::state::State>,
    payload: Json<RegisterRequestData>,
) -> Result<HttpResponse> {
    //

    Ok(HttpResponse::Ok().finish())
}
