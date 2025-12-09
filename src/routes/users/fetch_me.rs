use actix_web::{HttpRequest, HttpResponse, get, web};

use crate::{
    auth::sessions::extract_user_from_cookie,
    error::{Error, Result}, models::users::responses::common::UserPublic,
};

#[utoipa::path(
    get, 
    description = "Get information about an authenticated user",
    path = "/users/me", 
    tag = "users.me", 
    responses(
        (
            status = 200,
            body = UserPublic,
            description = "Authenticated user information",
        ),
        (
            status = 401, 
            description = "Unauthorized"
        ),
    )
)]
#[get("/me")]
pub async fn fetch_me(
    req: HttpRequest,
    state: web::Data<crate::state::State>,
) -> Result<HttpResponse> {
    let user = extract_user_from_cookie(&req, state.db())
        .await?
        .ok_or(Error::Unauthorized)?;

    Ok(HttpResponse::Ok().json(user.to_public_model()))
}
