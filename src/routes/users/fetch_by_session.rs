use actix_web::{HttpRequest, HttpResponse, get, web};

use crate::{
    auth::sessions::extract_user_from_cookie,
    error::{Error, Result},
    models::users::UserResponse,
};

#[utoipa::path(
    get, 
    path = "/api/users/me", 
    tag = "users", 
    responses(
        (
            status = 200,
            description = "Current authorized user",
        ),
        (
            status = 401, 
            description = "Client is not authorized"
        ),
    )
)]
#[get("/me")]
pub async fn fetch_by_session(
    req: HttpRequest,
    state: web::Data<crate::state::State>,
) -> Result<HttpResponse> {
    let user = extract_user_from_cookie(&req, state.db())
        .await?
        .ok_or(Error::Unauthorized)?;

    let res = UserResponse::from(user);
    Ok(HttpResponse::Ok().json(res))
}
