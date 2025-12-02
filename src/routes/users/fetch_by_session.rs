use actix_web::{HttpRequest, HttpResponse, get, web};

use crate::{
    auth::sessions::extract_session_from_cookie,
    database::repositories,
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
        ),
        (
            status = 404, 
            description = "User not found"
        ),
    )
)]
#[get("/me")]
pub async fn fetch_by_session(
    req: HttpRequest,
    state: web::Data<crate::state::State>,
) -> Result<HttpResponse> {
    let session = extract_session_from_cookie(&req, state.db())
        .await?
        .ok_or(Error::AccessDenied)?;

    let user = repositories::users::fetch_by_id(&session.owner_id, state.db())
        .await?
        .ok_or(Error::ItemNotFound)?;

    let res = UserResponse::from(user);
    Ok(HttpResponse::Ok().json(res))
}
