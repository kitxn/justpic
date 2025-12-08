use actix_web::{
    HttpRequest, HttpResponse, patch,
    web::{self, Json},
};

use crate::{
    auth::sessions::extract_session_from_cookie,
    database::repositories,
    error::{Error, Result},
    models::users::change_password::UserChangePasswordRequest,
    util,
};

#[utoipa::path(
    patch,
    description = "Change password for an authenticated user",
    path = "/users/me/password", 
    tag = "users.me", 
    request_body = UserChangePasswordRequest,
    responses(
        (
            status = 204,
            description = "Password changed"
        ),
        (
            status = 401, 
            description = "Unauthorized"
        ),
    )
)]
#[patch("/me/password")]
pub async fn change_me_password(
    req: HttpRequest,
    state: web::Data<crate::state::State>,
    payload: Json<UserChangePasswordRequest>,
) -> Result<HttpResponse> {
    let session = extract_session_from_cookie(&req, state.db())
        .await?
        .ok_or(Error::Unauthorized)?;

    let hashed_password =
        tokio::task::spawn_blocking(move || util::crypto::bcrypt_hash(&payload.new_password))
            .await??;

    let query_res =
        repositories::users::change_password(&session.owner_id(), &hashed_password, state.db())
            .await?;

    if query_res.rows_affected() == 0 {
        // TODO: change error type
        return Err(Error::Unauthorized);
    }

    Ok(HttpResponse::NoContent().finish())
}
