use actix_web::{HttpRequest, HttpResponse, post, web};

use crate::{
    auth::sessions::{extract_session_from_cookie, remove_session_cookie},
    repositories,
    error::{Error, Result},
};


#[utoipa::path(
    post, 
    description = "End the current session",
    path = "/auth/logout", 
    tag = "auth", 
    responses(
        (
            status = 204, 
            description = "Successful account logout"
        ),
        (
            status = 401, 
            description = "The user is not logged in yet"
        ),
    )
)]
#[post("/logout")]
pub async fn logout(
    req: HttpRequest,
    state: web::Data<crate::state::State>,
) -> Result<HttpResponse> {
    let session = extract_session_from_cookie(&req, state.db())
        .await?
        .ok_or(Error::Unauthorized)?;

    repositories::sessions::remove_by_id(session.id(), state.db()).await?;

    let cookie = remove_session_cookie();
    Ok(HttpResponse::NoContent().cookie(cookie).finish())
}

